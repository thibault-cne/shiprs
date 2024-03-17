# ClusterVolumeInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capacity_bytes** | Option<**i64**> | The capacity of the volume in bytes. A value of 0 indicates that the capacity is unknown.  | [optional]
**volume_context** | Option<**std::collections::HashMap<String, String>**> | A map of strings to strings returned from the storage plugin when the volume is created.  | [optional]
**volume_id** | Option<**String**> | The ID of the volume as returned by the CSI storage plugin. This is distinct from the volume's ID as provided by Docker. This ID is never used by the user when communicating with Docker to refer to this volume. If the ID is blank, then the Volume has not been successfully created in the plugin yet.  | [optional]
**accessible_topology** | Option<[**Vec<std::collections::HashMap<String, String>>**](std::collections::HashMap.md)> | The topology this volume is actually accessible from.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


