#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<String>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "sampleProperty", default, skip_serializing_if = "Option::is_none")]
    pub sample_property: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplianceStatus {
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<compliance_status::ComplianceState>,
    #[serde(rename = "lastConfigApplied", default, skip_serializing_if = "Option::is_none")]
    pub last_config_applied: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "messageLevel", default, skip_serializing_if = "Option::is_none")]
    pub message_level: Option<compliance_status::MessageLevel>,
}
pub mod compliance_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComplianceState {
        Pending,
        Compliant,
        Noncompliant,
        Installed,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MessageLevel {
        Error,
        Warning,
        Information,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartVersion {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartValues {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HelmOperatorProperties {
    #[serde(rename = "chartVersion", default, skip_serializing_if = "Option::is_none")]
    pub chart_version: Option<ChartVersion>,
    #[serde(rename = "chartValues", default, skip_serializing_if = "Option::is_none")]
    pub chart_values: Option<ChartValues>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationProtectedSettings {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperatorTypeDefinition {
    Flux,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperatorScopeDefinition {
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "namespace")]
    Namespace,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceControlConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<source_control_configuration::Properties>,
}
pub mod source_control_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "repositoryUrl", default, skip_serializing_if = "Option::is_none")]
        pub repository_url: Option<String>,
        #[serde(rename = "operatorNamespace", default, skip_serializing_if = "Option::is_none")]
        pub operator_namespace: Option<String>,
        #[serde(rename = "operatorInstanceName", default, skip_serializing_if = "Option::is_none")]
        pub operator_instance_name: Option<String>,
        #[serde(rename = "operatorType", default, skip_serializing_if = "Option::is_none")]
        pub operator_type: Option<OperatorTypeDefinition>,
        #[serde(rename = "operatorParams", default, skip_serializing_if = "Option::is_none")]
        pub operator_params: Option<String>,
        #[serde(rename = "configurationProtectedSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_protected_settings: Option<ConfigurationProtectedSettings>,
        #[serde(rename = "operatorScope", default, skip_serializing_if = "Option::is_none")]
        pub operator_scope: Option<OperatorScopeDefinition>,
        #[serde(rename = "repositoryPublicKey", default, skip_serializing_if = "Option::is_none")]
        pub repository_public_key: Option<String>,
        #[serde(rename = "sshKnownHostsContents", default, skip_serializing_if = "Option::is_none")]
        pub ssh_known_hosts_contents: Option<String>,
        #[serde(rename = "enableHelmOperator", default, skip_serializing_if = "Option::is_none")]
        pub enable_helm_operator: Option<bool>,
        #[serde(rename = "helmOperatorProperties", default, skip_serializing_if = "Option::is_none")]
        pub helm_operator_properties: Option<HelmOperatorProperties>,
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
        pub compliance_status: Option<ComplianceStatus>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Accepted,
            Deleting,
            Running,
            Succeeded,
            Failed,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceControlConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SourceControlConfiguration>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
pub mod resource_provider_operation {
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationSettings {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionProtectedSettings {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReleaseTrain {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeCluster {
    #[serde(rename = "releaseNamespace", default, skip_serializing_if = "Option::is_none")]
    pub release_namespace: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeNamespace {
    #[serde(rename = "targetNamespace", default, skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<ScopeCluster>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<ScopeNamespace>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InstallStateDefinition {
    Pending,
    Installed,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<extension_status::Level>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
pub mod extension_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Error,
        Warning,
        Information,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<configuration_identity::Type>,
}
pub mod configuration_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionInstance {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<extension_instance::Properties>,
}
pub mod extension_instance {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "extensionType", default, skip_serializing_if = "Option::is_none")]
        pub extension_type: Option<String>,
        #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
        pub auto_upgrade_minor_version: Option<bool>,
        #[serde(rename = "releaseTrain", default, skip_serializing_if = "Option::is_none")]
        pub release_train: Option<ReleaseTrain>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scope: Option<Scope>,
        #[serde(rename = "configurationSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_settings: Option<ConfigurationSettings>,
        #[serde(rename = "configurationProtectedSettings", default, skip_serializing_if = "Option::is_none")]
        pub configuration_protected_settings: Option<ExtensionProtectedSettings>,
        #[serde(rename = "installState", default, skip_serializing_if = "Option::is_none")]
        pub install_state: Option<InstallStateDefinition>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub statuses: Vec<ExtensionStatus>,
        #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
        pub creation_time: Option<String>,
        #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
        pub last_modified_time: Option<String>,
        #[serde(rename = "lastStatusTime", default, skip_serializing_if = "Option::is_none")]
        pub last_status_time: Option<String>,
        #[serde(rename = "errorInfo", default, skip_serializing_if = "Option::is_none")]
        pub error_info: Option<ErrorDefinition>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub identity: Option<ConfigurationIdentity>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionInstancesList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExtensionInstance>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionInstanceUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<extension_instance_update::Properties>,
}
pub mod extension_instance_update {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
        pub auto_upgrade_minor_version: Option<bool>,
        #[serde(rename = "releaseTrain", default, skip_serializing_if = "Option::is_none")]
        pub release_train: Option<ReleaseTrain>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
}