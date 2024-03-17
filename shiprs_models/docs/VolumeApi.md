# \VolumeApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**volume_create**](VolumeApi.md#volume_create) | **POST** /volumes/create | Create a volume
[**volume_delete**](VolumeApi.md#volume_delete) | **DELETE** /volumes/{name} | Remove a volume
[**volume_inspect**](VolumeApi.md#volume_inspect) | **GET** /volumes/{name} | Inspect a volume
[**volume_list**](VolumeApi.md#volume_list) | **GET** /volumes | List volumes
[**volume_prune**](VolumeApi.md#volume_prune) | **POST** /volumes/prune | Delete unused volumes
[**volume_update**](VolumeApi.md#volume_update) | **PUT** /volumes/{name} | \"Update a volume. Valid only for Swarm cluster volumes\" 



## volume_create

> models::Volume volume_create(volume_config)
Create a volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_config** | [**VolumeCreateOptions**](VolumeCreateOptions.md) | Volume configuration | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_delete

> volume_delete(name, force)
Remove a volume

Instruct the driver to remove the volume.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Volume name or ID | [required] |
**force** | Option<**bool**> | Force the removal of the volume |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_inspect

> models::Volume volume_inspect(name)
Inspect a volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Volume name or ID | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_list

> models::VolumeListResponse volume_list(filters)
List volumes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of the filters (a `map[string][]string`) to process on the volumes list. Available filters:  - `dangling=<boolean>` When set to `true` (or `1`), returns all    volumes that are not in use by a container. When set to `false`    (or `0`), only volumes that are in use by one or more    containers are returned. - `driver=<volume-driver-name>` Matches volumes based on their driver. - `label=<key>` or `label=<key>:<value>` Matches volumes based on    the presence of a `label` alone or a `label` and a value. - `name=<volume-name>` Matches all or part of a volume name.  |  |

### Return type

[**models::VolumeListResponse**](VolumeListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_prune

> models::VolumePruneResponse volume_prune(filters)
Delete unused volumes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | Filters to process on the prune list, encoded as JSON (a `map[string][]string`).  Available filters: - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune volumes with (or without, in case `label!=...` is used) the specified labels. - `all` (`all=true`) - Consider all (local) volumes for pruning and not just anonymous volumes.  |  |

### Return type

[**models::VolumePruneResponse**](VolumePruneResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_update

> volume_update(name, version, body)
\"Update a volume. Valid only for Swarm cluster volumes\" 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name or ID of the volume | [required] |
**version** | **i64** | The version number of the volume being updated. This is required to avoid conflicting writes. Found in the volume's `ClusterVolume` field.  | [required] |
**body** | Option<[**VolumeUpdateRequest**](VolumeUpdateRequest.md)> | The spec of the volume to update. Currently, only Availability may change. All other fields must remain unchanged.  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

