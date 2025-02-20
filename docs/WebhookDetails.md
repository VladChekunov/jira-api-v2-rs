# WebhookDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**jql_filter** | **String** | The JQL filter that specifies which issues the webhook is sent for. Only a subset of JQL can be used. The supported elements are:   *  Fields: `issueKey`, `project`, `issuetype`, `status`, `assignee`, `reporter`, `issue.property`, and `cf[id]` (for custom fields—only the epic label custom field is supported).  *  Operators: `=`, `!=`, `IN`, and `NOT IN`. | 
**events** | **Vec<String>** | The Jira events that trigger the webhook. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


