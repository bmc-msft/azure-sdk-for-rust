#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "actionGroupId", default, skip_serializing_if = "Option::is_none")]
    pub action_group_id: Option<String>,
    #[serde(rename = "webHookProperties", default, skip_serializing_if = "Option::is_none")]
    pub web_hook_properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: condition::TimeAggregation,
    #[serde(rename = "metricMeasureColumn", default, skip_serializing_if = "Option::is_none")]
    pub metric_measure_column: Option<String>,
    #[serde(rename = "resourceIdColumn", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_column: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    pub operator: condition::Operator,
    pub threshold: f64,
    #[serde(rename = "failingPeriods", default, skip_serializing_if = "Option::is_none")]
    pub failing_periods: Option<condition::FailingPeriods>,
}
pub mod condition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeAggregation {
        Count,
        Average,
        Minimum,
        Maximum,
        Total,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct FailingPeriods {
        #[serde(rename = "numberOfEvaluationPeriods", default, skip_serializing_if = "Option::is_none")]
        pub number_of_evaluation_periods: Option<i64>,
        #[serde(rename = "minFailingPeriodsToAlert", default, skip_serializing_if = "Option::is_none")]
        pub min_failing_periods_to_alert: Option<i64>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    pub name: String,
    pub operator: dimension::Operator,
    pub values: Vec<String>,
}
pub mod dimension {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Include,
        Exclude,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorContract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledQueryRuleCriteria {
    #[serde(rename = "allOf", default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<Condition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledQueryRuleProperties {
    #[serde(rename = "createdWithApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub created_with_api_version: Option<String>,
    #[serde(rename = "isLegacyLogAnalyticsRule", default, skip_serializing_if = "Option::is_none")]
    pub is_legacy_log_analytics_rule: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<scheduled_query_rule_properties::Severity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "evaluationFrequency", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[serde(rename = "overrideQueryTimeRange", default, skip_serializing_if = "Option::is_none")]
    pub override_query_time_range: Option<String>,
    #[serde(rename = "targetResourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub target_resource_types: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<ScheduledQueryRuleCriteria>,
    #[serde(rename = "muteActionsDuration", default, skip_serializing_if = "Option::is_none")]
    pub mute_actions_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
}
pub mod scheduled_query_rule_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        #[serde(rename = "0")]
        N0,
        #[serde(rename = "1")]
        N1,
        #[serde(rename = "2")]
        N2,
        #[serde(rename = "3")]
        N3,
        #[serde(rename = "4")]
        N4,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledQueryRuleResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    pub properties: ScheduledQueryRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledQueryRuleResourceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScheduledQueryRuleResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledQueryRuleResourcePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledQueryRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
