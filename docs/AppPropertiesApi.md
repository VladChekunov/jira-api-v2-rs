# \AppPropertiesApi

All URIs are relative to *https://your-domain.atlassian.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addon_properties_resource_period_delete_addon_property_delete**](AppPropertiesApi.md#addon_properties_resource_period_delete_addon_property_delete) | **DELETE** /rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey} | Delete app property
[**addon_properties_resource_period_get_addon_properties_get**](AppPropertiesApi.md#addon_properties_resource_period_get_addon_properties_get) | **GET** /rest/atlassian-connect/1/addons/{addonKey}/properties | Get app properties
[**addon_properties_resource_period_get_addon_property_get**](AppPropertiesApi.md#addon_properties_resource_period_get_addon_property_get) | **GET** /rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey} | Get app property
[**addon_properties_resource_period_put_addon_property_put**](AppPropertiesApi.md#addon_properties_resource_period_put_addon_property_put) | **PUT** /rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey} | Set app property



## addon_properties_resource_period_delete_addon_property_delete

> addon_properties_resource_period_delete_addon_property_delete(addon_key, property_key)
Delete app property

Deletes an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |
**property_key** | **String** | The key of the property. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_properties_resource_period_get_addon_properties_get

> models::PropertyKeys addon_properties_resource_period_get_addon_properties_get(addon_key)
Get app properties

Gets all the properties of an app.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |

### Return type

[**models::PropertyKeys**](PropertyKeys.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_properties_resource_period_get_addon_property_get

> models::EntityProperty addon_properties_resource_period_get_addon_property_get(addon_key, property_key)
Get app property

Returns the key and value of an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |
**property_key** | **String** | The key of the property. | [required] |

### Return type

[**models::EntityProperty**](EntityProperty.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_properties_resource_period_put_addon_property_put

> models::OperationMessage addon_properties_resource_period_put_addon_property_put(addon_key, property_key, body)
Set app property

Sets the value of an app's property. Use this resource to store custom data for your app.  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |
**property_key** | **String** | The key of the property. | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::OperationMessage**](OperationMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

