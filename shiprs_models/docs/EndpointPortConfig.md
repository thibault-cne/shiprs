# EndpointPortConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**protocol** | Option<**String**> |  | [optional]
**target_port** | Option<**i32**> | The port inside the container. | [optional]
**published_port** | Option<**i32**> | The port on the swarm hosts. | [optional]
**publish_mode** | Option<**String**> | The mode in which port is published.  <p><br /></p>  - \"ingress\" makes the target port accessible on every node,   regardless of whether there is a task for the service running on   that node or not. - \"host\" bypasses the routing mesh and publish the port directly on   the swarm node where that service is running.  | [optional][default to Ingress]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


