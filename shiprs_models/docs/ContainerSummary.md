# ContainerSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of this container | [optional]
**names** | Option<**Vec<String>**> | The names that this container has been given | [optional]
**image** | Option<**String**> | The name of the image used when creating this container | [optional]
**image_id** | Option<**String**> | The ID of the image that this container was created from | [optional]
**command** | Option<**String**> | Command to run when starting the container | [optional]
**created** | Option<**i64**> | When the container was created | [optional]
**ports** | Option<[**Vec<models::Port>**](Port.md)> | The ports exposed by this container | [optional]
**size_rw** | Option<**i64**> | The size of files that have been created or changed by this container | [optional]
**size_root_fs** | Option<**i64**> | The total size of all the files in this container | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]
**state** | Option<**String**> | The state of this container (e.g. `Exited`) | [optional]
**status** | Option<**String**> | Additional human-readable status of this container (e.g. `Exit 0`) | [optional]
**host_config** | Option<[**models::ContainerSummaryHostConfig**](ContainerSummary_HostConfig.md)> |  | [optional]
**network_settings** | Option<[**models::ContainerSummaryNetworkSettings**](ContainerSummary_NetworkSettings.md)> |  | [optional]
**mounts** | Option<[**Vec<models::MountPoint>**](MountPoint.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


