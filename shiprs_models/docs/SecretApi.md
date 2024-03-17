# \SecretApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**secret_create**](SecretApi.md#secret_create) | **POST** /secrets/create | Create a secret
[**secret_delete**](SecretApi.md#secret_delete) | **DELETE** /secrets/{id} | Delete a secret
[**secret_inspect**](SecretApi.md#secret_inspect) | **GET** /secrets/{id} | Inspect a secret
[**secret_list**](SecretApi.md#secret_list) | **GET** /secrets | List secrets
[**secret_update**](SecretApi.md#secret_update) | **POST** /secrets/{id}/update | Update a Secret



## secret_create

> models::IdResponse secret_create(body)
Create a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**SecretCreateRequest**](SecretCreateRequest.md)> |  |  |

### Return type

[**models::IdResponse**](IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_delete

> secret_delete(id)
Delete a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the secret | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_inspect

> models::Secret secret_inspect(id)
Inspect a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the secret | [required] |

### Return type

[**models::Secret**](Secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_list

> Vec<models::Secret> secret_list(filters)
List secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the secrets list.  Available filters:  - `id=<secret id>` - `label=<key> or label=<key>=value` - `name=<secret name>` - `names=<secret name>`  |  |

### Return type

[**Vec<models::Secret>**](Secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_update

> secret_update(id, version, body)
Update a Secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID or name of the secret | [required] |
**version** | **i64** | The version number of the secret object being updated. This is required to avoid conflicting writes.  | [required] |
**body** | Option<[**SecretSpec**](SecretSpec.md)> | The spec of the secret to update. Currently, only the Labels field can be updated. All other fields must remain unchanged from the [SecretInspect endpoint](#operation/SecretInspect) response values.  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

