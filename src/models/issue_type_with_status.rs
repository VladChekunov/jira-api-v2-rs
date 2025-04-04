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

/// IssueTypeWithStatus : Status details for an issue type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeWithStatus {
    /// The URL of the issue type's status details.
    #[serde(rename = "self")]
    pub param_self: String,
    /// The ID of the issue type.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the issue type.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether this issue type represents subtasks.
    #[serde(rename = "subtask")]
    pub subtask: bool,
    /// List of status details for the issue type.
    #[serde(rename = "statuses")]
    pub statuses: Vec<models::StatusDetails>,
}

impl IssueTypeWithStatus {
    /// Status details for an issue type.
    pub fn new(param_self: String, id: String, name: String, subtask: bool, statuses: Vec<models::StatusDetails>) -> IssueTypeWithStatus {
        IssueTypeWithStatus {
            param_self,
            id,
            name,
            subtask,
            statuses,
        }
    }
}

