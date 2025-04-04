/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldContextDefaultValueCascadingOption : Default value for a cascading select custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueCascadingOption {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the default option.
    #[serde(rename = "optionId")]
    pub option_id: String,
    /// The ID of the default cascading option.
    #[serde(rename = "cascadingOptionId", skip_serializing_if = "Option::is_none")]
    pub cascading_option_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueCascadingOption {
    /// Default value for a cascading select custom field.
    pub fn new(context_id: String, option_id: String, r#type: String) -> CustomFieldContextDefaultValueCascadingOption {
        CustomFieldContextDefaultValueCascadingOption {
            context_id,
            option_id,
            cascading_option_id: None,
            r#type,
        }
    }
}

