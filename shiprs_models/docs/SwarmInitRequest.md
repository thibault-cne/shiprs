# SwarmInitRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**listen_addr** | Option<**String**> | Listen address used for inter-manager communication, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP). This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the default swarm listening port is used.  | [optional]
**advertise_addr** | Option<**String**> | Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.  | [optional]
**data_path_addr** | Option<**String**> | Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same address as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other  nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.  | [optional]
**data_path_port** | Option<**i32**> | DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. if no port is set or is set to 0, default port 4789 will be used.  | [optional]
**default_addr_pool** | Option<**Vec<String>**> | Default Address Pool specifies default subnet pools for global scope networks.  | [optional]
**force_new_cluster** | Option<**bool**> | Force creation of a new swarm. | [optional]
**subnet_size** | Option<**i32**> | SubnetSize specifies the subnet size of the networks created from the default subnet pool.  | [optional]
**spec** | Option<[**models::SwarmSpec**](SwarmSpec.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


