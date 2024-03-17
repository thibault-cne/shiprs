# ClusterVolumeSpecAccessMode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scope** | Option<**String**> | The set of nodes this volume can be used on at one time. - `single` The volume may only be scheduled to one node at a time. - `multi` the volume may be scheduled to any supported number of nodes at a time.  | [optional][default to Single]
**sharing** | Option<**String**> | The number and way that different tasks can use this volume at one time. - `none` The volume may only be used by one task at a time. - `readonly` The volume may be used by any number of tasks, but they all must mount the volume as readonly - `onewriter` The volume may be used by any number of tasks, but only one may mount it as read/write. - `all` The volume may have any number of readers and writers.  | [optional][default to None]
**mount_volume** | Option<[**serde_json::Value**](.md)> | Options for using this volume as a Mount-type volume.      Either MountVolume or BlockVolume, but not both, must be     present.   properties:     FsType:       type: \"string\"       description: |         Specifies the filesystem type for the mount volume.         Optional.     MountFlags:       type: \"array\"       description: |         Flags to pass when mounting the volume. Optional.       items:         type: \"string\" BlockVolume:   type: \"object\"   description: |     Options for using this volume as a Block-type volume.     Intentionally empty.  | [optional]
**secrets** | Option<[**Vec<models::ClusterVolumeSpecAccessModeSecretsInner>**](ClusterVolumeSpec_AccessMode_Secrets_inner.md)> | Swarm Secrets that are passed to the CSI storage plugin when operating on this volume.  | [optional]
**accessibility_requirements** | Option<[**models::ClusterVolumeSpecAccessModeAccessibilityRequirements**](ClusterVolumeSpec_AccessMode_AccessibilityRequirements.md)> |  | [optional]
**capacity_range** | Option<[**models::ClusterVolumeSpecAccessModeCapacityRange**](ClusterVolumeSpec_AccessMode_CapacityRange.md)> |  | [optional]
**availability** | Option<**String**> | The availability of the volume for use in tasks. - `active` The volume is fully available for scheduling on the cluster - `pause` No new workloads should use the volume, but existing workloads are not stopped. - `drain` All workloads using this volume should be stopped and rescheduled, and no new ones should be started.  | [optional][default to Active]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


