#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeaderField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInfo {
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
pub struct OperationsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTest {
    #[serde(flatten)]
    pub webtests_resource: WebtestsResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<web_test::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebTestProperties>,
}
pub mod web_test {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "ping")]
        Ping,
        #[serde(rename = "multistep")]
        Multistep,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTestGeolocation {
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTestProperties {
    #[serde(rename = "SyntheticMonitorId")]
    pub synthetic_monitor_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Frequency", default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    #[serde(rename = "Timeout", default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "Kind")]
    pub kind: web_test_properties::Kind,
    #[serde(rename = "RetryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub retry_enabled: Option<bool>,
    #[serde(rename = "Locations")]
    pub locations: Vec<WebTestGeolocation>,
    #[serde(rename = "Configuration", default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<web_test_properties::Configuration>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "Request", default, skip_serializing_if = "Option::is_none")]
    pub request: Option<web_test_properties::Request>,
    #[serde(rename = "ValidationRules", default, skip_serializing_if = "Option::is_none")]
    pub validation_rules: Option<web_test_properties::ValidationRules>,
}
pub mod web_test_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "ping")]
        Ping,
        #[serde(rename = "multistep")]
        Multistep,
        #[serde(rename = "basic")]
        Basic,
        #[serde(rename = "standard")]
        Standard,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Configuration {
        #[serde(rename = "WebTest", default, skip_serializing_if = "Option::is_none")]
        pub web_test: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Request {
        #[serde(rename = "RequestUrl", default, skip_serializing_if = "Option::is_none")]
        pub request_url: Option<String>,
        #[serde(rename = "Headers", default, skip_serializing_if = "Vec::is_empty")]
        pub headers: Vec<HeaderField>,
        #[serde(rename = "HttpVerb", default, skip_serializing_if = "Option::is_none")]
        pub http_verb: Option<String>,
        #[serde(rename = "RequestBody", default, skip_serializing_if = "Option::is_none")]
        pub request_body: Option<String>,
        #[serde(rename = "ParseDependentRequests", default, skip_serializing_if = "Option::is_none")]
        pub parse_dependent_requests: Option<bool>,
        #[serde(rename = "FollowRedirects", default, skip_serializing_if = "Option::is_none")]
        pub follow_redirects: Option<bool>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ValidationRules {
        #[serde(rename = "ContentValidation", default, skip_serializing_if = "Option::is_none")]
        pub content_validation: Option<validation_rules::ContentValidation>,
        #[serde(rename = "SSLCheck", default, skip_serializing_if = "Option::is_none")]
        pub ssl_check: Option<bool>,
        #[serde(rename = "SSLCertRemainingLifetimeCheck", default, skip_serializing_if = "Option::is_none")]
        pub ssl_cert_remaining_lifetime_check: Option<i32>,
        #[serde(rename = "ExpectedHttpStatusCode", default, skip_serializing_if = "Option::is_none")]
        pub expected_http_status_code: Option<i32>,
        #[serde(rename = "IgnoreHttpsStatusCode", default, skip_serializing_if = "Option::is_none")]
        pub ignore_https_status_code: Option<bool>,
    }
    pub mod validation_rules {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct ContentValidation {
            #[serde(rename = "ContentMatch", default, skip_serializing_if = "Option::is_none")]
            pub content_match: Option<String>,
            #[serde(rename = "IgnoreCase", default, skip_serializing_if = "Option::is_none")]
            pub ignore_case: Option<bool>,
            #[serde(rename = "PassIfTextFound", default, skip_serializing_if = "Option::is_none")]
            pub pass_if_text_found: Option<bool>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebtestsResource {
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
pub struct WebTestListResult {
    pub value: Vec<WebTest>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
