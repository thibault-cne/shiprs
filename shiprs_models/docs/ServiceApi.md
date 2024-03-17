# \ServiceApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**service_create**](ServiceApi.md#service_create) | **POST** /services/create | Create a service
[**service_delete**](ServiceApi.md#service_delete) | **DELETE** /services/{id} | Delete a service
[**service_inspect**](ServiceApi.md#service_inspect) | **GET** /services/{id} | Inspect a service
[**service_list**](ServiceApi.md#service_list) | **GET** /services | List services
[**service_logs**](ServiceApi.md#service_logs) | **GET** /services/{id}/logs | Get service logs
[**service_update**](ServiceApi.md#service_update) | **POST** /services/{id}/update | Update a service



## service_create

> models::ServiceCreateResponse service_create(body, x_registry_auth)
Create a service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ServiceCreateRequest**](ServiceCreateRequest.md) |  | [required] |
**x_registry_auth** | Option<**String**> | A base64url-encoded auth configuration for pulling from private registries.  Refer to the [authentication section](#section/Authentication) for details.  |  |

### Return type

[**models::ServiceCreateResponse**](ServiceCreateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_delete

> service_delete(id)
Delete a service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of service. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_inspect

> models::Service service_inspect(id, insert_defaults)
Inspect a service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of service. | [required] |
**insert_defaults** | Option<**bool**> | Fill empty fields with default values. |  |[default to false]

### Return type

[**models::Service**](Service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_list

> Vec<models::Service> service_list(filters, status)
List services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the services list.  Available filters:  - `id=<service id>` - `label=<service label>` - `mode=[\"replicated\"|\"global\"]` - `name=<service name>`  |  |
**status** | Option<**bool**> | Include service status, with count of running and desired tasks.  |  |

### Return type

[**Vec<models::Service>**](Service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_logs

> std::path::PathBuf service_logs(id, details, follow, stdout, stderr, since, timestamps, tail)
Get service logs

Get `stdout` and `stderr` logs from a service. See also [`/containers/{id}/logs`](#operation/ContainerLogs).  **Note**: This endpoint works only for services with the `local`, `json-file` or `journald` logging drivers. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the service | [required] |
**details** | Option<**bool**> | Show service context and extra details provided to logs. |  |[default to false]
**follow** | Option<**bool**> | Keep connection after returning logs. |  |[default to false]
**stdout** | Option<**bool**> | Return logs from `stdout` |  |[default to false]
**stderr** | Option<**bool**> | Return logs from `stderr` |  |[default to false]
**since** | Option<**i32**> | Only return logs since this time, as a UNIX timestamp |  |[default to 0]
**timestamps** | Option<**bool**> | Add timestamps to every log line |  |[default to false]
**tail** | Option<**String**> | Only return this number of log lines from the end of the logs. Specify as an integer or `all` to output all log lines.  |  |[default to all]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.docker.raw-stream, application/vnd.docker.multiplexed-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_update

> models::ServiceUpdateResponse service_update(id, version, body, registry_auth_from, rollback, x_registry_auth)
Update a service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of service. | [required] |
**version** | **i32** | The version number of the service object being updated. This is required to avoid conflicting writes. This version number should be the value as currently set on the service *before* the update. You can find the current version by calling `GET /services/{id}`  | [required] |
**body** | [**ServiceUpdateRequest**](ServiceUpdateRequest.md) |  | [required] |
**registry_auth_from** | Option<**String**> | If the `X-Registry-Auth` header is not specified, this parameter indicates where to find registry authorization credentials.  |  |[default to spec]
**rollback** | Option<**String**> | Set to this parameter to `previous` to cause a server-side rollback to the previous service spec. The supplied spec will be ignored in this case.  |  |
**x_registry_auth** | Option<**String**> | A base64url-encoded auth configuration for pulling from private registries.  Refer to the [authentication section](#section/Authentication) for details.  |  |

### Return type

[**models::ServiceUpdateResponse**](ServiceUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

