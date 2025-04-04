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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleListWrapperApplicationRole {
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::ApplicationRole>>,
    #[serde(rename = "pagingCallback", skip_serializing_if = "Option::is_none")]
    pub paging_callback: Option<serde_json::Value>,
    #[serde(rename = "callback", skip_serializing_if = "Option::is_none")]
    pub callback: Option<serde_json::Value>,
    #[serde(rename = "max-results", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl SimpleListWrapperApplicationRole {
    pub fn new() -> SimpleListWrapperApplicationRole {
        SimpleListWrapperApplicationRole {
            size: None,
            items: None,
            paging_callback: None,
            callback: None,
            max_results: None,
        }
    }
}

