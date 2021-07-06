#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvancedThreatProtectionProperties {
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvancedThreatProtectionSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdvancedThreatProtectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoProvisioningSettingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AutoProvisioningSetting>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoProvisioningSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoProvisioningSettingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoProvisioningSettingProperties {
    #[serde(rename = "autoProvision")]
    pub auto_provision: auto_provisioning_setting_properties::AutoProvision,
}
pub mod auto_provisioning_setting_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AutoProvision {
        On,
        Off,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplianceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Compliance>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compliance {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ComplianceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplianceProperties {
    #[serde(rename = "assessmentTimestampUtcDate", default, skip_serializing_if = "Option::is_none")]
    pub assessment_timestamp_utc_date: Option<String>,
    #[serde(rename = "resourceCount", default, skip_serializing_if = "Option::is_none")]
    pub resource_count: Option<i64>,
    #[serde(rename = "assessmentResult", default, skip_serializing_if = "Vec::is_empty")]
    pub assessment_result: Vec<ComplianceSegment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplianceSegment {
    #[serde(rename = "segmentType", default, skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceSecurityGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeviceSecurityGroup>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceSecurityGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeviceSecurityGroupProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceSecurityGroupProperties {
    #[serde(rename = "thresholdRules", default, skip_serializing_if = "Vec::is_empty")]
    pub threshold_rules: Vec<ThresholdCustomAlertRule>,
    #[serde(rename = "timeWindowRules", default, skip_serializing_if = "Vec::is_empty")]
    pub time_window_rules: Vec<TimeWindowCustomAlertRule>,
    #[serde(rename = "allowlistRules", default, skip_serializing_if = "Vec::is_empty")]
    pub allowlist_rules: Vec<AllowlistCustomAlertRule>,
    #[serde(rename = "denylistRules", default, skip_serializing_if = "Vec::is_empty")]
    pub denylist_rules: Vec<DenylistCustomAlertRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomAlertRule {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "ruleType")]
    pub rule_type: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListCustomAlertRule {
    #[serde(flatten)]
    pub custom_alert_rule: CustomAlertRule,
    #[serde(rename = "valueType", default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<list_custom_alert_rule::ValueType>,
}
pub mod list_custom_alert_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValueType {
        IpCidr,
        String,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllowlistCustomAlertRule {
    #[serde(flatten)]
    pub list_custom_alert_rule: ListCustomAlertRule,
    #[serde(rename = "allowlistValues")]
    pub allowlist_values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionToIpNotAllowed {
    #[serde(flatten)]
    pub allowlist_custom_alert_rule: AllowlistCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalUserNotAllowed {
    #[serde(flatten)]
    pub allowlist_custom_alert_rule: AllowlistCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessNotAllowed {
    #[serde(flatten)]
    pub allowlist_custom_alert_rule: AllowlistCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenylistCustomAlertRule {
    #[serde(flatten)]
    pub list_custom_alert_rule: ListCustomAlertRule,
    #[serde(rename = "denylistValues")]
    pub denylist_values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThresholdCustomAlertRule {
    #[serde(flatten)]
    pub custom_alert_rule: CustomAlertRule,
    #[serde(rename = "minThreshold")]
    pub min_threshold: i64,
    #[serde(rename = "maxThreshold")]
    pub max_threshold: i64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeWindowCustomAlertRule {
    #[serde(flatten)]
    pub threshold_custom_alert_rule: ThresholdCustomAlertRule,
    #[serde(rename = "timeWindowSize")]
    pub time_window_size: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveConnectionsNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmqpC2dMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttC2dMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpC2dMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmqpC2dRejectedMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttC2dRejectedMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpC2dRejectedMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmqpD2cMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttD2cMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpD2cMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectMethodInvokesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedLocalLoginsNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileUploadsNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueuePurgesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwinUpdatesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnauthorizedOperationsNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformationProtectionPolicyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InformationProtectionPolicy>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformationProtectionPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InformationProtectionPolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformationProtectionPolicyProperties {
    #[serde(rename = "lastModifiedUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
    #[serde(rename = "informationTypes", default, skip_serializing_if = "Option::is_none")]
    pub information_types: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitivityLabel {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<sensitivity_label::Rank>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
pub mod sensitivity_label {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Rank {
        None,
        Low,
        Medium,
        High,
        Critical,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformationType {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "recommendedLabelId", default, skip_serializing_if = "Option::is_none")]
    pub recommended_label_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<InformationProtectionKeyword>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformationProtectionKeyword {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    #[serde(rename = "canBeNumeric", default, skip_serializing_if = "Option::is_none")]
    pub can_be_numeric: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSeverityMetrics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionAnalyticsModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecuritySolutionAnalyticsModelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionAnalyticsModelProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<IoTSeverityMetrics>,
    #[serde(rename = "unhealthyDeviceCount", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_device_count: Option<i64>,
    #[serde(rename = "devicesMetrics", default, skip_serializing_if = "Vec::is_empty")]
    pub devices_metrics: Vec<serde_json::Value>,
    #[serde(rename = "topAlertedDevices", default, skip_serializing_if = "Option::is_none")]
    pub top_alerted_devices: Option<IoTSecurityAlertedDevicesList>,
    #[serde(rename = "mostPrevalentDeviceAlerts", default, skip_serializing_if = "Option::is_none")]
    pub most_prevalent_device_alerts: Option<IoTSecurityDeviceAlertsList>,
    #[serde(rename = "mostPrevalentDeviceRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub most_prevalent_device_recommendations: Option<IoTSecurityDeviceRecommendationsList>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionAnalyticsModelList {
    pub value: Vec<IoTSecuritySolutionAnalyticsModel>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedAlertList {
    pub value: Vec<IoTSecurityAggregatedAlert>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedRecommendationList {
    pub value: Vec<IoTSecurityAggregatedRecommendation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAlertedDevicesList {
    pub value: Vec<IoTSecurityAlertedDevice>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityDeviceAlertsList {
    pub value: Vec<IoTSecurityDeviceAlert>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityDeviceRecommendationsList {
    pub value: Vec<IoTSecurityDeviceRecommendation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedAlert {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecurityAggregatedAlertProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedAlertProperties {
    #[serde(rename = "alertType", default, skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[serde(rename = "aggregatedDateUtc", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_date_utc: Option<String>,
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_aggregated_alert_properties::ReportedSeverity>,
    #[serde(rename = "remediationSteps", default, skip_serializing_if = "Option::is_none")]
    pub remediation_steps: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "effectedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub effected_resource_type: Option<String>,
    #[serde(rename = "systemSource", default, skip_serializing_if = "Option::is_none")]
    pub system_source: Option<String>,
    #[serde(rename = "actionTaken", default, skip_serializing_if = "Option::is_none")]
    pub action_taken: Option<String>,
    #[serde(rename = "logAnalyticsQuery", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_query: Option<String>,
}
pub mod io_t_security_aggregated_alert_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReportedSeverity {
        Informational,
        Low,
        Medium,
        High,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedRecommendation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecurityAggregatedRecommendationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedRecommendationProperties {
    #[serde(rename = "recommendationName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_name: Option<String>,
    #[serde(rename = "recommendationDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "recommendationTypeId", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_type_id: Option<String>,
    #[serde(rename = "detectedBy", default, skip_serializing_if = "Option::is_none")]
    pub detected_by: Option<String>,
    #[serde(rename = "remediationSteps", default, skip_serializing_if = "Option::is_none")]
    pub remediation_steps: Option<String>,
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_aggregated_recommendation_properties::ReportedSeverity>,
    #[serde(rename = "healthyDevices", default, skip_serializing_if = "Option::is_none")]
    pub healthy_devices: Option<i64>,
    #[serde(rename = "unhealthyDeviceCount", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_device_count: Option<i64>,
    #[serde(rename = "logAnalyticsQuery", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_query: Option<String>,
}
pub mod io_t_security_aggregated_recommendation_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReportedSeverity {
        Informational,
        Low,
        Medium,
        High,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAlertedDevice {
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityDeviceAlert {
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_device_alert::ReportedSeverity>,
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i64>,
}
pub mod io_t_security_device_alert {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReportedSeverity {
        Informational,
        Low,
        Medium,
        High,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityDeviceRecommendation {
    #[serde(rename = "recommendationDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_display_name: Option<String>,
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_device_recommendation::ReportedSeverity>,
    #[serde(rename = "devicesCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_count: Option<i64>,
}
pub mod io_t_security_device_recommendation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReportedSeverity {
        Informational,
        Low,
        Medium,
        High,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionsList {
    pub value: Vec<IoTSecuritySolutionModel>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecuritySolutionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionProperties {
    pub workspace: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<io_t_security_solution_properties::Status>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub export: Vec<String>,
    #[serde(rename = "disabledDataSources", default, skip_serializing_if = "Vec::is_empty")]
    pub disabled_data_sources: Vec<String>,
    #[serde(rename = "iotHubs")]
    pub iot_hubs: Vec<String>,
    #[serde(rename = "userDefinedResources", default, skip_serializing_if = "Option::is_none")]
    pub user_defined_resources: Option<UserDefinedResourcesProperties>,
    #[serde(rename = "autoDiscoveredResources", default, skip_serializing_if = "Vec::is_empty")]
    pub auto_discovered_resources: Vec<String>,
    #[serde(rename = "recommendationsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recommendations_configuration: Option<RecommendationConfigurationList>,
}
pub mod io_t_security_solution_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDefinedResourcesProperties {
    pub query: String,
    #[serde(rename = "querySubscriptions")]
    pub query_subscriptions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendationConfigurationProperties {
    #[serde(rename = "recommendationType")]
    pub recommendation_type: recommendation_configuration_properties::RecommendationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub status: recommendation_configuration_properties::Status,
}
pub mod recommendation_configuration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RecommendationType {
        #[serde(rename = "IoT_ACRAuthentication")]
        IoTAcrAuthentication,
        #[serde(rename = "IoT_AgentSendsUnutilizedMessages")]
        IoTAgentSendsUnutilizedMessages,
        #[serde(rename = "IoT_Baseline")]
        IoTBaseline,
        #[serde(rename = "IoT_EdgeHubMemOptimize")]
        IoTEdgeHubMemOptimize,
        #[serde(rename = "IoT_EdgeLoggingOptions")]
        IoTEdgeLoggingOptions,
        #[serde(rename = "IoT_InconsistentModuleSettings")]
        IoTInconsistentModuleSettings,
        #[serde(rename = "IoT_InstallAgent")]
        IoTInstallAgent,
        #[serde(rename = "IoT_IPFilter_DenyAll")]
        IoTIpFilterDenyAll,
        #[serde(rename = "IoT_IPFilter_PermissiveRule")]
        IoTIpFilterPermissiveRule,
        #[serde(rename = "IoT_OpenPorts")]
        IoTOpenPorts,
        #[serde(rename = "IoT_PermissiveFirewallPolicy")]
        IoTPermissiveFirewallPolicy,
        #[serde(rename = "IoT_PermissiveInputFirewallRules")]
        IoTPermissiveInputFirewallRules,
        #[serde(rename = "IoT_PermissiveOutputFirewallRules")]
        IoTPermissiveOutputFirewallRules,
        #[serde(rename = "IoT_PrivilegedDockerOptions")]
        IoTPrivilegedDockerOptions,
        #[serde(rename = "IoT_SharedCredentials")]
        IoTSharedCredentials,
        #[serde(rename = "IoT_VulnerableTLSCipherSuite")]
        IoTVulnerableTlsCipherSuite,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
    }
}
pub type RecommendationConfigurationList = Vec<RecommendationConfigurationProperties>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIotSecuritySolutionData {
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[serde(rename = "userDefinedResources", default, skip_serializing_if = "Option::is_none")]
    pub user_defined_resources: Option<UserDefinedResourcesProperties>,
    #[serde(rename = "recommendationsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recommendations_configuration: Option<RecommendationConfigurationList>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingList {
    pub value: Vec<Pricing>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pricing {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PricingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingProperties {
    #[serde(rename = "pricingTier")]
    pub pricing_tier: pricing_properties::PricingTier,
}
pub mod pricing_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PricingTier {
        Free,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityContactList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SecurityContact>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityContact {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityContactProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityContactProperties {
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "alertNotifications")]
    pub alert_notifications: security_contact_properties::AlertNotifications,
    #[serde(rename = "alertsToAdmins")]
    pub alerts_to_admins: security_contact_properties::AlertsToAdmins,
}
pub mod security_contact_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AlertNotifications {
        On,
        Off,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AlertsToAdmins {
        On,
        Off,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Setting>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSetting {
    #[serde(flatten)]
    pub setting: Setting,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataExportSettingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Setting {
    #[serde(flatten)]
    pub resource: Resource,
    pub kind: setting::Kind,
}
pub mod setting {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        DataExportSetting,
        AlertSuppressionSetting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSettingProperties {
    pub enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSettingList {
    pub value: Vec<WorkspaceSetting>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceSettingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSettingProperties {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    pub scope: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}