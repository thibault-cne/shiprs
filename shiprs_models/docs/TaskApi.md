# \TaskApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**task_inspect**](TaskApi.md#task_inspect) | **GET** /tasks/{id} | Inspect a task
[**task_list**](TaskApi.md#task_list) | **GET** /tasks | List tasks
[**task_logs**](TaskApi.md#task_logs) | **GET** /tasks/{id}/logs | Get task logs



## task_inspect

> models::Task task_inspect(id)
Inspect a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the task | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## task_list

> Vec<models::Task> task_list(filters)
List tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the tasks list.  Available filters:  - `desired-state=(running | shutdown | accepted)` - `id=<task id>` - `label=key` or `label=\"key=value\"` - `name=<task name>` - `node=<node id or name>` - `service=<service name>`  |  |

### Return type

[**Vec<models::Task>**](Task.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## task_logs

> std::path::PathBuf task_logs(id, details, follow, stdout, stderr, since, timestamps, tail)
Get task logs

Get `stdout` and `stderr` logs from a task. See also [`/containers/{id}/logs`](#operation/ContainerLogs).  **Note**: This endpoint works only for services with the `local`, `json-file` or `journald` logging drivers. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the task | [required] |
**details** | Option<**bool**> | Show task context and extra details provided to logs. |  |[default to false]
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

