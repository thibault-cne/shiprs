# SwarmJoinRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**listen_addr** | Option<**String**> | Listen address used for inter-manager communication if the node gets promoted to manager, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP).  | [optional]
**advertise_addr** | Option<**String**> | Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.  | [optional]
**data_path_addr** | Option<**String**> | Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same address as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.  | [optional]
**remote_addrs** | Option<**Vec<String>**> | Addresses of manager nodes already participating in the swarm.  | [optional]
**join_token** | Option<**String**> | Secret token for joining this swarm. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


