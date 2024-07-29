use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RrSetRoutingPolicyLoadBalancerTarget : The configuration for an individual load balancer to health check.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RrSetRoutingPolicyLoadBalancerTarget {
    /// The frontend IP address of the load balancer to health check.
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The protocol of the load balancer to health check.
    #[serde(rename = "ipProtocol", skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<IpProtocol>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The type of load balancer specified by this target. This value must match the configuration of the load balancer located at the LoadBalancerTarget's IP address, port, and region. Use the following: - *regionalL4ilb*: for a regional internal passthrough Network Load Balancer. - *regionalL7ilb*: for a regional internal Application Load Balancer. - *globalL7ilb*: for a global internal Application Load Balancer.
    #[serde(rename = "loadBalancerType", skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<LoadBalancerType>,
    /// The fully qualified URL of the network that the load balancer is attached to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network} .
    #[serde(rename = "networkUrl", skip_serializing_if = "Option::is_none")]
    pub network_url: Option<String>,
    /// The configured port of the load balancer.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// The project ID in which the load balancer is located.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The region in which the load balancer is located.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl RrSetRoutingPolicyLoadBalancerTarget {
    /// The configuration for an individual load balancer to health check.
    pub fn new() -> RrSetRoutingPolicyLoadBalancerTarget {
        RrSetRoutingPolicyLoadBalancerTarget {
            ip_address: None,
            ip_protocol: None,
            kind: None,
            load_balancer_type: None,
            network_url: None,
            port: None,
            project: None,
            region: None,
        }
    }
}

/// The protocol of the load balancer to health check.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IpProtocol {
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

impl Default for IpProtocol {
    fn default() -> IpProtocol {
        Self::Undefined
    }
}
/// The type of load balancer specified by this target. This value must match the configuration of the load balancer located at the LoadBalancerTarget's IP address, port, and region. Use the following: - *regionalL4ilb*: for a regional internal passthrough Network Load Balancer. - *regionalL7ilb*: for a regional internal Application Load Balancer. - *globalL7ilb*: for a global internal Application Load Balancer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoadBalancerType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "globalL7ilb")]
    GlobalL7ilb,
    #[serde(rename = "regionalL4ilb")]
    RegionalL4ilb,
    #[serde(rename = "regionalL7ilb")]
    RegionalL7ilb,
}

impl Default for LoadBalancerType {
    fn default() -> LoadBalancerType {
        Self::None
    }
}
