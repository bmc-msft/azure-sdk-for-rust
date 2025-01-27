#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponent {
    #[serde(flatten)]
    pub components_resource: ComponentsResource,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationInsightsComponentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentListResult {
    pub value: Vec<ApplicationInsightsComponent>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentProperties {
    #[serde(rename = "ApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "AppId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Application_Type")]
    pub application_type: application_insights_component_properties::ApplicationType,
    #[serde(rename = "Flow_Type", default, skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<application_insights_component_properties::FlowType>,
    #[serde(rename = "Request_Source", default, skip_serializing_if = "Option::is_none")]
    pub request_source: Option<application_insights_component_properties::RequestSource>,
    #[serde(rename = "InstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub instrumentation_key: Option<String>,
    #[serde(rename = "CreationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "TenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "HockeyAppId", default, skip_serializing_if = "Option::is_none")]
    pub hockey_app_id: Option<String>,
    #[serde(rename = "HockeyAppToken", default, skip_serializing_if = "Option::is_none")]
    pub hockey_app_token: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "SamplingPercentage", default, skip_serializing_if = "Option::is_none")]
    pub sampling_percentage: Option<f64>,
    #[serde(rename = "ConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[serde(rename = "RetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i64>,
    #[serde(rename = "DisableIpMasking", default, skip_serializing_if = "Option::is_none")]
    pub disable_ip_masking: Option<bool>,
    #[serde(rename = "ImmediatePurgeDataOn30Days", default, skip_serializing_if = "Option::is_none")]
    pub immediate_purge_data_on30_days: Option<bool>,
    #[serde(rename = "WorkspaceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_resource_id: Option<String>,
    #[serde(rename = "LaMigrationDate", default, skip_serializing_if = "Option::is_none")]
    pub la_migration_date: Option<String>,
    #[serde(rename = "PrivateLinkScopedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_scoped_resources: Vec<PrivateLinkScopedResource>,
    #[serde(rename = "publicNetworkAccessForIngestion", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_ingestion: Option<PublicNetworkAccessType>,
    #[serde(rename = "publicNetworkAccessForQuery", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_query: Option<PublicNetworkAccessType>,
    #[serde(rename = "IngestionMode", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_mode: Option<application_insights_component_properties::IngestionMode>,
    #[serde(rename = "DisableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[serde(rename = "ForceCustomerStorageForProfiler", default, skip_serializing_if = "Option::is_none")]
    pub force_customer_storage_for_profiler: Option<bool>,
}
pub mod application_insights_component_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApplicationType {
        #[serde(rename = "web")]
        Web,
        #[serde(rename = "other")]
        Other,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FlowType {
        Bluefield,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RequestSource {
        #[serde(rename = "rest")]
        Rest,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IngestionMode {
        ApplicationInsights,
        ApplicationInsightsWithDiagnosticSettings,
        LogAnalytics,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeBody {
    pub table: String,
    pub filters: Vec<ComponentPurgeBodyFilters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeBodyFilters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeResponse {
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeStatusResponse {
    pub status: component_purge_status_response::Status,
}
pub mod component_purge_status_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopedResource {
    #[serde(rename = "ResourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
