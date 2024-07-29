use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PacketMirroringForwardingRuleInfo {
    /// [Output Only] Unique identifier for the forwarding rule; defined by the server.
    #[serde(rename = "canonicalUrl", skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,
    /// Resource URL to the forwarding rule representing the ILB configured as destination of the mirrored traffic.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PacketMirroringForwardingRuleInfo {
    pub fn new() -> PacketMirroringForwardingRuleInfo {
        PacketMirroringForwardingRuleInfo {
            canonical_url: None,
            url: None,
        }
    }
}
