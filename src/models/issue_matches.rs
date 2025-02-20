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

/// IssueMatches : A list of matched issues or errors for each JQL query, in the order the JQL queries were passed.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueMatches {
    #[serde(rename = "matches")]
    pub matches: Vec<models::IssueMatchesForJql>,
}

impl IssueMatches {
    /// A list of matched issues or errors for each JQL query, in the order the JQL queries were passed.
    pub fn new(matches: Vec<models::IssueMatchesForJql>) -> IssueMatches {
        IssueMatches {
            matches,
        }
    }
}

