# \NodeApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**node_delete**](NodeApi.md#node_delete) | **DELETE** /nodes/{id} | Delete a node
[**node_inspect**](NodeApi.md#node_inspect) | **GET** /nodes/{id} | Inspect a node
[**node_list**](NodeApi.md#node_list) | **GET** /nodes | List nodes
[**node_update**](NodeApi.md#node_update) | **POST** /nodes/{id}/update | Update a node



## node_delete

> node_delete(id, force)
Delete a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID or name of the node | [required] |
**force** | Option<**bool**> | Force remove a node from the swarm |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## node_inspect

> models::Node node_inspect(id)
Inspect a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID or name of the node | [required] |

### Return type

[**models::Node**](Node.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## node_list

> Vec<models::Node> node_list(filters)
List nodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | Filters to process on the nodes list, encoded as JSON (a `map[string][]string`).  Available filters: - `id=<node id>` - `label=<engine label>` - `membership=`(`accepted`|`pending`)` - `name=<node name>` - `node.label=<node label>` - `role=`(`manager`|`worker`)`  |  |

### Return type

[**Vec<models::Node>**](Node.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## node_update

> node_update(id, version, body)
Update a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the node | [required] |
**version** | **i64** | The version number of the node object being updated. This is required to avoid conflicting writes.  | [required] |
**body** | Option<[**NodeSpec**](NodeSpec.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

