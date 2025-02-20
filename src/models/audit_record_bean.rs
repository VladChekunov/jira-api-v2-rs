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

/// AuditRecordBean : An audit record.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditRecordBean {
    /// The ID of the audit record.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The summary of the audit record.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// The URL of the computer where the creation of the audit record was initiated.
    #[serde(rename = "remoteAddress", skip_serializing_if = "Option::is_none")]
    pub remote_address: Option<String>,
    /// Deprecated, use `authorAccountId` instead. The key of the user who created the audit record.
    #[serde(rename = "authorKey", skip_serializing_if = "Option::is_none")]
    pub author_key: Option<String>,
    /// The date and time on which the audit record was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The category of the audit record. For a list of these categories, see the help article [Auditing in Jira applications](https://confluence.atlassian.com/x/noXKM).
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The event the audit record originated from.
    #[serde(rename = "eventSource", skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    /// The description of the audit record.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "objectItem", skip_serializing_if = "Option::is_none")]
    pub object_item: Option<Box<models::AssociatedItemBean>>,
    /// The list of values changed in the record event.
    #[serde(rename = "changedValues", skip_serializing_if = "Option::is_none")]
    pub changed_values: Option<Vec<models::ChangedValueBean>>,
    /// The list of items associated with the changed record.
    #[serde(rename = "associatedItems", skip_serializing_if = "Option::is_none")]
    pub associated_items: Option<Vec<models::AssociatedItemBean>>,
}

impl AuditRecordBean {
    /// An audit record.
    pub fn new() -> AuditRecordBean {
        AuditRecordBean {
            id: None,
            summary: None,
            remote_address: None,
            author_key: None,
            created: None,
            category: None,
            event_source: None,
            description: None,
            object_item: None,
            changed_values: None,
            associated_items: None,
        }
    }
}

