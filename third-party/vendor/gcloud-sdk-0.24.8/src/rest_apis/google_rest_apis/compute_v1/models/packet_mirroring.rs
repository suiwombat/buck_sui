use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// PacketMirroring : Represents a Packet Mirroring resource. Packet Mirroring clones the traffic of specified instances in your Virtual Private Cloud (VPC) network and forwards it to a collector destination, such as an instance group of an internal TCP/UDP load balancer, for analysis or examination. For more information about setting up Packet Mirroring, see Using Packet Mirroring.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PacketMirroring {
    #[serde(rename = "collectorIlb", skip_serializing_if = "Option::is_none")]
    pub collector_ilb:
        Option<Box<crate::google_rest_apis::compute_v1::models::PacketMirroringForwardingRuleInfo>>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates whether or not this packet mirroring takes effect. If set to FALSE, this packet mirroring policy will not be enforced on the network. The default is TRUE.
    #[serde(rename = "enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<Enable>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::google_rest_apis::compute_v1::models::PacketMirroringFilter>>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of the resource. Always compute#packetMirroring for packet mirrorings.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "mirroredResources", skip_serializing_if = "Option::is_none")]
    pub mirrored_resources: Option<
        Box<crate::google_rest_apis::compute_v1::models::PacketMirroringMirroredResourceInfo>,
    >,
    /// Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network:
        Option<Box<crate::google_rest_apis::compute_v1::models::PacketMirroringNetworkInfo>>,
    /// The priority of applying this configuration. Priority is used to break ties in cases where there is more than one matching rule. In the case of two rules that apply for a given Instance, the one with the lowest-numbered priority value wins. Default value is 1000. Valid range is 0 through 65535.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// [Output Only] URI of the region where the packetMirroring resides.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

impl PacketMirroring {
    /// Represents a Packet Mirroring resource. Packet Mirroring clones the traffic of specified instances in your Virtual Private Cloud (VPC) network and forwards it to a collector destination, such as an instance group of an internal TCP/UDP load balancer, for analysis or examination. For more information about setting up Packet Mirroring, see Using Packet Mirroring.
    pub fn new() -> PacketMirroring {
        PacketMirroring {
            collector_ilb: None,
            creation_timestamp: None,
            description: None,
            enable: None,
            filter: None,
            id: None,
            kind: None,
            mirrored_resources: None,
            name: None,
            network: None,
            priority: None,
            region: None,
            self_link: None,
        }
    }
}

/// Indicates whether or not this packet mirroring takes effect. If set to FALSE, this packet mirroring policy will not be enforced on the network. The default is TRUE.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Enable {
    #[serde(rename = "FALSE")]
    False,
    #[serde(rename = "TRUE")]
    True,
}

impl Default for Enable {
    fn default() -> Enable {
        Self::False
    }
}
