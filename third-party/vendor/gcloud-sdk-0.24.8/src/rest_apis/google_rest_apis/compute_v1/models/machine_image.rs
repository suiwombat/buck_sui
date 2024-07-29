use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// MachineImage : Represents a machine image resource. A machine image is a Compute Engine resource that stores all the configuration, metadata, permissions, and data from one or more disks required to create a Virtual machine (VM) instance. For more information, see Machine images.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MachineImage {
    /// [Output Only] The creation timestamp for this machine image in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Input Only] Whether to attempt an application consistent machine image by informing the OS to prepare for the snapshot process.
    #[serde(rename = "guestFlush", skip_serializing_if = "Option::is_none")]
    pub guest_flush: Option<bool>,
    /// [Output Only] A unique identifier for this machine image. The server defines this identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceProperties", skip_serializing_if = "Option::is_none")]
    pub instance_properties:
        Option<Box<crate::google_rest_apis::compute_v1::models::InstanceProperties>>,
    /// [Output Only] The resource type, which is always compute#machineImage for machine image.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(
        rename = "machineImageEncryptionKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_image_encryption_key:
        Option<Box<crate::google_rest_apis::compute_v1::models::CustomerEncryptionKey>>,
    /// Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output only. Reserved for future use.
    #[serde(rename = "satisfiesPzi", skip_serializing_if = "Option::is_none")]
    pub satisfies_pzi: Option<bool>,
    /// [Output Only] Reserved for future use.
    #[serde(rename = "satisfiesPzs", skip_serializing_if = "Option::is_none")]
    pub satisfies_pzs: Option<bool>,
    /// An array of Machine Image specific properties for disks attached to the source instance
    #[serde(rename = "savedDisks", skip_serializing_if = "Option::is_none")]
    pub saved_disks: Option<Vec<crate::google_rest_apis::compute_v1::models::SavedDisk>>,
    /// [Output Only] The URL for this machine image. The server defines this URL.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Input Only] The customer-supplied encryption key of the disks attached to the source instance. Required if the source disk is protected by a customer-supplied encryption key.
    #[serde(
        rename = "sourceDiskEncryptionKeys",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_disk_encryption_keys:
        Option<Vec<crate::google_rest_apis::compute_v1::models::SourceDiskEncryptionKey>>,
    /// The source instance used to create the machine image. You can provide this as a partial or full URL to the resource. For example, the following are valid values: - https://www.googleapis.com/compute/v1/projects/project/zones/zone /instances/instance - projects/project/zones/zone/instances/instance
    #[serde(rename = "sourceInstance", skip_serializing_if = "Option::is_none")]
    pub source_instance: Option<String>,
    #[serde(
        rename = "sourceInstanceProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_instance_properties:
        Option<Box<crate::google_rest_apis::compute_v1::models::SourceInstanceProperties>>,
    /// [Output Only] The status of the machine image. One of the following values: INVALID, CREATING, READY, DELETING, and UPLOADING.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The regional or multi-regional Cloud Storage bucket location where the machine image is stored.
    #[serde(rename = "storageLocations", skip_serializing_if = "Option::is_none")]
    pub storage_locations: Option<Vec<String>>,
    /// [Output Only] Total size of the storage used by the machine image.
    #[serde(rename = "totalStorageBytes", skip_serializing_if = "Option::is_none")]
    pub total_storage_bytes: Option<String>,
}

impl MachineImage {
    /// Represents a machine image resource. A machine image is a Compute Engine resource that stores all the configuration, metadata, permissions, and data from one or more disks required to create a Virtual machine (VM) instance. For more information, see Machine images.
    pub fn new() -> MachineImage {
        MachineImage {
            creation_timestamp: None,
            description: None,
            guest_flush: None,
            id: None,
            instance_properties: None,
            kind: None,
            machine_image_encryption_key: None,
            name: None,
            satisfies_pzi: None,
            satisfies_pzs: None,
            saved_disks: None,
            self_link: None,
            source_disk_encryption_keys: None,
            source_instance: None,
            source_instance_properties: None,
            status: None,
            storage_locations: None,
            total_storage_bytes: None,
        }
    }
}

/// [Output Only] The status of the machine image. One of the following values: INVALID, CREATING, READY, DELETING, and UPLOADING.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "CREATING")]
    Creating,
    #[serde(rename = "DELETING")]
    Deleting,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "UPLOADING")]
    Uploading,
}

impl Default for Status {
    fn default() -> Status {
        Self::Creating
    }
}
