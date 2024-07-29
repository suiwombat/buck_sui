use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RrSetRoutingPolicyGeoPolicy : Configures a RRSetRoutingPolicy that routes based on the geo location of the querying user.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RrSetRoutingPolicyGeoPolicy {
    /// Without fencing, if health check fails for all configured items in the current geo bucket, we failover to the next nearest geo bucket. With fencing, if health checking is enabled, as long as some targets in the current geo bucket are healthy, we return only the healthy targets. However, if all targets are unhealthy, we don't failover to the next nearest bucket; instead, we return all the items in the current bucket even when all targets are unhealthy.
    #[serde(rename = "enableFencing", skip_serializing_if = "Option::is_none")]
    pub enable_fencing: Option<bool>,
    /// The primary geo routing configuration. If there are multiple items with the same location, an error is returned instead.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<
        Vec<crate::google_rest_apis::dns_v1::models::RrSetRoutingPolicyGeoPolicyGeoPolicyItem>,
    >,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl RrSetRoutingPolicyGeoPolicy {
    /// Configures a RRSetRoutingPolicy that routes based on the geo location of the querying user.
    pub fn new() -> RrSetRoutingPolicyGeoPolicy {
        RrSetRoutingPolicyGeoPolicy {
            enable_fencing: None,
            items: None,
            kind: None,
        }
    }
}
