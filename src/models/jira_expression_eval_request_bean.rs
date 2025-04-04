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
pub struct JiraExpressionEvalRequestBean {
    /// The Jira expression to evaluate.
    #[serde(rename = "expression")]
    pub expression: String,
    /// The context in which the Jira expression is evaluated.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<models::JiraExpressionEvalContextBean>>,
}

impl JiraExpressionEvalRequestBean {
    pub fn new(expression: String) -> JiraExpressionEvalRequestBean {
        JiraExpressionEvalRequestBean {
            expression,
            context: None,
        }
    }
}

