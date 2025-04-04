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

/// Fields : Key fields from the linked issue.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fields {
    /// The summary description of the linked issue.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// The status of the linked issue.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::StatusDetails>,
    /// The priority of the linked issue.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<models::Priority>,
    /// The assignee of the linked issue.
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Box<models::UserDetails>>,
    /// The time tracking of the linked issue.
    #[serde(rename = "timetracking", skip_serializing_if = "Option::is_none")]
    pub timetracking: Option<Box<models::TimeTrackingDetails>>,
    #[serde(rename = "issuetype", skip_serializing_if = "Option::is_none")]
    pub issuetype: Option<Box<models::IssueTypeDetails>>,
    /// The type of the linked issue.
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<Box<models::IssueTypeDetails>>,
}

impl Fields {
    /// Key fields from the linked issue.
    pub fn new() -> Fields {
        Fields {
            summary: None,
            status: None,
            priority: None,
            assignee: None,
            timetracking: None,
            issuetype: None,
            issue_type: None,
        }
    }
}

