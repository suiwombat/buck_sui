use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// GuestOsFeature : Guest OS features.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GuestOsFeature {
    /// The ID of a supported feature. To add multiple values, use commas to separate values. Set to one or more of the following values: - VIRTIO_SCSI_MULTIQUEUE - WINDOWS - MULTI_IP_SUBNET - UEFI_COMPATIBLE - GVNIC - SEV_CAPABLE - SUSPEND_RESUME_COMPATIBLE - SEV_LIVE_MIGRATABLE - SEV_SNP_CAPABLE - TDX_CAPABLE - IDPF For more information, see Enabling guest operating system features.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl GuestOsFeature {
    /// Guest OS features.
    pub fn new() -> GuestOsFeature {
        GuestOsFeature { r#type: None }
    }
}

/// The ID of a supported feature. To add multiple values, use commas to separate values. Set to one or more of the following values: - VIRTIO_SCSI_MULTIQUEUE - WINDOWS - MULTI_IP_SUBNET - UEFI_COMPATIBLE - GVNIC - SEV_CAPABLE - SUSPEND_RESUME_COMPATIBLE - SEV_LIVE_MIGRATABLE - SEV_SNP_CAPABLE - TDX_CAPABLE - IDPF For more information, see Enabling guest operating system features.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FEATURE_TYPE_UNSPECIFIED")]
    FeatureTypeUnspecified,
    #[serde(rename = "GVNIC")]
    Gvnic,
    #[serde(rename = "IDPF")]
    Idpf,
    #[serde(rename = "MULTI_IP_SUBNET")]
    MultiIpSubnet,
    #[serde(rename = "SECURE_BOOT")]
    SecureBoot,
    #[serde(rename = "SEV_CAPABLE")]
    SevCapable,
    #[serde(rename = "SEV_LIVE_MIGRATABLE")]
    SevLiveMigratable,
    #[serde(rename = "SEV_LIVE_MIGRATABLE_V2")]
    SevLiveMigratableV2,
    #[serde(rename = "SEV_SNP_CAPABLE")]
    SevSnpCapable,
    #[serde(rename = "UEFI_COMPATIBLE")]
    UefiCompatible,
    #[serde(rename = "VIRTIO_SCSI_MULTIQUEUE")]
    VirtioScsiMultiqueue,
    #[serde(rename = "WINDOWS")]
    Windows,
    #[serde(rename = "SUSPEND_RESUME_COMPATIBLE")]
    SuspendResumeCompatible,
    #[serde(rename = "TDX_CAPABLE")]
    TdxCapable,
}

impl Default for Type {
    fn default() -> Type {
        Self::FeatureTypeUnspecified
    }
}
