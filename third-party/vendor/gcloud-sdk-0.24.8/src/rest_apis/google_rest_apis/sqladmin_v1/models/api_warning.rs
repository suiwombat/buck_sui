use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ApiWarning : An Admin API warning message.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiWarning {
    /// Code to uniquely identify the warning type.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    /// The warning message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The region name for REGION_UNREACHABLE warning.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl ApiWarning {
    /// An Admin API warning message.
    pub fn new() -> ApiWarning {
        ApiWarning {
            code: None,
            message: None,
            region: None,
        }
    }
}

/// Code to uniquely identify the warning type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "SQL_API_WARNING_CODE_UNSPECIFIED")]
    SqlApiWarningCodeUnspecified,
    #[serde(rename = "REGION_UNREACHABLE")]
    RegionUnreachable,
    #[serde(rename = "MAX_RESULTS_EXCEEDS_LIMIT")]
    MaxResultsExceedsLimit,
}

impl Default for Code {
    fn default() -> Code {
        Self::SqlApiWarningCodeUnspecified
    }
}
