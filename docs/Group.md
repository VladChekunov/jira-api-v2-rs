# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of group. | [optional]
**param_self** | Option<**String**> | The URL for these group details. | [optional][readonly]
**users** | Option<[**models::PagedListUserDetailsApplicationUser**](PagedListUserDetailsApplicationUser.md)> | A paginated list of the users that are members of the group. A maximum of 50 users is returned in the list, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 50 users, use`?expand=users[51:100]`. | [optional][readonly]
**expand** | Option<**String**> | Expand options that include additional group details in the response. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


