# NotificationRecipients

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reporter** | Option<**bool**> | Whether the notification should be sent to the issue's reporter. | [optional]
**assignee** | Option<**bool**> | Whether the notification should be sent to the issue's assignees. | [optional]
**watchers** | Option<**bool**> | Whether the notification should be sent to the issue's watchers. | [optional]
**voters** | Option<**bool**> | Whether the notification should be sent to the issue's voters. | [optional]
**users** | Option<[**Vec<models::UserDetails>**](UserDetails.md)> | List of users to receive the notification. | [optional]
**groups** | Option<[**Vec<models::GroupName>**](GroupName.md)> | List of groups to receive the notification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


