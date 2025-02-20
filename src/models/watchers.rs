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

/// Watchers : The details of watchers on an issue.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Watchers {
    /// The URL of these issue watcher details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// Whether the calling user is watching this issue.
    #[serde(rename = "isWatching", skip_serializing_if = "Option::is_none")]
    pub is_watching: Option<bool>,
    /// The number of users watching this issue.
    #[serde(rename = "watchCount", skip_serializing_if = "Option::is_none")]
    pub watch_count: Option<i32>,
    /// Details of the users watching this issue.
    #[serde(rename = "watchers", skip_serializing_if = "Option::is_none")]
    pub watchers: Option<Vec<models::UserDetails>>,
}

impl Watchers {
    /// The details of watchers on an issue.
    pub fn new() -> Watchers {
        Watchers {
            param_self: None,
            is_watching: None,
            watch_count: None,
            watchers: None,
        }
    }
}

