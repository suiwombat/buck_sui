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
pub struct SparkLoggingInfo {
    /// [Output-only] Project ID used for logging
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// [Output-only] Resource type used for logging
    #[serde(rename = "resource_type", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

impl SparkLoggingInfo {
    pub fn new() -> SparkLoggingInfo {
        SparkLoggingInfo {
            project_id: None,
            resource_type: None,
        }
    }
}
