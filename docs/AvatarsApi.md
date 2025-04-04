# \AvatarsApi

All URIs are relative to *https://your-domain.atlassian.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_avatar**](AvatarsApi.md#delete_avatar) | **DELETE** /rest/api/2/universal_avatar/type/{type}/owner/{owningObjectId}/avatar/{id} | Delete avatar
[**get_all_system_avatars**](AvatarsApi.md#get_all_system_avatars) | **GET** /rest/api/2/avatar/{type}/system | Get system avatars by type
[**get_avatars**](AvatarsApi.md#get_avatars) | **GET** /rest/api/2/universal_avatar/type/{type}/owner/{entityId} | Get avatars
[**store_avatar**](AvatarsApi.md#store_avatar) | **POST** /rest/api/2/universal_avatar/type/{type}/owner/{entityId} | Load avatar



## delete_avatar

> delete_avatar(r#type, owning_object_id, id)
Delete avatar

Deletes an avatar from a project or issue type.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The avatar type. | [required] |
**owning_object_id** | **String** | The ID of the item the avatar is associated with. | [required] |
**id** | **i64** | The ID of the avatar. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_system_avatars

> models::SystemAvatars get_all_system_avatars(r#type)
Get system avatars by type

Returns a list of system avatar details by owner type, where the owner types are issue type, project, or user.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The avatar type. | [required] |

### Return type

[**models::SystemAvatars**](SystemAvatars.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_avatars

> models::Avatars get_avatars(r#type, entity_id)
Get avatars

Returns the system and custom avatars for a project or issue type.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  for custom project avatars, *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project the avatar belongs to.  *  for custom issue type avatars, *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for at least one project the issue type is used in.  *  for system avatars, none.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The avatar type. | [required] |
**entity_id** | **String** | The ID of the item the avatar is associated with. | [required] |

### Return type

[**models::Avatars**](Avatars.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_avatar

> models::Avatar store_avatar(r#type, entity_id, size, body, x, y)
Load avatar

Loads a custom avatar for a project or issue type.  Specify the avatar's local file location in the body of the request. Also, include the following headers:   *  `X-Atlassian-Token: no-check` To prevent XSRF protection blocking the request, for more information see [Special Headers](#special-request-headers).  *  `Content-Type: image/image type` Valid image types are JPEG, GIF, or PNG.  For example:   `curl --request POST `  `--user email@example.com:<api_token> `  `--header 'X-Atlassian-Token: no-check' `  `--header 'Content-Type: image/< image_type>' `  `--data-binary \"<@/path/to/file/with/your/avatar>\" `  `--url 'https://your-domain.atlassian.net/rest/api/2/universal_avatar/type/{type}/owner/{entityId}'`  The avatar is cropped to a square. If no crop parameters are specified, the square originates at the top left of the image. The length of the square's sides is set to the smaller of the height or width of the image.  The cropped image is then used to create avatars of 16x16, 24x24, 32x32, and 48x48 in size.  After creating the avatar use:   *  [Update issue type](#api-rest-api-2-issuetype-id-put) to set it as the issue type's displayed avatar.  *  [Set project avatar](#api-rest-api-2-project-projectIdOrKey-avatar-put) to set it as the project's displayed avatar.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The avatar type. | [required] |
**entity_id** | **String** | The ID of the item the avatar is associated with. | [required] |
**size** | **i32** | The length of each side of the crop region. | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |
**x** | Option<**i32**> | The X coordinate of the top-left corner of the crop region. |  |[default to 0]
**y** | Option<**i32**> | The Y coordinate of the top-left corner of the crop region. |  |[default to 0]

### Return type

[**models::Avatar**](Avatar.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

