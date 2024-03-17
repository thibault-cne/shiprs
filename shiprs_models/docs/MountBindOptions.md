# MountBindOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**propagation** | Option<**String**> | A propagation mode with the value `[r]private`, `[r]shared`, or `[r]slave`. | [optional]
**non_recursive** | Option<**bool**> | Disable recursive bind mount. | [optional][default to false]
**create_mountpoint** | Option<**bool**> | Create mount point on host if missing | [optional][default to false]
**read_only_non_recursive** | Option<**bool**> | Make the mount non-recursively read-only, but still leave the mount recursive (unless NonRecursive is set to true in conjunction).  | [optional][default to false]
**read_only_force_recursive** | Option<**bool**> | Raise an error if the mount cannot be made recursively read-only. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


