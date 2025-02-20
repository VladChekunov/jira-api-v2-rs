# Workflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**models::PublishedWorkflowId**](PublishedWorkflowId.md) |  | 
**description** | **String** | The description of the workflow. | 
**transitions** | Option<[**Vec<models::Transition>**](Transition.md)> | The transitions of the workflow. | [optional]
**statuses** | Option<[**Vec<models::WorkflowStatus>**](WorkflowStatus.md)> | The statuses of the workflow. | [optional]
**is_default** | Option<**bool**> | Whether this is the default workflow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


