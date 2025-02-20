# IssueTypeSchemeDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the issue type scheme. The name must be unique. The maximum length is 255 characters. | 
**description** | Option<**String**> | The description of the issue type scheme. The maximum length is 4000 characters. | [optional]
**default_issue_type_id** | Option<**String**> | The ID of the default issue type of the issue type scheme. This ID must be included in `issueTypeIds`. | [optional]
**issue_type_ids** | **Vec<String>** | The list of issue types IDs of the issue type scheme. At least one standard issue type ID is required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


