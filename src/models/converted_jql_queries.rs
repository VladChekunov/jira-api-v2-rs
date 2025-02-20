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

/// ConvertedJqlQueries : The converted JQL queries.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConvertedJqlQueries {
    /// The list of converted query strings with account IDs in place of user identifiers.
    #[serde(rename = "queryStrings", skip_serializing_if = "Option::is_none")]
    pub query_strings: Option<Vec<String>>,
    /// List of queries containing user information that could not be mapped to an existing user
    #[serde(rename = "queriesWithUnknownUsers", skip_serializing_if = "Option::is_none")]
    pub queries_with_unknown_users: Option<Vec<models::JqlQueryWithUnknownUsers>>,
}

impl ConvertedJqlQueries {
    /// The converted JQL queries.
    pub fn new() -> ConvertedJqlQueries {
        ConvertedJqlQueries {
            query_strings: None,
            queries_with_unknown_users: None,
        }
    }
}

