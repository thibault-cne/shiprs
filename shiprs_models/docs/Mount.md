# Mount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target** | Option<**String**> | Container path. | [optional]
**source** | Option<**String**> | Mount source (e.g. a volume name, a host path). | [optional]
**r#type** | Option<**String**> | The mount type. Available types:  - `bind` Mounts a file or directory from the host into the container. Must exist prior to creating the container. - `volume` Creates a volume with the given name and options (or uses a pre-existing volume with the same name and options). These are **not** removed when the container is removed. - `tmpfs` Create a tmpfs with the given options. The mount source cannot be specified for tmpfs. - `npipe` Mounts a named pipe from the host into the container. Must exist prior to creating the container. - `cluster` a Swarm cluster volume  | [optional]
**read_only** | Option<**bool**> | Whether the mount should be read-only. | [optional]
**consistency** | Option<**String**> | The consistency requirement for the mount: `default`, `consistent`, `cached`, or `delegated`. | [optional]
**bind_options** | Option<[**models::MountBindOptions**](Mount_BindOptions.md)> |  | [optional]
**volume_options** | Option<[**models::MountVolumeOptions**](Mount_VolumeOptions.md)> |  | [optional]
**tmpfs_options** | Option<[**models::MountTmpfsOptions**](Mount_TmpfsOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


