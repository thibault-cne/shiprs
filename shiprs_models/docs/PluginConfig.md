# PluginConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**docker_version** | Option<**String**> | Docker Version used to create the plugin | [optional]
**description** | **String** |  | 
**documentation** | **String** |  | 
**interface** | [**models::PluginConfigInterface**](Plugin_Config_Interface.md) |  | 
**entrypoint** | **Vec<String>** |  | 
**work_dir** | **String** |  | 
**user** | Option<[**models::PluginConfigUser**](Plugin_Config_User.md)> |  | [optional]
**network** | [**models::PluginConfigNetwork**](Plugin_Config_Network.md) |  | 
**linux** | [**models::PluginConfigLinux**](Plugin_Config_Linux.md) |  | 
**propagated_mount** | **String** |  | 
**ipc_host** | **bool** |  | 
**pid_host** | **bool** |  | 
**mounts** | [**Vec<models::PluginMount>**](PluginMount.md) |  | 
**env** | [**Vec<models::PluginEnv>**](PluginEnv.md) |  | 
**args** | [**models::PluginConfigArgs**](Plugin_Config_Args.md) |  | 
**rootfs** | Option<[**models::PluginConfigRootfs**](Plugin_Config_rootfs.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


