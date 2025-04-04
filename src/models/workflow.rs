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

/// Workflow : Details about a workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(rename = "id")]
    pub id: Box<models::PublishedWorkflowId>,
    /// The description of the workflow.
    #[serde(rename = "description")]
    pub description: String,
    /// The transitions of the workflow.
    #[serde(rename = "transitions", skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<models::Transition>>,
    /// The statuses of the workflow.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<models::WorkflowStatus>>,
    /// Whether this is the default workflow.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl Workflow {
    /// Details about a workflow.
    pub fn new(id: models::PublishedWorkflowId, description: String) -> Workflow {
        Workflow {
            id: Box::new(id),
            description,
            transitions: None,
            statuses: None,
            is_default: None,
        }
    }
}

