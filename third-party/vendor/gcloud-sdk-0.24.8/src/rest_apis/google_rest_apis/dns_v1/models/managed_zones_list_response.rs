use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ManagedZonesListResponse {
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<crate::google_rest_apis::dns_v1::models::ResponseHeader>>,
    /// Type of resource.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The managed zone resources.
    #[serde(rename = "managedZones", skip_serializing_if = "Option::is_none")]
    pub managed_zones: Option<Vec<crate::google_rest_apis::dns_v1::models::ManagedZone>>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. This lets you the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ManagedZonesListResponse {
    pub fn new() -> ManagedZonesListResponse {
        ManagedZonesListResponse {
            header: None,
            kind: None,
            managed_zones: None,
            next_page_token: None,
        }
    }
}
