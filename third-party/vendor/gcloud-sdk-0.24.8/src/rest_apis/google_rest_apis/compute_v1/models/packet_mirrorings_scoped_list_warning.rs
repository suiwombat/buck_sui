use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// PacketMirroringsScopedListWarning : Informational warning which replaces the list of packetMirrorings when the list is empty.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PacketMirroringsScopedListWarning {
    /// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    /// [Output Only] Metadata about this warning in key: value format. For example: \"data\": [ { \"key\": \"scope\", \"value\": \"zones/us-east1-d\" } 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::google_rest_apis::compute_v1::models::AcceleratorTypeAggregatedListWarningDataInner>>,
    /// [Output Only] A human-readable description of the warning code.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl PacketMirroringsScopedListWarning {
    /// Informational warning which replaces the list of packetMirrorings when the list is empty.
    pub fn new() -> PacketMirroringsScopedListWarning {
        PacketMirroringsScopedListWarning {
            code: None,
            data: None,
            message: None,
        }
    }
}

/// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "CLEANUP_FAILED")]
    CleanupFailed,
    #[serde(rename = "DEPRECATED_RESOURCE_USED")]
    DeprecatedResourceUsed,
    #[serde(rename = "DEPRECATED_TYPE_USED")]
    DeprecatedTypeUsed,
    #[serde(rename = "DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
    DiskSizeLargerThanImageSize,
    #[serde(rename = "EXPERIMENTAL_TYPE_USED")]
    ExperimentalTypeUsed,
    #[serde(rename = "EXTERNAL_API_WARNING")]
    ExternalApiWarning,
    #[serde(rename = "FIELD_VALUE_OVERRIDEN")]
    FieldValueOverriden,
    #[serde(rename = "INJECTED_KERNELS_DEPRECATED")]
    InjectedKernelsDeprecated,
    #[serde(rename = "INVALID_HEALTH_CHECK_FOR_DYNAMIC_WIEGHTED_LB")]
    InvalidHealthCheckForDynamicWieghtedLb,
    #[serde(rename = "LARGE_DEPLOYMENT_WARNING")]
    LargeDeploymentWarning,
    #[serde(rename = "LIST_OVERHEAD_QUOTA_EXCEED")]
    ListOverheadQuotaExceed,
    #[serde(rename = "MISSING_TYPE_DEPENDENCY")]
    MissingTypeDependency,
    #[serde(rename = "NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
    NextHopAddressNotAssigned,
    #[serde(rename = "NEXT_HOP_CANNOT_IP_FORWARD")]
    NextHopCannotIpForward,
    #[serde(rename = "NEXT_HOP_INSTANCE_HAS_NO_IPV6_INTERFACE")]
    NextHopInstanceHasNoIpv6Interface,
    #[serde(rename = "NEXT_HOP_INSTANCE_NOT_FOUND")]
    NextHopInstanceNotFound,
    #[serde(rename = "NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
    NextHopInstanceNotOnNetwork,
    #[serde(rename = "NEXT_HOP_NOT_RUNNING")]
    NextHopNotRunning,
    #[serde(rename = "NOT_CRITICAL_ERROR")]
    NotCriticalError,
    #[serde(rename = "NO_RESULTS_ON_PAGE")]
    NoResultsOnPage,
    #[serde(rename = "PARTIAL_SUCCESS")]
    PartialSuccess,
    #[serde(rename = "REQUIRED_TOS_AGREEMENT")]
    RequiredTosAgreement,
    #[serde(rename = "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
    ResourceInUseByOtherResourceWarning,
    #[serde(rename = "RESOURCE_NOT_DELETED")]
    ResourceNotDeleted,
    #[serde(rename = "SCHEMA_VALIDATION_IGNORED")]
    SchemaValidationIgnored,
    #[serde(rename = "SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
    SingleInstancePropertyTemplate,
    #[serde(rename = "UNDECLARED_PROPERTIES")]
    UndeclaredProperties,
    #[serde(rename = "UNREACHABLE")]
    Unreachable,
}

impl Default for Code {
    fn default() -> Code {
        Self::CleanupFailed
    }
}
