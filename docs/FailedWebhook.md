# FailedWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The webhook ID, as sent in the `X-Atlassian-Webhook-Identifier` header with the webhook. | 
**body** | Option<**String**> | The webhook body. | [optional]
**url** | **String** | The original webhook destination. | 
**failure_time** | **i64** | The time the webhook was added to the list of failed webhooks (that is, the time of the last failed retry). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


