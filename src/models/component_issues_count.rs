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

/// ComponentIssuesCount : Count of issues assigned to a component.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentIssuesCount {
    /// The URL for this count of issues for a component.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The count of issues assigned to a component.
    #[serde(rename = "issueCount", skip_serializing_if = "Option::is_none")]
    pub issue_count: Option<i64>,
}

impl ComponentIssuesCount {
    /// Count of issues assigned to a component.
    pub fn new() -> ComponentIssuesCount {
        ComponentIssuesCount {
            param_self: None,
            issue_count: None,
        }
    }
}

