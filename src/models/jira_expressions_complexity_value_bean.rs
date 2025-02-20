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
pub struct JiraExpressionsComplexityValueBean {
    /// The complexity value of the current expression.
    #[serde(rename = "value")]
    pub value: i32,
    /// The maximum allowed complexity. The evaluation will fail if this value is exceeded.
    #[serde(rename = "limit")]
    pub limit: i32,
}

impl JiraExpressionsComplexityValueBean {
    pub fn new(value: i32, limit: i32) -> JiraExpressionsComplexityValueBean {
        JiraExpressionsComplexityValueBean {
            value,
            limit,
        }
    }
}

