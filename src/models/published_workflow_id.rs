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

/// PublishedWorkflowId : Properties that identify a published workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedWorkflowId {
    /// The name of the workflow.
    #[serde(rename = "name")]
    pub name: String,
    /// The entity ID of the workflow.
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

impl PublishedWorkflowId {
    /// Properties that identify a published workflow.
    pub fn new(name: String) -> PublishedWorkflowId {
        PublishedWorkflowId {
            name,
            entity_id: None,
        }
    }
}

