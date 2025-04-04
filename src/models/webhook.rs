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

/// Webhook : A webhook.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    /// The ID of the webhook.
    #[serde(rename = "id")]
    pub id: i64,
    /// The JQL filter that specifies which issues the webhook is sent for.
    #[serde(rename = "jqlFilter")]
    pub jql_filter: String,
    /// The Jira events that trigger the webhook.
    #[serde(rename = "events")]
    pub events: Vec<Events>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: i64,
}

impl Webhook {
    /// A webhook.
    pub fn new(id: i64, jql_filter: String, events: Vec<Events>, expiration_date: i64) -> Webhook {
        Webhook {
            id,
            jql_filter,
            events,
            expiration_date,
        }
    }
}
/// The Jira events that trigger the webhook.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Events {
    #[serde(rename = "jira:issue_created")]
    JiraColonIssueCreated,
    #[serde(rename = "jira:issue_updated")]
    JiraColonIssueUpdated,
    #[serde(rename = "jira:issue_deleted")]
    JiraColonIssueDeleted,
    #[serde(rename = "comment_created")]
    CommentCreated,
    #[serde(rename = "comment_updated")]
    CommentUpdated,
    #[serde(rename = "comment_deleted")]
    CommentDeleted,
    #[serde(rename = "issue_property_set")]
    IssuePropertySet,
    #[serde(rename = "issue_property_deleted")]
    IssuePropertyDeleted,
}

impl Default for Events {
    fn default() -> Events {
        Self::JiraColonIssueCreated
    }
}

