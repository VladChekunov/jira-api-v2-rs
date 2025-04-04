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

/// Project : Details about a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// Expand options that include additional project details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The URL of the project details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The ID of the project.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the project.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// A brief description of the project.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The username of the project lead.
    #[serde(rename = "lead", skip_serializing_if = "Option::is_none")]
    pub lead: Option<Box<models::User>>,
    /// List of the components contained in the project.
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<models::Component>>,
    /// List of the issue types available in the project.
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<models::IssueTypeDetails>>,
    /// A link to information about this project, such as project documentation.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// An email address associated with the project.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The default assignee when creating issues for this project.
    #[serde(rename = "assigneeType", skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<AssigneeType>,
    /// The versions defined in the project. For more information, see [Create version](#api-rest-api-2-version-post).
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<models::Version>>,
    /// The name of the project.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The name and self URL for each role defined in the project. For more information, see [Create project role](#api-rest-api-2-role-post).
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<std::collections::HashMap<String, String>>,
    /// The URLs of the project's avatars.
    #[serde(rename = "avatarUrls", skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<Box<models::AvatarUrlsBean>>,
    /// The category the project belongs to.
    #[serde(rename = "projectCategory", skip_serializing_if = "Option::is_none")]
    pub project_category: Option<Box<models::ProjectCategory>>,
    /// The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes) of the project.
    #[serde(rename = "projectTypeKey", skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<ProjectTypeKey>,
    /// Whether the project is simplified.
    #[serde(rename = "simplified", skip_serializing_if = "Option::is_none")]
    pub simplified: Option<bool>,
    /// The type of the project.
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    /// Whether the project is selected as a favorite.
    #[serde(rename = "favourite", skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
    /// Whether the project is private.
    #[serde(rename = "isPrivate", skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// The issue type hierarchy for the project.
    #[serde(rename = "issueTypeHierarchy", skip_serializing_if = "Option::is_none")]
    pub issue_type_hierarchy: Option<Box<models::Hierarchy>>,
    /// User permissions on the project
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::ProjectPermissions>>,
    /// Map of project properties
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Unique ID for next-gen projects.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    /// Insights about the project.
    #[serde(rename = "insight", skip_serializing_if = "Option::is_none")]
    pub insight: Option<Box<models::ProjectInsight>>,
    /// Whether the project is marked as deleted.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The date when the project is deleted permanently.
    #[serde(rename = "retentionTillDate", skip_serializing_if = "Option::is_none")]
    pub retention_till_date: Option<String>,
    /// The date when the project was marked as deleted.
    #[serde(rename = "deletedDate", skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<String>,
    /// The user who marked the project as deleted.
    #[serde(rename = "deletedBy", skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<Box<models::User>>,
    /// Whether the project is archived.
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// The date when the project was archived.
    #[serde(rename = "archivedDate", skip_serializing_if = "Option::is_none")]
    pub archived_date: Option<String>,
    /// The user who archived the project.
    #[serde(rename = "archivedBy", skip_serializing_if = "Option::is_none")]
    pub archived_by: Option<Box<models::User>>,
}

impl Project {
    /// Details about a project.
    pub fn new() -> Project {
        Project {
            expand: None,
            param_self: None,
            id: None,
            key: None,
            description: None,
            lead: None,
            components: None,
            issue_types: None,
            url: None,
            email: None,
            assignee_type: None,
            versions: None,
            name: None,
            roles: None,
            avatar_urls: None,
            project_category: None,
            project_type_key: None,
            simplified: None,
            style: None,
            favourite: None,
            is_private: None,
            issue_type_hierarchy: None,
            permissions: None,
            properties: None,
            uuid: None,
            insight: None,
            deleted: None,
            retention_till_date: None,
            deleted_date: None,
            deleted_by: None,
            archived: None,
            archived_date: None,
            archived_by: None,
        }
    }
}
/// The default assignee when creating issues for this project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssigneeType {
    #[serde(rename = "PROJECT_LEAD")]
    ProjectLead,
    #[serde(rename = "UNASSIGNED")]
    Unassigned,
}

impl Default for AssigneeType {
    fn default() -> AssigneeType {
        Self::ProjectLead
    }
}
/// The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes) of the project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectTypeKey {
    #[serde(rename = "software")]
    Software,
    #[serde(rename = "service_desk")]
    ServiceDesk,
    #[serde(rename = "business")]
    Business,
}

impl Default for ProjectTypeKey {
    fn default() -> ProjectTypeKey {
        Self::Software
    }
}
/// The type of the project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "next-gen")]
    NextGen,
}

impl Default for Style {
    fn default() -> Style {
        Self::Classic
    }
}

