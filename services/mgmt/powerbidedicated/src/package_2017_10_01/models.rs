#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCapacityNameAvailabilityParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCapacityNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCapacities {
    pub value: Vec<DedicatedCapacity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCapacity {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedCapacityProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCapacityAdministrators {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCapacityMutableProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration: Option<DedicatedCapacityAdministrators>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCapacityProperties {
    #[serde(flatten)]
    pub dedicated_capacity_mutable_properties: DedicatedCapacityMutableProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<dedicated_capacity_properties::State>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<dedicated_capacity_properties::ProvisioningState>,
}
pub mod dedicated_capacity_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCapacityUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedCapacityMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
pub mod error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<operation::Properties>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
        pub service_specification: Option<ServiceSpecification>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    pub sku: ResourceSku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<resource_sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
pub mod resource_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        #[serde(rename = "PBIE_Azure")]
        PbieAzure,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuDetailsForExistingResource {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuEnumerationForExistingResourceResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDetailsForExistingResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuEnumerationForNewResourceResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceSku>,
}
