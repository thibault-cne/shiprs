# \SystemApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**system_auth**](SystemApi.md#system_auth) | **POST** /auth | Check auth configuration
[**system_data_usage**](SystemApi.md#system_data_usage) | **GET** /system/df | Get data usage information
[**system_events**](SystemApi.md#system_events) | **GET** /events | Monitor events
[**system_info**](SystemApi.md#system_info) | **GET** /info | Get system information
[**system_ping**](SystemApi.md#system_ping) | **GET** /_ping | Ping
[**system_ping_head**](SystemApi.md#system_ping_head) | **HEAD** /_ping | Ping
[**system_version**](SystemApi.md#system_version) | **GET** /version | Get version



## system_auth

> models::SystemAuthResponse system_auth(auth_config)
Check auth configuration

Validate credentials for a registry and, if available, get an identity token for accessing the registry without password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_config** | Option<[**AuthConfig**](AuthConfig.md)> | Authentication to check |  |

### Return type

[**models::SystemAuthResponse**](SystemAuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_data_usage

> models::SystemDataUsageResponse system_data_usage(r#type)
Get data usage information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<[**Vec<String>**](String.md)> | Object types, for which to compute and return data.  |  |

### Return type

[**models::SystemDataUsageResponse**](SystemDataUsageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_events

> models::EventMessage system_events(since, until, filters)
Monitor events

Stream real-time events from the server.  Various objects within Docker report events when something happens to them.  Containers report these events: `attach`, `commit`, `copy`, `create`, `destroy`, `detach`, `die`, `exec_create`, `exec_detach`, `exec_start`, `exec_die`, `export`, `health_status`, `kill`, `oom`, `pause`, `rename`, `resize`, `restart`, `start`, `stop`, `top`, `unpause`, `update`, and `prune`  Images report these events: `delete`, `import`, `load`, `pull`, `push`, `save`, `tag`, `untag`, and `prune`  Volumes report these events: `create`, `mount`, `unmount`, `destroy`, and `prune`  Networks report these events: `create`, `connect`, `disconnect`, `destroy`, `update`, `remove`, and `prune`  The Docker daemon reports these events: `reload`  Services report these events: `create`, `update`, and `remove`  Nodes report these events: `create`, `update`, and `remove`  Secrets report these events: `create`, `update`, and `remove`  Configs report these events: `create`, `update`, and `remove`  The Builder reports `prune` events 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**String**> | Show events created since this timestamp then stream new events. |  |
**until** | Option<**String**> | Show events created until this timestamp then stop streaming. |  |
**filters** | Option<**String**> | A JSON encoded value of filters (a `map[string][]string`) to process on the event list. Available filters:  - `config=<string>` config name or ID - `container=<string>` container name or ID - `daemon=<string>` daemon name or ID - `event=<string>` event type - `image=<string>` image name or ID - `label=<string>` image or container label - `network=<string>` network name or ID - `node=<string>` node ID - `plugin`=<string> plugin name or ID - `scope`=<string> local or swarm - `secret=<string>` secret name or ID - `service=<string>` service name or ID - `type=<string>` object to filter by, one of `container`, `image`, `volume`, `network`, `daemon`, `plugin`, `node`, `service`, `secret` or `config` - `volume=<string>` volume name  |  |

### Return type

[**models::EventMessage**](EventMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_info

> models::SystemInfo system_info()
Get system information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemInfo**](SystemInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_ping

> String system_ping()
Ping

This is a dummy endpoint you can use to test if the server is accessible.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_ping_head

> String system_ping_head()
Ping

This is a dummy endpoint you can use to test if the server is accessible.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_version

> models::SystemVersion system_version()
Get version

Returns the version of Docker that is running and various information about the system that Docker is running on.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemVersion**](SystemVersion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

