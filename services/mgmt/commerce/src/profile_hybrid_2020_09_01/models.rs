#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeterInfo {
    #[serde(rename = "MeterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "MeterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[serde(rename = "MeterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[serde(rename = "MeterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[serde(rename = "Unit", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "MeterTags", default, skip_serializing_if = "Vec::is_empty")]
    pub meter_tags: Vec<String>,
    #[serde(rename = "MeterRegion", default, skip_serializing_if = "Option::is_none")]
    pub meter_region: Option<String>,
    #[serde(rename = "MeterRates", default, skip_serializing_if = "Option::is_none")]
    pub meter_rates: Option<serde_json::Value>,
    #[serde(rename = "EffectiveDate", default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(rename = "IncludedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub included_quantity: Option<f32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonetaryCommitment {
    #[serde(flatten)]
    pub offer_term_info: OfferTermInfo,
    #[serde(rename = "TieredDiscount", default, skip_serializing_if = "Option::is_none")]
    pub tiered_discount: Option<serde_json::Value>,
    #[serde(rename = "ExcludedMeterIds", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_meter_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonetaryCredit {
    #[serde(flatten)]
    pub offer_term_info: OfferTermInfo,
    #[serde(rename = "Credit", default, skip_serializing_if = "Option::is_none")]
    pub credit: Option<f64>,
    #[serde(rename = "ExcludedMeterIds", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_meter_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferTermInfo {
    #[serde(rename = "Name")]
    pub name: offer_term_info::Name,
    #[serde(rename = "EffectiveDate", default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
}
pub mod offer_term_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Recurring Charge")]
        RecurringCharge,
        #[serde(rename = "Monetary Commitment")]
        MonetaryCommitment,
        #[serde(rename = "Monetary Credit")]
        MonetaryCredit,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RateCardQueryParameters {
    #[serde(rename = "OfferDurableId")]
    pub offer_durable_id: String,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Locale")]
    pub locale: String,
    #[serde(rename = "RegionInfo")]
    pub region_info: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurringCharge {
    #[serde(flatten)]
    pub offer_term_info: OfferTermInfo,
    #[serde(rename = "RecurringCharge", default, skip_serializing_if = "Option::is_none")]
    pub recurring_charge: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRateCardInfo {
    #[serde(rename = "Currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "Locale", default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "IsTaxIncluded", default, skip_serializing_if = "Option::is_none")]
    pub is_tax_included: Option<bool>,
    #[serde(rename = "OfferTerms", default, skip_serializing_if = "Vec::is_empty")]
    pub offer_terms: Vec<OfferTermInfo>,
    #[serde(rename = "Meters", default, skip_serializing_if = "Vec::is_empty")]
    pub meters: Vec<MeterInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAggregation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UsageSample>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAggregationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageAggregation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSample {
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "usageStartTime", default, skip_serializing_if = "Option::is_none")]
    pub usage_start_time: Option<String>,
    #[serde(rename = "usageEndTime", default, skip_serializing_if = "Option::is_none")]
    pub usage_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "meterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[serde(rename = "meterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[serde(rename = "meterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[serde(rename = "meterRegion", default, skip_serializing_if = "Option::is_none")]
    pub meter_region: Option<String>,
    #[serde(rename = "infoFields", default, skip_serializing_if = "Option::is_none")]
    pub info_fields: Option<InfoField>,
    #[serde(rename = "instanceData", default, skip_serializing_if = "Option::is_none")]
    pub instance_data: Option<String>,
}
