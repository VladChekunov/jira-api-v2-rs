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

/// FieldDetails : Details about a field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldDetails {
    /// The ID of the field.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the field.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the field.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the field is a custom field.
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    /// Whether the content of the field can be used to order lists.
    #[serde(rename = "orderable", skip_serializing_if = "Option::is_none")]
    pub orderable: Option<bool>,
    /// Whether the field can be used as a column on the issue navigator.
    #[serde(rename = "navigable", skip_serializing_if = "Option::is_none")]
    pub navigable: Option<bool>,
    /// Whether the content of the field can be searched.
    #[serde(rename = "searchable", skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    /// The names that can be used to reference the field in an advanced search. For more information, see [Advanced searching - fields reference](https://confluence.atlassian.com/x/gwORLQ).
    #[serde(rename = "clauseNames", skip_serializing_if = "Option::is_none")]
    pub clause_names: Option<Vec<String>>,
    /// The scope of the field.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::Scope>,
    /// The data schema for the field.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<models::JsonTypeBean>>,
}

impl FieldDetails {
    /// Details about a field.
    pub fn new() -> FieldDetails {
        FieldDetails {
            id: None,
            key: None,
            name: None,
            custom: None,
            orderable: None,
            navigable: None,
            searchable: None,
            clause_names: None,
            scope: None,
            schema: None,
        }
    }
}

