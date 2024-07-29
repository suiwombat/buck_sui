use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// UrlRewrite : The spec for modifying the path before sending the request to the matched backend service.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UrlRewrite {
    /// Before forwarding the request to the selected service, the request's host header is replaced with contents of hostRewrite. The value must be from 1 to 255 characters.
    #[serde(rename = "hostRewrite", skip_serializing_if = "Option::is_none")]
    pub host_rewrite: Option<String>,
    /// Before forwarding the request to the selected backend service, the matching portion of the request's path is replaced by pathPrefixRewrite. The value must be from 1 to 1024 characters.
    #[serde(rename = "pathPrefixRewrite", skip_serializing_if = "Option::is_none")]
    pub path_prefix_rewrite: Option<String>,
    ///  If specified, the pattern rewrites the URL path (based on the :path header) using the HTTP template syntax. A corresponding path_template_match must be specified. Any template variables must exist in the path_template_match field. - -At least one variable must be specified in the path_template_match field - You can omit variables from the rewritten URL - The * and ** operators cannot be matched unless they have a corresponding variable name - e.g. {format=*} or {var=**}. For example, a path_template_match of /static/{format=**} could be rewritten as /static/content/{format} to prefix /content to the URL. Variables can also be re-ordered in a rewrite, so that /{country}/{format}/{suffix=**} can be rewritten as /content/{format}/{country}/{suffix}. At least one non-empty routeRules[].matchRules[].path_template_match is required. Only one of path_prefix_rewrite or path_template_rewrite may be specified.
    #[serde(
        rename = "pathTemplateRewrite",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_template_rewrite: Option<String>,
}

impl UrlRewrite {
    /// The spec for modifying the path before sending the request to the matched backend service.
    pub fn new() -> UrlRewrite {
        UrlRewrite {
            host_rewrite: None,
            path_prefix_rewrite: None,
            path_template_rewrite: None,
        }
    }
}