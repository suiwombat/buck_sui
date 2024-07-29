use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// UsageExportLocation : The location in Cloud Storage and naming method of the daily usage report. Contains bucket_name and report_name prefix.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UsageExportLocation {
    /// The name of an existing bucket in Cloud Storage where the usage report object is stored. The Google Service Account is granted write access to this bucket. This can either be the bucket name by itself, such as example-bucket, or the bucket name with gs:// or https://storage.googleapis.com/ in front of it, such as gs://example-bucket.
    #[serde(rename = "bucketName", skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// An optional prefix for the name of the usage report object stored in bucketName. If not supplied, defaults to usage_gce. The report is stored as a CSV file named report_name_prefix_gce_YYYYMMDD.csv where YYYYMMDD is the day of the usage according to Pacific Time. If you supply a prefix, it should conform to Cloud Storage object naming conventions.
    #[serde(rename = "reportNamePrefix", skip_serializing_if = "Option::is_none")]
    pub report_name_prefix: Option<String>,
}

impl UsageExportLocation {
    /// The location in Cloud Storage and naming method of the daily usage report. Contains bucket_name and report_name prefix.
    pub fn new() -> UsageExportLocation {
        UsageExportLocation {
            bucket_name: None,
            report_name_prefix: None,
        }
    }
}
