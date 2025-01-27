#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogSku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<SkuCost>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<SkuRestrictions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentAssociation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommitmentAssociationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentAssociationListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommitmentAssociation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentAssociationProperties {
    #[serde(rename = "associatedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub associated_resource_id: Option<String>,
    #[serde(rename = "commitmentPlanId", default, skip_serializing_if = "Option::is_none")]
    pub commitment_plan_id: Option<String>,
    #[serde(rename = "creationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlan {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommitmentPlanProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlanListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommitmentPlan>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlanPatchPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlanProperties {
    #[serde(rename = "chargeForOverage", default, skip_serializing_if = "Option::is_none")]
    pub charge_for_overage: Option<bool>,
    #[serde(rename = "chargeForPlan", default, skip_serializing_if = "Option::is_none")]
    pub charge_for_plan: Option<bool>,
    #[serde(rename = "creationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "includedQuantities", default, skip_serializing_if = "Option::is_none")]
    pub included_quantities: Option<serde_json::Value>,
    #[serde(rename = "maxAssociationLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_association_limit: Option<i32>,
    #[serde(rename = "maxCapacityLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_capacity_limit: Option<i32>,
    #[serde(rename = "minCapacityLimit", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity_limit: Option<i32>,
    #[serde(rename = "planMeter", default, skip_serializing_if = "Option::is_none")]
    pub plan_meter: Option<String>,
    #[serde(rename = "refillFrequencyInDays", default, skip_serializing_if = "Option::is_none")]
    pub refill_frequency_in_days: Option<i32>,
    #[serde(rename = "suspendPlanOnOverage", default, skip_serializing_if = "Option::is_none")]
    pub suspend_plan_on_overage: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveCommitmentAssociationRequest {
    #[serde(rename = "destinationPlanId", default, skip_serializing_if = "Option::is_none")]
    pub destination_plan_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntityListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanQuantity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "includedQuantityMeter", default, skip_serializing_if = "Option::is_none")]
    pub included_quantity_meter: Option<String>,
    #[serde(rename = "overageMeter", default, skip_serializing_if = "Option::is_none")]
    pub overage_meter: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanUsageHistory {
    #[serde(rename = "planDeletionOverage", default, skip_serializing_if = "Option::is_none")]
    pub plan_deletion_overage: Option<serde_json::Value>,
    #[serde(rename = "planMigrationOverage", default, skip_serializing_if = "Option::is_none")]
    pub plan_migration_overage: Option<serde_json::Value>,
    #[serde(rename = "planQuantitiesAfterUsage", default, skip_serializing_if = "Option::is_none")]
    pub plan_quantities_after_usage: Option<serde_json::Value>,
    #[serde(rename = "planQuantitiesBeforeUsage", default, skip_serializing_if = "Option::is_none")]
    pub plan_quantities_before_usage: Option<serde_json::Value>,
    #[serde(rename = "planUsageOverage", default, skip_serializing_if = "Option::is_none")]
    pub plan_usage_overage: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<serde_json::Value>,
    #[serde(rename = "usageDate", default, skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanUsageHistoryListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PlanUsageHistory>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub location: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapacity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<sku_capacity::ScaleType>,
}
pub mod sku_capacity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        Automatic,
        Manual,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCost {
    #[serde(rename = "meterID", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(rename = "extendedUnit", default, skip_serializing_if = "Option::is_none")]
    pub extended_unit: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CatalogSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuRestrictions {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sku_restrictions::Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<sku_restrictions::ReasonCode>,
}
pub mod sku_restrictions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "location")]
        Location,
        #[serde(rename = "zone")]
        Zone,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tags {}
