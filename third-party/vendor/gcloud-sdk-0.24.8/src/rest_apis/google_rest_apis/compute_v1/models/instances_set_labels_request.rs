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
pub struct InstancesSetLabelsRequest {
    /// Fingerprint of the previous set of labels for this resource, used to prevent conflicts. Provide the latest fingerprint value when making a request to add or change labels.
    #[serde(rename = "labelFingerprint", skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl InstancesSetLabelsRequest {
    pub fn new() -> InstancesSetLabelsRequest {
        InstancesSetLabelsRequest {
            label_fingerprint: None,
            labels: None,
        }
    }
}
