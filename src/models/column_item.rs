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

/// ColumnItem : Details of an issue navigator column item.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnItem {
    /// The issue navigator column label.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The issue navigator column value.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ColumnItem {
    /// Details of an issue navigator column item.
    pub fn new() -> ColumnItem {
        ColumnItem {
            label: None,
            value: None,
        }
    }
}

