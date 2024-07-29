use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Buckets : A list of buckets.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Buckets {
    /// The list of items.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::google_rest_apis::storage_v1::models::Bucket>>,
    /// The kind of item this is. For lists of buckets, this is always storage#buckets.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl Buckets {
    /// A list of buckets.
    pub fn new() -> Buckets {
        Buckets {
            items: None,
            kind: None,
            next_page_token: None,
        }
    }
}
