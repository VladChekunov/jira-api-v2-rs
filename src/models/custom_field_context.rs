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

/// CustomFieldContext : The details of a custom field context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContext {
    /// The ID of the context.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the context.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the context.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether the context is global.
    #[serde(rename = "isGlobalContext")]
    pub is_global_context: bool,
    /// Whether the context apply to all issue types.
    #[serde(rename = "isAnyIssueType")]
    pub is_any_issue_type: bool,
}

impl CustomFieldContext {
    /// The details of a custom field context.
    pub fn new(id: String, name: String, description: String, is_global_context: bool, is_any_issue_type: bool) -> CustomFieldContext {
        CustomFieldContext {
            id,
            name,
            description,
            is_global_context,
            is_any_issue_type,
        }
    }
}

