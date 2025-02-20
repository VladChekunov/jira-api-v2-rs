# Field

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the field. | 
**name** | **String** | The name of the field. | 
**schema** | [**models::JsonTypeBean**](JsonTypeBean.md) |  | 
**description** | Option<**String**> | The description of the field. | [optional]
**key** | Option<**String**> | The key of the field. | [optional]
**is_locked** | Option<**bool**> | Whether the field is locked. | [optional]
**searcher_key** | Option<**String**> | The searcher key of the field. Returned for custom fields. | [optional]
**screens_count** | Option<**i64**> | Number of screens where the field is used. | [optional]
**contexts_count** | Option<**i64**> | Number of contexts where the field is used. | [optional]
**last_used** | Option<[**models::FieldLastUsed**](FieldLastUsed.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


