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

/// SecurityScheme : Details about a security scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityScheme {
    /// The URL of the issue security scheme.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The ID of the issue security scheme.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the issue security scheme.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the issue security scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the default security level.
    #[serde(rename = "defaultSecurityLevelId", skip_serializing_if = "Option::is_none")]
    pub default_security_level_id: Option<i64>,
    #[serde(rename = "levels", skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<models::SecurityLevel>>,
}

impl SecurityScheme {
    /// Details about a security scheme.
    pub fn new() -> SecurityScheme {
        SecurityScheme {
            param_self: None,
            id: None,
            name: None,
            description: None,
            default_security_level_id: None,
            levels: None,
        }
    }
}

