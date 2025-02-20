# Filter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**param_self** | Option<**String**> | The URL of the filter. | [optional][readonly]
**id** | Option<**String**> | The unique identifier for the filter. | [optional][readonly]
**name** | **String** | The name of the filter. Must be unique. | 
**description** | Option<**String**> | A description of the filter. | [optional]
**owner** | Option<[**models::User**](User.md)> | The user who owns the filter. This is defaulted to the creator of the filter, however Jira administrators can change the owner of a shared filter in the admin settings. | [optional][readonly]
**jql** | Option<**String**> | The JQL query for the filter. For example, *project = SSP AND issuetype = Bug*. | [optional]
**view_url** | Option<**String**> | A URL to view the filter results in Jira, using the ID of the filter. For example, *https://your-domain.atlassian.net/issues/?filter=10100*. | [optional][readonly]
**search_url** | Option<**String**> | A URL to view the filter results in Jira, using the [Search for issues using JQL](#api-rest-api-2-filter-search-get) operation with the filter's JQL string to return the filter results. For example, *https://your-domain.atlassian.net/rest/api/2/search?jql=project+%3D+SSP+AND+issuetype+%3D+Bug*. | [optional][readonly]
**favourite** | Option<**bool**> | Whether the filter is selected as a favorite. | [optional]
**favourited_count** | Option<**i64**> | The count of how many users have selected this filter as a favorite, including the filter owner. | [optional][readonly]
**share_permissions** | Option<[**Vec<models::SharePermission>**](SharePermission.md)> | The groups and projects that the filter is shared with. | [optional]
**shared_users** | Option<[**models::UserList**](UserList.md)> | A paginated list of the users that the filter is shared with. This includes users that are members of the groups or can browse the projects that the filter is shared with. | [optional][readonly]
**subscriptions** | Option<[**models::FilterSubscriptionsList**](FilterSubscriptionsList.md)> | A paginated list of the users that are subscribed to the filter. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


