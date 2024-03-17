# TaskSpecContainerSpecConfigsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file** | Option<[**models::TaskSpecContainerSpecConfigsInnerFile**](TaskSpec_ContainerSpec_Configs_inner_File.md)> |  | [optional]
**runtime** | Option<[**serde_json::Value**](.md)> | Runtime represents a target that is not mounted into the container but is used by the task  <p><br /><p>  > **Note**: `Configs.File` and `Configs.Runtime` are mutually > exclusive  | [optional]
**config_id** | Option<**String**> | ConfigID represents the ID of the specific config that we're referencing.  | [optional]
**config_name** | Option<**String**> | ConfigName is the name of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


