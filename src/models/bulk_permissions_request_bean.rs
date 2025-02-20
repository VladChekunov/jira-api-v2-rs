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

/// BulkPermissionsRequestBean : Details of global permissions to look up and project permissions with associated projects and issues to look up.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkPermissionsRequestBean {
    /// Project permissions with associated projects and issues to look up.
    #[serde(rename = "projectPermissions", skip_serializing_if = "Option::is_none")]
    pub project_permissions: Option<Vec<models::BulkProjectPermissions>>,
    /// Global permissions to look up.
    #[serde(rename = "globalPermissions", skip_serializing_if = "Option::is_none")]
    pub global_permissions: Option<Vec<String>>,
    /// The account ID of a user.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl BulkPermissionsRequestBean {
    /// Details of global permissions to look up and project permissions with associated projects and issues to look up.
    pub fn new() -> BulkPermissionsRequestBean {
        BulkPermissionsRequestBean {
            project_permissions: None,
            global_permissions: None,
            account_id: None,
        }
    }
}

