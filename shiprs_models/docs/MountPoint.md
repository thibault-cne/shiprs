# MountPoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The mount type:  - `bind` a mount of a file or directory from the host into the container. - `volume` a docker volume with the given `Name`. - `tmpfs` a `tmpfs`. - `npipe` a named pipe from the host into the container. - `cluster` a Swarm cluster volume  | [optional]
**name** | Option<**String**> | Name is the name reference to the underlying data defined by `Source` e.g., the volume name.  | [optional]
**source** | Option<**String**> | Source location of the mount.  For volumes, this contains the storage location of the volume (within `/var/lib/docker/volumes/`). For bind-mounts, and `npipe`, this contains the source (host) part of the bind-mount. For `tmpfs` mount points, this field is empty.  | [optional]
**destination** | Option<**String**> | Destination is the path relative to the container root (`/`) where the `Source` is mounted inside the container.  | [optional]
**driver** | Option<**String**> | Driver is the volume driver used to create the volume (if it is a volume).  | [optional]
**mode** | Option<**String**> | Mode is a comma separated list of options supplied by the user when creating the bind/volume mount.  The default is platform-specific (`\"z\"` on Linux, empty on Windows).  | [optional]
**rw** | Option<**bool**> | Whether the mount is mounted writable (read-write).  | [optional]
**propagation** | Option<**String**> | Propagation describes how mounts are propagated from the host into the mount point, and vice-versa. Refer to the [Linux kernel documentation](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt) for details. This field is not used on Windows.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


