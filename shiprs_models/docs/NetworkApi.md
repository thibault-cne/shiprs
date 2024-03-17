# \NetworkApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**network_connect**](NetworkApi.md#network_connect) | **POST** /networks/{id}/connect | Connect a container to a network
[**network_create**](NetworkApi.md#network_create) | **POST** /networks/create | Create a network
[**network_delete**](NetworkApi.md#network_delete) | **DELETE** /networks/{id} | Remove a network
[**network_disconnect**](NetworkApi.md#network_disconnect) | **POST** /networks/{id}/disconnect | Disconnect a container from a network
[**network_inspect**](NetworkApi.md#network_inspect) | **GET** /networks/{id} | Inspect a network
[**network_list**](NetworkApi.md#network_list) | **GET** /networks | List networks
[**network_prune**](NetworkApi.md#network_prune) | **POST** /networks/prune | Delete unused networks



## network_connect

> network_connect(id, container)
Connect a container to a network

The network must be either a local-scoped network or a swarm-scoped network with the `attachable` option set. A network cannot be re-attached to a running container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Network ID or name | [required] |
**container** | [**NetworkConnectRequest**](NetworkConnectRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_create

> models::NetworkCreateResponse network_create(network_config)
Create a network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_config** | [**NetworkCreateRequest**](NetworkCreateRequest.md) | Network configuration | [required] |

### Return type

[**models::NetworkCreateResponse**](NetworkCreateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_delete

> network_delete(id)
Remove a network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Network ID or name | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_disconnect

> network_disconnect(id, container)
Disconnect a container from a network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Network ID or name | [required] |
**container** | [**NetworkDisconnectRequest**](NetworkDisconnectRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_inspect

> models::Network network_inspect(id, verbose, scope)
Inspect a network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Network ID or name | [required] |
**verbose** | Option<**bool**> | Detailed inspect output for troubleshooting |  |[default to false]
**scope** | Option<**String**> | Filter the network by scope (swarm, global, or local) |  |

### Return type

[**models::Network**](Network.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_list

> Vec<models::Network> network_list(filters)
List networks

Returns a list of networks. For details on the format, see the [network inspect endpoint](#operation/NetworkInspect).  Note that it uses a different, smaller representation of a network than inspecting a single network. For example, the list of containers attached to the network is not propagated in API versions 1.28 and up. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of the filters (a `map[string][]string`) to process on the networks list.  Available filters:  - `dangling=<boolean>` When set to `true` (or `1`), returns all    networks that are not in use by a container. When set to `false`    (or `0`), only networks that are in use by one or more    containers are returned. - `driver=<driver-name>` Matches a network's driver. - `id=<network-id>` Matches all or part of a network ID. - `label=<key>` or `label=<key>=<value>` of a network label. - `name=<network-name>` Matches all or part of a network name. - `scope=[\"swarm\"|\"global\"|\"local\"]` Filters networks by scope (`swarm`, `global`, or `local`). - `type=[\"custom\"|\"builtin\"]` Filters networks by type. The `custom` keyword returns all user-defined networks.  |  |

### Return type

[**Vec<models::Network>**](Network.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_prune

> models::NetworkPruneResponse network_prune(filters)
Delete unused networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | Filters to process on the prune list, encoded as JSON (a `map[string][]string`).  Available filters: - `until=<timestamp>` Prune networks created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time. - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune networks with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**models::NetworkPruneResponse**](NetworkPruneResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

