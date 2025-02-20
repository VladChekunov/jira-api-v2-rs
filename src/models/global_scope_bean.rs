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
pub struct GlobalScopeBean {
    /// Defines the behavior of the option in the global context.If notSelectable is set, the option cannot be set as the field's value. This is useful for archiving an option that has previously been selected but shouldn't be used anymore.If defaultValue is set, the option is selected by default.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashSet<Attributes>>,
}

impl GlobalScopeBean {
    pub fn new() -> GlobalScopeBean {
        GlobalScopeBean {
            attributes: None,
        }
    }
}
/// Defines the behavior of the option in the global context.If notSelectable is set, the option cannot be set as the field's value. This is useful for archiving an option that has previously been selected but shouldn't be used anymore.If defaultValue is set, the option is selected by default.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Attributes {
    #[serde(rename = "notSelectable")]
    NotSelectable,
    #[serde(rename = "defaultValue")]
    DefaultValue,
}

impl Default for Attributes {
    fn default() -> Attributes {
        Self::NotSelectable
    }
}

