# SwarmInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_id** | Option<**String**> | Unique identifier of for this node in the swarm. | [optional][default to ]
**node_addr** | Option<**String**> | IP address at which this node can be reached by other nodes in the swarm.  | [optional][default to ]
**local_node_state** | Option<[**models::LocalNodeState**](LocalNodeState.md)> |  | [optional]
**control_available** | Option<**bool**> |  | [optional][default to false]
**error** | Option<**String**> |  | [optional][default to ]
**remote_managers** | Option<[**Vec<models::PeerNode>**](PeerNode.md)> | List of ID's and addresses of other managers in the swarm.  | [optional]
**nodes** | Option<**i32**> | Total number of nodes in the swarm. | [optional]
**managers** | Option<**i32**> | Total number of managers in the swarm. | [optional]
**cluster** | Option<[**models::ClusterInfo**](ClusterInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


