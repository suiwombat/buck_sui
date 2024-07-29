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
pub struct SecurityPoliciesWafConfig {
    #[serde(rename = "wafRules", skip_serializing_if = "Option::is_none")]
    pub waf_rules: Option<Box<crate::google_rest_apis::compute_v1::models::PreconfiguredWafSet>>,
}

impl SecurityPoliciesWafConfig {
    pub fn new() -> SecurityPoliciesWafConfig {
        SecurityPoliciesWafConfig { waf_rules: None }
    }
}
