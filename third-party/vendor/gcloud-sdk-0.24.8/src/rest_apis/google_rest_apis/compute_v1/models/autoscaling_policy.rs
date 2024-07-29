use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// AutoscalingPolicy : Cloud Autoscaler policy.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutoscalingPolicy {
    /// The number of seconds that your application takes to initialize on a VM instance. This is referred to as the [initialization period](/compute/docs/autoscaler#cool_down_period). Specifying an accurate initialization period improves autoscaler decisions. For example, when scaling out, the autoscaler ignores data from VMs that are still initializing because those VMs might not yet represent normal usage of your application. The default initialization period is 60 seconds. Initialization periods might vary because of numerous factors. We recommend that you test how long your application takes to initialize. To do this, create a VM and time your application's startup process.
    #[serde(rename = "coolDownPeriodSec", skip_serializing_if = "Option::is_none")]
    pub cool_down_period_sec: Option<i32>,
    #[serde(rename = "cpuUtilization", skip_serializing_if = "Option::is_none")]
    pub cpu_utilization:
        Option<Box<crate::google_rest_apis::compute_v1::models::AutoscalingPolicyCpuUtilization>>,
    /// Configuration parameters of autoscaling based on a custom metric.
    #[serde(
        rename = "customMetricUtilizations",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_metric_utilizations: Option<
        Vec<crate::google_rest_apis::compute_v1::models::AutoscalingPolicyCustomMetricUtilization>,
    >,
    #[serde(
        rename = "loadBalancingUtilization",
        skip_serializing_if = "Option::is_none"
    )]
    pub load_balancing_utilization: Option<
        Box<crate::google_rest_apis::compute_v1::models::AutoscalingPolicyLoadBalancingUtilization>,
    >,
    /// The maximum number of instances that the autoscaler can scale out to. This is required when creating or updating an autoscaler. The maximum number of replicas must not be lower than minimal number of replicas.
    #[serde(rename = "maxNumReplicas", skip_serializing_if = "Option::is_none")]
    pub max_num_replicas: Option<i32>,
    /// The minimum number of replicas that the autoscaler can scale in to. This cannot be less than 0. If not provided, autoscaler chooses a default value depending on maximum number of instances allowed.
    #[serde(rename = "minNumReplicas", skip_serializing_if = "Option::is_none")]
    pub min_num_replicas: Option<i32>,
    /// Defines the operating mode for this policy. The following modes are available: - OFF: Disables the autoscaler but maintains its configuration. - ONLY_SCALE_OUT: Restricts the autoscaler to add VM instances only. - ON: Enables all autoscaler activities according to its policy. For more information, see \"Turning off or restricting an autoscaler\"
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "scaleInControl", skip_serializing_if = "Option::is_none")]
    pub scale_in_control:
        Option<Box<crate::google_rest_apis::compute_v1::models::AutoscalingPolicyScaleInControl>>,
    /// Scaling schedules defined for an autoscaler. Multiple schedules can be set on an autoscaler, and they can overlap. During overlapping periods the greatest min_required_replicas of all scaling schedules is applied. Up to 128 scaling schedules are allowed.
    #[serde(rename = "scalingSchedules", skip_serializing_if = "Option::is_none")]
    pub scaling_schedules: Option<
        ::std::collections::HashMap<
            String,
            crate::google_rest_apis::compute_v1::models::AutoscalingPolicyScalingSchedule,
        >,
    >,
}

impl AutoscalingPolicy {
    /// Cloud Autoscaler policy.
    pub fn new() -> AutoscalingPolicy {
        AutoscalingPolicy {
            cool_down_period_sec: None,
            cpu_utilization: None,
            custom_metric_utilizations: None,
            load_balancing_utilization: None,
            max_num_replicas: None,
            min_num_replicas: None,
            mode: None,
            scale_in_control: None,
            scaling_schedules: None,
        }
    }
}

/// Defines the operating mode for this policy. The following modes are available: - OFF: Disables the autoscaler but maintains its configuration. - ONLY_SCALE_OUT: Restricts the autoscaler to add VM instances only. - ON: Enables all autoscaler activities according to its policy. For more information, see \"Turning off or restricting an autoscaler\"
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "false")]
    False,
    #[serde(rename = "true")]
    True,
    #[serde(rename = "ONLY_SCALE_OUT")]
    OnlyScaleOut,
    #[serde(rename = "ONLY_UP")]
    OnlyUp,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::False
    }
}