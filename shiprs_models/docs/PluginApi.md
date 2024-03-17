# \PluginApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_plugin_privileges**](PluginApi.md#get_plugin_privileges) | **GET** /plugins/privileges | Get plugin privileges
[**plugin_create**](PluginApi.md#plugin_create) | **POST** /plugins/create | Create a plugin
[**plugin_delete**](PluginApi.md#plugin_delete) | **DELETE** /plugins/{name} | Remove a plugin
[**plugin_disable**](PluginApi.md#plugin_disable) | **POST** /plugins/{name}/disable | Disable a plugin
[**plugin_enable**](PluginApi.md#plugin_enable) | **POST** /plugins/{name}/enable | Enable a plugin
[**plugin_inspect**](PluginApi.md#plugin_inspect) | **GET** /plugins/{name}/json | Inspect a plugin
[**plugin_list**](PluginApi.md#plugin_list) | **GET** /plugins | List plugins
[**plugin_pull**](PluginApi.md#plugin_pull) | **POST** /plugins/pull | Install a plugin
[**plugin_push**](PluginApi.md#plugin_push) | **POST** /plugins/{name}/push | Push a plugin
[**plugin_set**](PluginApi.md#plugin_set) | **POST** /plugins/{name}/set | Configure a plugin
[**plugin_upgrade**](PluginApi.md#plugin_upgrade) | **POST** /plugins/{name}/upgrade | Upgrade a plugin



## get_plugin_privileges

> Vec<models::PluginPrivilege> get_plugin_privileges(remote)
Get plugin privileges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote** | **String** | The name of the plugin. The `:latest` tag is optional, and is the default if omitted.  | [required] |

### Return type

[**Vec<models::PluginPrivilege>**](PluginPrivilege.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_create

> plugin_create(name, tar_context)
Create a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the plugin. The `:latest` tag is optional, and is the default if omitted.  | [required] |
**tar_context** | Option<**std::path::PathBuf**> | Path to tar containing plugin rootfs and manifest |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-tar
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_delete

> models::Plugin plugin_delete(name, force)
Remove a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the plugin. The `:latest` tag is optional, and is the default if omitted.  | [required] |
**force** | Option<**bool**> | Disable the plugin before removing. This may result in issues if the plugin is in use by a container.  |  |[default to false]

### Return type

[**models::Plugin**](Plugin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_disable

> plugin_disable(name, force)
Disable a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the plugin. The `:latest` tag is optional, and is the default if omitted.  | [required] |
**force** | Option<**bool**> | Force disable a plugin even if still in use.  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_enable

> plugin_enable(name, timeout)
Enable a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the plugin. The `:latest` tag is optional, and is the default if omitted.  | [required] |
**timeout** | Option<**i32**> | Set the HTTP client timeout (in seconds) |  |[default to 0]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_inspect

> models::Plugin plugin_inspect(name)
Inspect a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the plugin. The `:latest` tag is optional, and is the default if omitted.  | [required] |

### Return type

[**models::Plugin**](Plugin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_list

> Vec<models::Plugin> plugin_list(filters)
List plugins

Returns information about installed plugins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the plugin list.  Available filters:  - `capability=<capability name>` - `enable=<true>|<false>`  |  |

### Return type

[**Vec<models::Plugin>**](Plugin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_pull

> plugin_pull(remote, name, x_registry_auth, body)
Install a plugin

Pulls and installs a plugin. After the plugin is installed, it can be enabled using the [`POST /plugins/{name}/enable` endpoint](#operation/PostPluginsEnable). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote** | **String** | Remote reference for plugin to install.  The `:latest` tag is optional, and is used as the default if omitted.  | [required] |
**name** | Option<**String**> | Local name for the pulled plugin.  The `:latest` tag is optional, and is used as the default if omitted.  |  |
**x_registry_auth** | Option<**String**> | A base64url-encoded auth configuration to use when pulling a plugin from a registry.  Refer to the [authentication section](#section/Authentication) for details.  |  |
**body** | Option<[**Vec<models::PluginPrivilege>**](PluginPrivilege.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_push

> plugin_push(name)
Push a plugin

Push a plugin to the registry. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the plugin. The `:latest` tag is optional, and is the default if omitted.  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_set

> plugin_set(name, body)
Configure a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the plugin. The `:latest` tag is optional, and is the default if omitted.  | [required] |
**body** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_upgrade

> plugin_upgrade(name, remote, x_registry_auth, body)
Upgrade a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the plugin. The `:latest` tag is optional, and is the default if omitted.  | [required] |
**remote** | **String** | Remote reference to upgrade to.  The `:latest` tag is optional, and is used as the default if omitted.  | [required] |
**x_registry_auth** | Option<**String**> | A base64url-encoded auth configuration to use when pulling a plugin from a registry.  Refer to the [authentication section](#section/Authentication) for details.  |  |
**body** | Option<[**Vec<models::PluginPrivilege>**](PluginPrivilege.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

