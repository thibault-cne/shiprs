# ContainerInspectResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the container | [optional]
**created** | Option<**String**> | The time the container was created | [optional]
**path** | Option<**String**> | The path to the command being run | [optional]
**args** | Option<**Vec<String>**> | The arguments to the command being run | [optional]
**state** | Option<[**models::ContainerState**](ContainerState.md)> |  | [optional]
**image** | Option<**String**> | The container's image ID | [optional]
**resolv_conf_path** | Option<**String**> |  | [optional]
**hostname_path** | Option<**String**> |  | [optional]
**hosts_path** | Option<**String**> |  | [optional]
**log_path** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**restart_count** | Option<**i32**> |  | [optional]
**driver** | Option<**String**> |  | [optional]
**platform** | Option<**String**> |  | [optional]
**mount_label** | Option<**String**> |  | [optional]
**process_label** | Option<**String**> |  | [optional]
**app_armor_profile** | Option<**String**> |  | [optional]
**exec_ids** | Option<**Vec<String>**> | IDs of exec instances that are running in the container. | [optional]
**host_config** | Option<[**models::HostConfig**](HostConfig.md)> |  | [optional]
**graph_driver** | Option<[**models::GraphDriverData**](GraphDriverData.md)> |  | [optional]
**size_rw** | Option<**i64**> | The size of files that have been created or changed by this container.  | [optional]
**size_root_fs** | Option<**i64**> | The total size of all the files in this container. | [optional]
**mounts** | Option<[**Vec<models::MountPoint>**](MountPoint.md)> |  | [optional]
**config** | Option<[**models::ContainerConfig**](ContainerConfig.md)> |  | [optional]
**network_settings** | Option<[**models::NetworkSettings**](NetworkSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


