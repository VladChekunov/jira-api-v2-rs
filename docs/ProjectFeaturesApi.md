# \ProjectFeaturesApi

All URIs are relative to *https://your-domain.atlassian.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_features_for_project**](ProjectFeaturesApi.md#get_features_for_project) | **GET** /rest/api/2/project/{projectIdOrKey}/features | Get features for project
[**toggle_feature_for_project**](ProjectFeaturesApi.md#toggle_feature_for_project) | **PUT** /rest/api/2/project/{projectIdOrKey}/features/{featureKey} | Toggle feature for project



## get_features_for_project

> models::ProjectFeaturesResponse get_features_for_project(project_id_or_key)
Get features for project

Returns the list of features for a project. The project must be a [company-managed](https://support.atlassian.com/jira-service-management-cloud/docs/learn-the-differences-between-classic-and-next-gen-projects/) project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The ID or (case-sensitive) key of the project. | [required] |

### Return type

[**models::ProjectFeaturesResponse**](ProjectFeaturesResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_feature_for_project

> models::ProjectFeaturesResponse toggle_feature_for_project(project_id_or_key, feature_key, project_feature_toggle_request)
Toggle feature for project

Changes the state of a feature to ENABLED or DISABLED for the project. The project must be a [company-managed](https://support.atlassian.com/jira-service-management-cloud/docs/learn-the-differences-between-classic-and-next-gen-projects/) project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The ID or (case-sensitive) key of the project. | [required] |
**feature_key** | **String** | The key of the feature to change the state. | [required] |
**project_feature_toggle_request** | [**ProjectFeatureToggleRequest**](ProjectFeatureToggleRequest.md) | The request object describing whether we should enable or disable the feature. | [required] |

### Return type

[**models::ProjectFeaturesResponse**](ProjectFeaturesResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

