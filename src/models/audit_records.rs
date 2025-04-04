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

/// AuditRecords : Container for a list of audit records.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditRecords {
    /// The number of audit items skipped before the first item in this list.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// The requested or default limit on the number of audit items to be returned.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// The total number of audit items returned.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// The list of audit items.
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<models::AuditRecordBean>>,
}

impl AuditRecords {
    /// Container for a list of audit records.
    pub fn new() -> AuditRecords {
        AuditRecords {
            offset: None,
            limit: None,
            total: None,
            records: None,
        }
    }
}

