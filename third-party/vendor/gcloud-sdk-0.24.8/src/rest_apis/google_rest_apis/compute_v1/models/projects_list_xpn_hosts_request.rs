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
pub struct ProjectsListXpnHostsRequest {
    /// Optional organization ID managed by Cloud Resource Manager, for which to list shared VPC host projects. If not specified, the organization will be inferred from the project.
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

impl ProjectsListXpnHostsRequest {
    pub fn new() -> ProjectsListXpnHostsRequest {
        ProjectsListXpnHostsRequest { organization: None }
    }
}
