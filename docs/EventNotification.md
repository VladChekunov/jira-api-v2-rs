# EventNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expand** | Option<**String**> | Expand options that include additional event notification details in the response. | [optional]
**id** | Option<**i64**> | The ID of the notification. | [optional]
**notification_type** | Option<**String**> | Identifies the recipients of the notification. | [optional]
**parameter** | Option<**String**> | The value of the `notificationType`:   *  `User` The `parameter` is the user account ID.  *  `Group` The `parameter` is the group name.  *  `ProjectRole` The `parameter` is the project role ID.  *  `UserCustomField` The `parameter` is the ID of the custom field.  *  `GroupCustomField` The `parameter` is the ID of the custom field. | [optional]
**group** | Option<[**models::GroupName**](GroupName.md)> | The specified group. | [optional]
**field** | Option<[**models::FieldDetails**](FieldDetails.md)> | The custom user or group field. | [optional]
**email_address** | Option<**String**> | The email address. | [optional]
**project_role** | Option<[**models::ProjectRole**](ProjectRole.md)> | The specified project role. | [optional]
**user** | Option<[**models::UserDetails**](UserDetails.md)> | The specified user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


