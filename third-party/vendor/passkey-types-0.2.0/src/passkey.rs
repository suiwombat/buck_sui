use std::fmt::Debug;

use super::u2f::{AuthenticationRequest, RegisterRequest, RegisterResponse};
use crate::{ctap2::make_credential as ctap2, webauthn, Bytes};
use coset::CoseKey;

/// The private WebAuthn credential containing all relevant required and optional information for an
/// authentication ceremony.
///
/// The WebAuthn term for this is a [Public Key Credential Source][cred-src].
///
/// # Personally Identifying Information (PII) considerations
/// While this struct implements [`Debug`], it only prints the following fields:
/// * [`CoseKey::kty`] enum from the [`Self::key`] field,
/// * [`Self::counter`] which is the number of times this was used to authenticate.
///
/// The rest of this struct should be considered secret, either for cryptographic security, or because
/// its value could be used as PII.
///
/// [cred-src]: https://w3c.github.io/webauthn/#public-key-credential-source
// TODO: Implement Zeroize on this if/when rolling our own CoseKey type
// TODO: use `#[non_exhaustive]` here with a builder pattern for building new passkeys
#[derive(Clone)]
pub struct Passkey {
    /// The private key in COSE key format.
    ///
    /// # PII considerations
    /// This value should be considered secret and never printed out as it is a secret cryptographic
    /// key. The only thing that get printed in the `Debug` implementation is the key type,
    /// e.g: EC2, RSA, etc.
    pub key: CoseKey,

    /// A probabilistically-unique byte sequence identifying this [`Passkey`]. It must be at most 1023
    /// bytes long.
    ///
    /// Credential IDs are generated by authenticators in two forms:
    /// 1. At least 16 bytes that include at least 100 bits of entropy, or
    /// 2. The [`Passkey`] item, without its `credential_id`, encrypted so only its managing
    /// authenticator can decrypt it. This form allows the authenticator to be nearly stateless, by
    /// having the Relying Party store any necessary state.
    ///
    /// Relying Parties do not need to distinguish these two `credential id` forms.
    ///
    ///
    /// # PII considerations
    /// This value should be considered secret as it is the user's credential ID for the associated
    /// Relying Party. See [Privacy leak via credential IDs][privacy] for more information.
    ///
    /// [privacy]: https://w3c.github.io/webauthn/#sctn-credential-id-privacy-leak
    pub credential_id: Bytes,

    /// The [Relying Party ID][RP_ID] for which the [`Passkey`] is associated. This value mirrors the
    /// [`webauthn::PublicKeyCredentialRpEntity::id`] value passed during the creation of this credential.
    ///
    /// # PII considerations
    /// This should be handled similarly to a URL. Since this is a user credential for a Relying
    /// Party, this would leak the fact that a user has an account for this particular Relying Party.
    ///
    /// [RP_ID]: https://w3c.github.io/webauthn/#relying-party-identifier
    pub rp_id: String,

    /// This is the [`webauthn::PublicKeyCredentialUserEntity::id`] passed in during the creation of
    /// this credential. An Authenticator can choose to store this or not. If it stores this value,
    /// this [`Passkey`] will become a [Discoverable Credential] and will be returned during authentication
    /// Ceremonies.
    ///
    /// # PII Considerations
    /// This is the identifier a Relying party uses on their side to personally identify a user. This
    /// value is analogous to a username.
    ///
    /// [Discoverable Credential]: https://w3c.github.io/webauthn/#client-side-discoverable-credential
    pub user_handle: Option<Bytes>,

    /// Value tracks the number of times an authentication ceremony has been successfully completed.
    /// If the value is `None` then it will be sent as the constant `0`.
    /// See [Signature counter considerations][signCount] for more information.
    ///
    /// # PII considerations
    /// This value, if populated, is used by the Relying Party as an indicator of a cloned
    /// authenticator. If this [`Passkey`] is to be synced, consider leaving this value empty unless
    /// you can guarantee the value to always be increased for every use of this passkey across its
    /// distribution.
    ///
    /// [signCount]: https://w3c.github.io/webauthn/#signature-counter
    pub counter: Option<u32>,
}

impl Passkey {
    /// Standardised way to "upgrade" a U2F register request into a passkey
    pub fn from_u2f_register_response(
        request: &RegisterRequest,
        response: &RegisterResponse,
        private_key: &CoseKey,
    ) -> Self {
        let app_id: Bytes = request.application.to_vec().into();
        Self {
            key: private_key.clone(),
            credential_id: response.key_handle.clone().to_vec().into(),
            rp_id: app_id.into(),
            user_handle: None,
            counter: Some(0),
        }
    }

    /// Updgrade a U2F Authentication Request into a Passkey
    pub fn from_u2f_auth_request(
        request: &AuthenticationRequest,
        counter: u32,
        private_key: &CoseKey,
    ) -> Self {
        let app_id: Bytes = request.application.to_vec().into();
        Self {
            key: private_key.clone(),
            credential_id: request.key_handle.clone().to_vec().into(),
            rp_id: app_id.into(),
            user_handle: None,
            counter: Some(counter),
        }
    }

    /// This function wraps up a U2F registration request as a Passkey for storing
    /// in a CredentialStore.
    pub fn wrap_u2f_registration_request(
        request: &RegisterRequest,
        response: &RegisterResponse,
        key_handle: &[u8],
        private_key: &CoseKey,
    ) -> (
        Passkey,
        ctap2::PublicKeyCredentialUserEntity,
        ctap2::PublicKeyCredentialRpEntity,
    ) {
        let passkey = Passkey::from_u2f_register_response(request, response, private_key);

        let user_entity = ctap2::PublicKeyCredentialUserEntity {
            id: key_handle.to_vec().into(),
            display_name: None,
            name: None,
            icon_url: None,
        };

        let app_id: Bytes = request.application.to_vec().into();
        let rp = ctap2::PublicKeyCredentialRpEntity {
            id: app_id.into(),
            name: None,
        };

        (passkey, user_entity, rp)
    }
}

impl From<Passkey> for webauthn::PublicKeyCredentialDescriptor {
    fn from(value: Passkey) -> Self {
        Self {
            ty: webauthn::PublicKeyCredentialType::PublicKey,
            id: value.credential_id,
            transports: None,
        }
    }
}

impl From<&Passkey> for webauthn::PublicKeyCredentialDescriptor {
    fn from(value: &Passkey) -> Self {
        Self {
            ty: webauthn::PublicKeyCredentialType::PublicKey,
            id: value.credential_id.clone(),
            transports: None,
        }
    }
}

impl Debug for Passkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Passkey")
            .field("key_type", &self.key.kty)
            .field("counter", &self.counter)
            .finish()
    }
}
