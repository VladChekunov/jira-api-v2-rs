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
pub struct SharePermissionInputBean {
    /// The type of the share permission.Specify the type as follows:   *  `group` Share with a group. Specify `groupname` as well.  *  `project` Share with a project. Specify `projectId` as well.  *  `projectRole` Share with a project role in a project. Specify `projectId` and `projectRoleId` as well.  *  `global` Share globally, including anonymous users. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set.  *  `authenticated` Share with all logged-in users. This shows as `loggedin` in the response. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The ID of the project to share the filter with. Set `type` to `project`.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The name of the group to share the filter with. Set `type` to `group`.
    #[serde(rename = "groupname", skip_serializing_if = "Option::is_none")]
    pub groupname: Option<String>,
    /// The ID of the project role to share the filter with. Set `type` to `projectRole` and the `projectId` for the project that the role is in.
    #[serde(rename = "projectRoleId", skip_serializing_if = "Option::is_none")]
    pub project_role_id: Option<String>,
}

impl SharePermissionInputBean {
    pub fn new(r#type: Type) -> SharePermissionInputBean {
        SharePermissionInputBean {
            r#type,
            project_id: None,
            groupname: None,
            project_role_id: None,
        }
    }
}
/// The type of the share permission.Specify the type as follows:   *  `group` Share with a group. Specify `groupname` as well.  *  `project` Share with a project. Specify `projectId` as well.  *  `projectRole` Share with a project role in a project. Specify `projectId` and `projectRoleId` as well.  *  `global` Share globally, including anonymous users. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set.  *  `authenticated` Share with all logged-in users. This shows as `loggedin` in the response. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "projectRole")]
    ProjectRole,
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "authenticated")]
    Authenticated,
}

impl Default for Type {
    fn default() -> Type {
        Self::Project
    }
}

