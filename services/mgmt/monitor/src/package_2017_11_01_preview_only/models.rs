#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Baseline {
    pub sensitivity: baseline::Sensitivity,
    #[serde(rename = "lowThresholds")]
    pub low_thresholds: Vec<f64>,
    #[serde(rename = "highThresholds")]
    pub high_thresholds: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
}
pub mod baseline {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Sensitivity {
        Low,
        Medium,
        High,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineMetadataValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timespan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub baseline: Vec<Baseline>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<BaselineMetadataValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BaselineProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculateBaselineResponse {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
    pub baseline: Vec<Baseline>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    pub value: String,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesInformation {
    pub sensitivities: Vec<String>,
    pub values: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
}
