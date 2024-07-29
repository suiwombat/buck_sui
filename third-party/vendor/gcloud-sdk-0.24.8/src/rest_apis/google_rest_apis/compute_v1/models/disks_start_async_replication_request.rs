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
pub struct DisksStartAsyncReplicationRequest {
    /// The secondary disk to start asynchronous replication to. You can provide this as a partial or full URL to the resource. For example, the following are valid values: - https://www.googleapis.com/compute/v1/projects/project/zones/zone /disks/disk - https://www.googleapis.com/compute/v1/projects/project/regions/region /disks/disk - projects/project/zones/zone/disks/disk - projects/project/regions/region/disks/disk - zones/zone/disks/disk - regions/region/disks/disk
    #[serde(rename = "asyncSecondaryDisk", skip_serializing_if = "Option::is_none")]
    pub async_secondary_disk: Option<String>,
}

impl DisksStartAsyncReplicationRequest {
    pub fn new() -> DisksStartAsyncReplicationRequest {
        DisksStartAsyncReplicationRequest {
            async_secondary_disk: None,
        }
    }
}
