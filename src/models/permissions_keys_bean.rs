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
pub struct PermissionsKeysBean {
    /// A list of permission keys.
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
}

impl PermissionsKeysBean {
    pub fn new(permissions: Vec<String>) -> PermissionsKeysBean {
        PermissionsKeysBean {
            permissions,
        }
    }
}

