# NetworkCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The network's name. | 
**check_duplicate** | Option<**bool**> | Deprecated: CheckDuplicate is now always enabled.  | [optional]
**driver** | Option<**String**> | Name of the network driver plugin to use. | [optional][default to bridge]
**internal** | Option<**bool**> | Restrict external access to the network. | [optional]
**attachable** | Option<**bool**> | Globally scoped network is manually attachable by regular containers from workers in swarm mode.  | [optional]
**ingress** | Option<**bool**> | Ingress network is the network which provides the routing-mesh in swarm mode.  | [optional]
**ipam** | Option<[**models::Ipam**](IPAM.md)> |  | [optional]
**enable_ipv6** | Option<**bool**> | Enable IPv6 on the network. | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> | Network specific options to be used by the drivers. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


