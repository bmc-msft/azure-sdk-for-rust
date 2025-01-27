#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentPricingPlan {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PricingPlanProperties>,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EaSubscriptionMigrationDate {
    #[serde(rename = "isGrandFatherableSubscription", default, skip_serializing_if = "Option::is_none")]
    pub is_grand_fatherable_subscription: Option<bool>,
    #[serde(rename = "optedInDate", default, skip_serializing_if = "Option::is_none")]
    pub opted_in_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingPlanProperties {
    #[serde(rename = "planType", default, skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<f64>,
    #[serde(rename = "resetHour", default, skip_serializing_if = "Option::is_none")]
    pub reset_hour: Option<i64>,
    #[serde(rename = "warningThreshold", default, skip_serializing_if = "Option::is_none")]
    pub warning_threshold: Option<i64>,
    #[serde(rename = "stopSendNotificationWhenHitThreshold", default, skip_serializing_if = "Option::is_none")]
    pub stop_send_notification_when_hit_threshold: Option<bool>,
    #[serde(rename = "stopSendNotificationWhenHitCap", default, skip_serializing_if = "Option::is_none")]
    pub stop_send_notification_when_hit_cap: Option<bool>,
    #[serde(rename = "maxHistoryCap", default, skip_serializing_if = "Option::is_none")]
    pub max_history_cap: Option<f64>,
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
