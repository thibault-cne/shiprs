# \SwarmApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**swarm_init**](SwarmApi.md#swarm_init) | **POST** /swarm/init | Initialize a new swarm
[**swarm_inspect**](SwarmApi.md#swarm_inspect) | **GET** /swarm | Inspect swarm
[**swarm_join**](SwarmApi.md#swarm_join) | **POST** /swarm/join | Join an existing swarm
[**swarm_leave**](SwarmApi.md#swarm_leave) | **POST** /swarm/leave | Leave a swarm
[**swarm_unlock**](SwarmApi.md#swarm_unlock) | **POST** /swarm/unlock | Unlock a locked manager
[**swarm_unlockkey**](SwarmApi.md#swarm_unlockkey) | **GET** /swarm/unlockkey | Get the unlock key
[**swarm_update**](SwarmApi.md#swarm_update) | **POST** /swarm/update | Update a swarm



## swarm_init

> String swarm_init(body)
Initialize a new swarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SwarmInitRequest**](SwarmInitRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swarm_inspect

> models::Swarm swarm_inspect()
Inspect swarm

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Swarm**](Swarm.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swarm_join

> swarm_join(body)
Join an existing swarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SwarmJoinRequest**](SwarmJoinRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swarm_leave

> swarm_leave(force)
Leave a swarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | Option<**bool**> | Force leave swarm, even if this is the last manager or that it will break the cluster.  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swarm_unlock

> swarm_unlock(body)
Unlock a locked manager

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SwarmUnlockRequest**](SwarmUnlockRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swarm_unlockkey

> models::UnlockKeyResponse swarm_unlockkey()
Get the unlock key

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UnlockKeyResponse**](UnlockKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swarm_update

> swarm_update(version, body, rotate_worker_token, rotate_manager_token, rotate_manager_unlock_key)
Update a swarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **i64** | The version number of the swarm object being updated. This is required to avoid conflicting writes.  | [required] |
**body** | [**SwarmSpec**](SwarmSpec.md) |  | [required] |
**rotate_worker_token** | Option<**bool**> | Rotate the worker join token. |  |[default to false]
**rotate_manager_token** | Option<**bool**> | Rotate the manager join token. |  |[default to false]
**rotate_manager_unlock_key** | Option<**bool**> | Rotate the manager unlock key. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

