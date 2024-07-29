use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// NotificationEndpoint : Represents a notification endpoint. A notification endpoint resource defines an endpoint to receive notifications when there are status changes detected by the associated health check service. For more information, see Health checks overview.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationEndpoint {
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "grpcSettings", skip_serializing_if = "Option::is_none")]
    pub grpc_settings:
        Option<Box<crate::google_rest_apis::compute_v1::models::NotificationEndpointGrpcSettings>>,
    /// [Output Only] A unique identifier for this resource type. The server generates this identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of the resource. Always compute#notificationEndpoint for notification endpoints.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] URL of the region where the notification endpoint resides. This field applies only to the regional resource. You must specify this field as part of the HTTP request URL. It is not settable as a field in the request body.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

impl NotificationEndpoint {
    /// Represents a notification endpoint. A notification endpoint resource defines an endpoint to receive notifications when there are status changes detected by the associated health check service. For more information, see Health checks overview.
    pub fn new() -> NotificationEndpoint {
        NotificationEndpoint {
            creation_timestamp: None,
            description: None,
            grpc_settings: None,
            id: None,
            kind: None,
            name: None,
            region: None,
            self_link: None,
        }
    }
}
