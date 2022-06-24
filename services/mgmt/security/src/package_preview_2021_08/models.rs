#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "describe the properties of a security assessment object reference (by key)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignedComponentItem {
    #[doc = "unique key to a security assessment object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl AssignedComponentItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describe the properties of a of a security standard object reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignedStandardItem {
    #[doc = "full resourceId of the Microsoft.Security/standard object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl AssignedStandardItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security Assignment on a resource group over a given scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Assignment {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of a standardAssignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssignmentProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Assignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Page of a standard assignment list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentList {
    #[doc = "Collection of standardAssignments in this page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Assignment>,
    #[doc = "The URI to fetch the next page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssignmentList {
    fn continuation(&self) -> Option<azure_core::prelude::Continuation> {
        self.next_link.clone().map(azure_core::prelude::Continuation::from)
    }
}
impl AssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a standardAssignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentProperties {
    #[doc = "display name of the standardAssignment"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "description of the standardAssignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "describe the properties of a of a security standard object reference"]
    #[serde(rename = "assignedStandard", default, skip_serializing_if = "Option::is_none")]
    pub assigned_standard: Option<AssignedStandardItem>,
    #[doc = "describe the properties of a security assessment object reference (by key)"]
    #[serde(rename = "assignedComponent", default, skip_serializing_if = "Option::is_none")]
    pub assigned_component: Option<AssignedComponentItem>,
    #[doc = "Scope to which the standardAssignment applies - can be a subscription path or a resource group under that subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "expected effect of this assignment (Disable/Exempt/etc)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[doc = "Expiration date of this assignment as a full ISO date"]
    #[serde(rename = "expiresOn", default, skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    #[doc = "Additional data about the assignment"]
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<assignment_properties::AdditionalData>,
    #[doc = "The assignment metadata. Metadata is an open ended object and is typically a collection of key value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl AssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assignment_properties {
    use super::*;
    #[doc = "Additional data about the assignment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct AdditionalData {
        #[doc = "Exemption category of this assignment"]
        #[serde(rename = "exemptionCategory", default, skip_serializing_if = "Option::is_none")]
        pub exemption_category: Option<String>,
    }
    impl AdditionalData {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Describes an Azure resource with location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureTrackedResourceLocation {
    #[doc = "Location where the resource is stored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl AzureTrackedResourceLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    fn continuation(&self) -> Option<azure_core::prelude::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Entity tag is used for comparing two or more entities from the same requested resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ETag {
    #[doc = "Entity tag is used for comparing two or more entities from the same requested resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ETag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure resource with kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Kind {
    #[doc = "Kind of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl Kind {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security Standard on a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Standard {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes properties of a standard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StandardProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Standard {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of an component as related to the standard"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StandardComponentProperties {
    #[doc = "Component Key matching componentMetadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl StandardComponentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Page of a Standard list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StandardList {
    #[doc = "Collection of standards in this page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Standard>,
    #[doc = "The URI to fetch the next page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StandardList {
    fn continuation(&self) -> Option<azure_core::prelude::Continuation> {
        self.next_link.clone().map(azure_core::prelude::Continuation::from)
    }
}
impl StandardList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of a standard."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StandardProperties {
    #[doc = "display name of the standard, equivalent to the standardId"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "standard type (Custom or BuiltIn only currently)"]
    #[serde(rename = "standardType", default, skip_serializing_if = "Option::is_none")]
    pub standard_type: Option<String>,
    #[doc = "description of the standard"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "category of the standard provided"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "List of component objects containing component unique keys (such as assessment keys) to apply to standard scope.  Currently only supports assessment keys."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<StandardComponentProperties>,
    #[doc = "List of all standard supported clouds."]
    #[serde(rename = "supportedClouds", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_clouds: Vec<StandardSupportedClouds>,
}
impl StandardProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The cloud that the standard is supported on."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StandardSupportedClouds {
    #[serde(rename = "AWS")]
    Aws,
    #[serde(rename = "GCP")]
    Gcp,
}
#[doc = "A list of key value pairs that describe the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "A list of key value pairs that describe the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure tracked resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub azure_tracked_resource_location: AzureTrackedResourceLocation,
    #[serde(flatten)]
    pub kind: Kind,
    #[serde(flatten)]
    pub e_tag: ETag,
    #[serde(flatten)]
    pub tags: Tags,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
