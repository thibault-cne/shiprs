# ClusterVolumePublishStatusInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_id** | Option<**String**> | The ID of the Swarm node the volume is published on.  | [optional]
**state** | Option<**String**> | The published state of the volume. * `pending-publish` The volume should be published to this node, but the call to the controller plugin to do so has not yet been successfully completed. * `published` The volume is published successfully to the node. * `pending-node-unpublish` The volume should be unpublished from the node, and the manager is awaiting confirmation from the worker that it has done so. * `pending-controller-unpublish` The volume is successfully unpublished from the node, but has not yet been successfully unpublished on the controller.  | [optional]
**publish_context** | Option<**std::collections::HashMap<String, String>**> | A map of strings to strings returned by the CSI controller plugin when a volume is published.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


