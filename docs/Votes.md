# Votes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**param_self** | Option<**String**> | The URL of these issue vote details. | [optional][readonly]
**votes** | Option<**i64**> | The number of votes on the issue. | [optional][readonly]
**has_voted** | Option<**bool**> | Whether the user making this request has voted on the issue. | [optional][readonly]
**voters** | Option<[**Vec<models::User>**](User.md)> | List of the users who have voted on this issue. An empty list is returned when the calling user doesn't have the *View voters and watchers* project permission. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


