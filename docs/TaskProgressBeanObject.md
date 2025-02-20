# TaskProgressBeanObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**param_self** | **String** | The URL of the task. | 
**id** | **String** | The ID of the task. | 
**description** | Option<**String**> | The description of the task. | [optional]
**status** | **String** | The status of the task. | 
**message** | Option<**String**> | Information about the progress of the task. | [optional]
**result** | Option<[**serde_json::Value**](.md)> | The result of the task execution. | [optional]
**submitted_by** | **i64** | The ID of the user who submitted the task. | 
**progress** | **i64** | The progress of the task, as a percentage complete. | 
**elapsed_runtime** | **i64** | The execution time of the task, in milliseconds. | 
**submitted** | **i64** | A timestamp recording when the task was submitted. | 
**started** | Option<**i64**> | A timestamp recording when the task was started. | [optional]
**finished** | Option<**i64**> | A timestamp recording when the task was finished. | [optional]
**last_update** | **i64** | A timestamp recording when the task progress was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


