use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BackendServiceLocalityLoadBalancingPolicyConfigPolicy : The configuration for a built-in load balancing policy.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BackendServiceLocalityLoadBalancingPolicyConfigPolicy {
    /// The name of a locality load-balancing policy. Valid values include ROUND_ROBIN and, for Java clients, LEAST_REQUEST. For information about these values, see the description of localityLbPolicy. Do not specify the same policy more than once for a backend. If you do, the configuration is rejected.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
}

impl BackendServiceLocalityLoadBalancingPolicyConfigPolicy {
    /// The configuration for a built-in load balancing policy.
    pub fn new() -> BackendServiceLocalityLoadBalancingPolicyConfigPolicy {
        BackendServiceLocalityLoadBalancingPolicyConfigPolicy { name: None }
    }
}

/// The name of a locality load-balancing policy. Valid values include ROUND_ROBIN and, for Java clients, LEAST_REQUEST. For information about these values, see the description of localityLbPolicy. Do not specify the same policy more than once for a backend. If you do, the configuration is rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "INVALID_LB_POLICY")]
    InvalidLbPolicy,
    #[serde(rename = "LEAST_REQUEST")]
    LeastRequest,
    #[serde(rename = "MAGLEV")]
    Maglev,
    #[serde(rename = "ORIGINAL_DESTINATION")]
    OriginalDestination,
    #[serde(rename = "RANDOM")]
    Random,
    #[serde(rename = "RING_HASH")]
    RingHash,
    #[serde(rename = "ROUND_ROBIN")]
    RoundRobin,
    #[serde(rename = "WEIGHTED_MAGLEV")]
    WeightedMaglev,
}

impl Default for Name {
    fn default() -> Name {
        Self::InvalidLbPolicy
    }
}
