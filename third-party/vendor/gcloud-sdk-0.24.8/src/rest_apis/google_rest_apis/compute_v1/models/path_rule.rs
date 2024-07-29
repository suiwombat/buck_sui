use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// PathRule : A path-matching rule for a URL. If matched, will use the specified BackendService to handle the traffic arriving at this URL.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PathRule {
    /// The list of path patterns to match. Each must start with / and the only place a * is allowed is at the end following a /. The string fed to the path matcher does not include any text after the first ? or #, and those chars are not allowed here.
    #[serde(rename = "paths", skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    #[serde(rename = "routeAction", skip_serializing_if = "Option::is_none")]
    pub route_action: Option<Box<crate::google_rest_apis::compute_v1::models::HttpRouteAction>>,
    /// The full or partial URL of the backend service resource to which traffic is directed if this rule is matched. If routeAction is also specified, advanced routing actions, such as URL rewrites, take effect before sending the request to the backend. However, if service is specified, routeAction cannot contain any weightedBackendServices. Conversely, if routeAction specifies any weightedBackendServices, service must not be specified. Only one of urlRedirect, service or routeAction.weightedBackendService must be set.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "urlRedirect", skip_serializing_if = "Option::is_none")]
    pub url_redirect: Option<Box<crate::google_rest_apis::compute_v1::models::HttpRedirectAction>>,
}

impl PathRule {
    /// A path-matching rule for a URL. If matched, will use the specified BackendService to handle the traffic arriving at this URL.
    pub fn new() -> PathRule {
        PathRule {
            paths: None,
            route_action: None,
            service: None,
            url_redirect: None,
        }
    }
}
