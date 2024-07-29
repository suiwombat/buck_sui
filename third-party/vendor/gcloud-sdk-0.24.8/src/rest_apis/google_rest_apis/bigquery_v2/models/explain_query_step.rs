use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExplainQueryStep {
    /// Machine-readable operation type.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Human-readable stage descriptions.
    #[serde(rename = "substeps", skip_serializing_if = "Option::is_none")]
    pub substeps: Option<Vec<String>>,
}

impl ExplainQueryStep {
    pub fn new() -> ExplainQueryStep {
        ExplainQueryStep {
            kind: None,
            substeps: None,
        }
    }
}
