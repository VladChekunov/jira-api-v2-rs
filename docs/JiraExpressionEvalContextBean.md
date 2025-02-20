# JiraExpressionEvalContextBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issue** | Option<[**models::IdOrKeyBean**](IdOrKeyBean.md)> | The issue that is available under the `issue` variable when evaluating the expression. | [optional]
**issues** | Option<[**models::JexpIssues**](JexpIssues.md)> | The collection of issues that is available under the `issues` variable when evaluating the expression. | [optional]
**project** | Option<[**models::IdOrKeyBean**](IdOrKeyBean.md)> | The project that is available under the `project` variable when evaluating the expression. | [optional]
**sprint** | Option<**i64**> | The ID of the sprint that is available under the `sprint` variable when evaluating the expression. | [optional]
**board** | Option<**i64**> | The ID of the board that is available under the `board` variable when evaluating the expression. | [optional]
**service_desk** | Option<**i64**> | The ID of the service desk that is available under the `serviceDesk` variable when evaluating the expression. | [optional]
**customer_request** | Option<**i64**> | The ID of the customer request that is available under the `customerRequest` variable when evaluating the expression. This is the same as the ID of the underlying Jira issue, but the customer request context variable will have a different type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


