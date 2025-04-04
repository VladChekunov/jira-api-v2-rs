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

/// Status : The status of the item.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    /// Whether the item is resolved. If set to \"true\", the link to the issue is displayed in a strikethrough font, otherwise the link displays in normal font.
    #[serde(rename = "resolved", skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
    /// Details of the icon representing the status. If not provided, no status icon displays in Jira.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<models::Icon>,
}

impl Status {
    /// The status of the item.
    pub fn new() -> Status {
        Status {
            resolved: None,
            icon: None,
        }
    }
}

