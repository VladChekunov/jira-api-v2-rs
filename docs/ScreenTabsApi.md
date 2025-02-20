# \ScreenTabsApi

All URIs are relative to *https://your-domain.atlassian.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_screen_tab**](ScreenTabsApi.md#add_screen_tab) | **POST** /rest/api/2/screens/{screenId}/tabs | Create screen tab
[**delete_screen_tab**](ScreenTabsApi.md#delete_screen_tab) | **DELETE** /rest/api/2/screens/{screenId}/tabs/{tabId} | Delete screen tab
[**get_all_screen_tabs**](ScreenTabsApi.md#get_all_screen_tabs) | **GET** /rest/api/2/screens/{screenId}/tabs | Get all screen tabs
[**move_screen_tab**](ScreenTabsApi.md#move_screen_tab) | **POST** /rest/api/2/screens/{screenId}/tabs/{tabId}/move/{pos} | Move screen tab
[**rename_screen_tab**](ScreenTabsApi.md#rename_screen_tab) | **PUT** /rest/api/2/screens/{screenId}/tabs/{tabId} | Update screen tab



## add_screen_tab

> models::ScreenableTab add_screen_tab(screen_id, screenable_tab)
Create screen tab

Creates a tab for a screen.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**screenable_tab** | [**ScreenableTab**](ScreenableTab.md) |  | [required] |

### Return type

[**models::ScreenableTab**](ScreenableTab.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_screen_tab

> delete_screen_tab(screen_id, tab_id)
Delete screen tab

Deletes a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_screen_tabs

> Vec<models::ScreenableTab> get_all_screen_tabs(screen_id, project_key)
Get all screen tabs

Returns the list of tabs for a screen.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) when the project key is specified, providing that the screen is associated with the project through a Screen Scheme and Issue Type Screen Scheme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**project_key** | Option<**String**> | The key of the project. |  |

### Return type

[**Vec<models::ScreenableTab>**](ScreenableTab.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_screen_tab

> serde_json::Value move_screen_tab(screen_id, tab_id, pos)
Move screen tab

Moves a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**pos** | **i32** | The position of tab. The base index is 0. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rename_screen_tab

> models::ScreenableTab rename_screen_tab(screen_id, tab_id, screenable_tab)
Update screen tab

Updates the name of a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**screenable_tab** | [**ScreenableTab**](ScreenableTab.md) |  | [required] |

### Return type

[**models::ScreenableTab**](ScreenableTab.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

