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

/// FunctionOperand : An operand that is a function. See [Advanced searching - functions reference](https://confluence.atlassian.com/x/dwiiLQ) for more information about JQL functions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionOperand {
    /// The name of the function.
    #[serde(rename = "function")]
    pub function: String,
    /// The list of function arguments.
    #[serde(rename = "arguments")]
    pub arguments: Vec<String>,
}

impl FunctionOperand {
    /// An operand that is a function. See [Advanced searching - functions reference](https://confluence.atlassian.com/x/dwiiLQ) for more information about JQL functions.
    pub fn new(function: String, arguments: Vec<String>) -> FunctionOperand {
        FunctionOperand {
            function,
            arguments,
        }
    }
}

