# \ConfigApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**config_create**](ConfigApi.md#config_create) | **POST** /configs/create | Create a config
[**config_delete**](ConfigApi.md#config_delete) | **DELETE** /configs/{id} | Delete a config
[**config_inspect**](ConfigApi.md#config_inspect) | **GET** /configs/{id} | Inspect a config
[**config_list**](ConfigApi.md#config_list) | **GET** /configs | List configs
[**config_update**](ConfigApi.md#config_update) | **POST** /configs/{id}/update | Update a Config



## config_create

> models::IdResponse config_create(body)
Create a config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ConfigCreateRequest**](ConfigCreateRequest.md)> |  |  |

### Return type

[**models::IdResponse**](IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_delete

> config_delete(id)
Delete a config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the config | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_inspect

> models::Config config_inspect(id)
Inspect a config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the config | [required] |

### Return type

[**models::Config**](Config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_list

> Vec<models::Config> config_list(filters)
List configs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the configs list.  Available filters:  - `id=<config id>` - `label=<key> or label=<key>=value` - `name=<config name>` - `names=<config name>`  |  |

### Return type

[**Vec<models::Config>**](Config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_update

> config_update(id, version, body)
Update a Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID or name of the config | [required] |
**version** | **i64** | The version number of the config object being updated. This is required to avoid conflicting writes.  | [required] |
**body** | Option<[**ConfigSpec**](ConfigSpec.md)> | The spec of the config to update. Currently, only the Labels field can be updated. All other fields must remain unchanged from the [ConfigInspect endpoint](#operation/ConfigInspect) response values.  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

