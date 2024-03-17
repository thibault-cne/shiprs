# \ExecApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**container_exec**](ExecApi.md#container_exec) | **POST** /containers/{id}/exec | Create an exec instance
[**exec_inspect**](ExecApi.md#exec_inspect) | **GET** /exec/{id}/json | Inspect an exec instance
[**exec_resize**](ExecApi.md#exec_resize) | **POST** /exec/{id}/resize | Resize an exec instance
[**exec_start**](ExecApi.md#exec_start) | **POST** /exec/{id}/start | Start an exec instance



## container_exec

> models::IdResponse container_exec(id, exec_config)
Create an exec instance

Run a command inside a running container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of container | [required] |
**exec_config** | [**ExecConfig**](ExecConfig.md) | Exec configuration | [required] |

### Return type

[**models::IdResponse**](IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_inspect

> models::ExecInspectResponse exec_inspect(id)
Inspect an exec instance

Return low-level information about an exec instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Exec instance ID | [required] |

### Return type

[**models::ExecInspectResponse**](ExecInspectResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_resize

> exec_resize(id, h, w)
Resize an exec instance

Resize the TTY session used by an exec instance. This endpoint only works if `tty` was specified as part of creating and starting the exec instance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Exec instance ID | [required] |
**h** | Option<**i32**> | Height of the TTY session in characters |  |
**w** | Option<**i32**> | Width of the TTY session in characters |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_start

> exec_start(id, exec_start_config)
Start an exec instance

Starts a previously set up exec instance. If detach is true, this endpoint returns immediately after starting the command. Otherwise, it sets up an interactive session with the command. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Exec instance ID | [required] |
**exec_start_config** | Option<[**ExecStartConfig**](ExecStartConfig.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/vnd.docker.raw-stream, application/vnd.docker.multiplexed-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

