# Swarm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the swarm. | [optional]
**version** | Option<[**models::ObjectVersion**](ObjectVersion.md)> |  | [optional]
**created_at** | Option<**String**> | Date and time at which the swarm was initialised in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  | [optional]
**updated_at** | Option<**String**> | Date and time at which the swarm was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  | [optional]
**spec** | Option<[**models::SwarmSpec**](SwarmSpec.md)> |  | [optional]
**tls_info** | Option<[**models::TlsInfo**](TLSInfo.md)> |  | [optional]
**root_rotation_in_progress** | Option<**bool**> | Whether there is currently a root CA rotation in progress for the swarm  | [optional]
**data_path_port** | Option<**i32**> | DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. If no port is set or is set to 0, the default port (4789) is used.  | [optional]
**default_addr_pool** | Option<**Vec<String>**> | Default Address Pool specifies default subnet pools for global scope networks.  | [optional]
**subnet_size** | Option<**i32**> | SubnetSize specifies the subnet size of the networks created from the default subnet pool.  | [optional]
**join_tokens** | Option<[**models::JoinTokens**](JoinTokens.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


