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

/// FilterSubscriptionsList : A paginated list of subscriptions to a filter.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterSubscriptionsList {
    /// The number of items on the page.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// The list of items.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::FilterSubscription>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "max-results", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The index of the first item returned on the page.
    #[serde(rename = "start-index", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
    /// The index of the last item returned on the page.
    #[serde(rename = "end-index", skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
}

impl FilterSubscriptionsList {
    /// A paginated list of subscriptions to a filter.
    pub fn new() -> FilterSubscriptionsList {
        FilterSubscriptionsList {
            size: None,
            items: None,
            max_results: None,
            start_index: None,
            end_index: None,
        }
    }
}

