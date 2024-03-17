# ClusterVolume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Swarm ID of this volume. Because cluster volumes are Swarm objects, they have an ID, unlike non-cluster volumes. This ID can be used to refer to the Volume instead of the name.  | [optional]
**version** | Option<[**models::ObjectVersion**](ObjectVersion.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**spec** | Option<[**models::ClusterVolumeSpec**](ClusterVolumeSpec.md)> |  | [optional]
**info** | Option<[**models::ClusterVolumeInfo**](ClusterVolume_Info.md)> |  | [optional]
**publish_status** | Option<[**Vec<models::ClusterVolumePublishStatusInner>**](ClusterVolume_PublishStatus_inner.md)> | The status of the volume as it pertains to its publishing and use on specific nodes  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


