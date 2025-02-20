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

/// Avatar : Details of an avatar.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Avatar {
    /// The ID of the avatar.
    #[serde(rename = "id")]
    pub id: String,
    /// The owner of the avatar. For a system avatar the owner is null (and nothing is returned). For non-system avatars this is the appropriate identifier, such as the ID for a project or the account ID for a user.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Whether the avatar is a system avatar.
    #[serde(rename = "isSystemAvatar", skip_serializing_if = "Option::is_none")]
    pub is_system_avatar: Option<bool>,
    /// Whether the avatar is used in Jira. For example, shown as a project's avatar.
    #[serde(rename = "isSelected", skip_serializing_if = "Option::is_none")]
    pub is_selected: Option<bool>,
    /// Whether the avatar can be deleted.
    #[serde(rename = "isDeletable", skip_serializing_if = "Option::is_none")]
    pub is_deletable: Option<bool>,
    /// The file name of the avatar icon. Returned for system avatars.
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The list of avatar icon URLs.
    #[serde(rename = "urls", skip_serializing_if = "Option::is_none")]
    pub urls: Option<std::collections::HashMap<String, String>>,
}

impl Avatar {
    /// Details of an avatar.
    pub fn new(id: String) -> Avatar {
        Avatar {
            id,
            owner: None,
            is_system_avatar: None,
            is_selected: None,
            is_deletable: None,
            file_name: None,
            urls: None,
        }
    }
}

