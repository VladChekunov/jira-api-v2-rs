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

/// FieldConfiguration : Details of a field configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfiguration {
    /// The ID of the field configuration.
    #[serde(rename = "id")]
    pub id: i64,
    /// The name of the field configuration.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the field configuration.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether the field configuration is the default.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl FieldConfiguration {
    /// Details of a field configuration.
    pub fn new(id: i64, name: String, description: String) -> FieldConfiguration {
        FieldConfiguration {
            id,
            name,
            description,
            is_default: None,
        }
    }
}

