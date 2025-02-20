# ApplicationRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | Option<**String**> | The key of the application role. | [optional]
**groups** | Option<**Vec<String>**> | The groups associated with the application role. | [optional]
**name** | Option<**String**> | The display name of the application role. | [optional]
**default_groups** | Option<**Vec<String>**> | The groups that are granted default access for this application role. | [optional]
**selected_by_default** | Option<**bool**> | Determines whether this application role should be selected by default on user creation. | [optional]
**defined** | Option<**bool**> | Deprecated. | [optional]
**number_of_seats** | Option<**i32**> | The maximum count of users on your license. | [optional]
**remaining_seats** | Option<**i32**> | The count of users remaining on your license. | [optional]
**user_count** | Option<**i32**> | The number of users counting against your license. | [optional]
**user_count_description** | Option<**String**> | The [type of users](https://confluence.atlassian.com/x/lRW3Ng) being counted against your license. | [optional]
**has_unlimited_seats** | Option<**bool**> |  | [optional]
**platform** | Option<**bool**> | Indicates if the application role belongs to Jira platform (`jira-core`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


