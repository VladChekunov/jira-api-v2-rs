# PageOfDashboards

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_at** | Option<**i32**> | The index of the first item returned on the page. | [optional][readonly]
**max_results** | Option<**i32**> | The maximum number of results that could be on the page. | [optional][readonly]
**total** | Option<**i32**> | The number of results on the page. | [optional][readonly]
**prev** | Option<**String**> | The URL of the previous page of results, if any. | [optional][readonly]
**next** | Option<**String**> | The URL of the next page of results, if any. | [optional][readonly]
**dashboards** | Option<[**Vec<models::Dashboard>**](Dashboard.md)> | List of dashboards. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


