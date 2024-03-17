# \DistributionApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**distribution_inspect**](DistributionApi.md#distribution_inspect) | **GET** /distribution/{name}/json | Get image information from the registry



## distribution_inspect

> models::DistributionInspect distribution_inspect(name)
Get image information from the registry

Return image digest and platform information by contacting the registry. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Image name or id | [required] |

### Return type

[**models::DistributionInspect**](DistributionInspect.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

