# UserWriteBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**param_self** | Option<**String**> | The URL of the user. | [optional][readonly]
**key** | Option<**String**> | The key for the user. When provided with `name`, overrides the value in `name` to set both `name` and `key`. This property is deprecated because of privacy changes. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. | [optional]
**name** | Option<**String**> | The username for the user. This property is deprecated because of privacy changes. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. | [optional]
**password** | Option<**String**> | A password for the user. If a password is not set, a random password is generated. | [optional]
**email_address** | **String** | The email address for the user. | 
**display_name** | **String** | The display name for the user. | 
**application_keys** | Option<**Vec<String>**> | Deprecated, do not use. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


