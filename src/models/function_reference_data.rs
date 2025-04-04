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

/// FunctionReferenceData : Details of functions that can be used in advanced searches.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionReferenceData {
    /// The function identifier.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The display name of the function.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether the function can take a list of arguments.
    #[serde(rename = "isList", skip_serializing_if = "Option::is_none")]
    pub is_list: Option<IsList>,
    /// The data types returned by the function.
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

impl FunctionReferenceData {
    /// Details of functions that can be used in advanced searches.
    pub fn new() -> FunctionReferenceData {
        FunctionReferenceData {
            value: None,
            display_name: None,
            is_list: None,
            types: None,
        }
    }
}
/// Whether the function can take a list of arguments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsList {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for IsList {
    fn default() -> IsList {
        Self::True
    }
}

