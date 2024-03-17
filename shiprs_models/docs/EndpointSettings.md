# EndpointSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ipam_config** | Option<[**models::EndpointIpamConfig**](EndpointIPAMConfig.md)> |  | [optional]
**links** | Option<**Vec<String>**> |  | [optional]
**mac_address** | Option<**String**> | MAC address for the endpoint on this network. The network driver might ignore this parameter.  | [optional]
**aliases** | Option<**Vec<String>**> |  | [optional]
**network_id** | Option<**String**> | Unique ID of the network.  | [optional]
**endpoint_id** | Option<**String**> | Unique ID for the service endpoint in a Sandbox.  | [optional]
**gateway** | Option<**String**> | Gateway address for this network.  | [optional]
**ip_address** | Option<**String**> | IPv4 address.  | [optional]
**ip_prefix_len** | Option<**i32**> | Mask length of the IPv4 address.  | [optional]
**ipv6_gateway** | Option<**String**> | IPv6 gateway address.  | [optional]
**global_ipv6_address** | Option<**String**> | Global IPv6 address.  | [optional]
**global_ipv6_prefix_len** | Option<**i64**> | Mask length of the global IPv6 address.  | [optional]
**driver_opts** | Option<**std::collections::HashMap<String, String>**> | DriverOpts is a mapping of driver options and values. These options are passed directly to the driver and are driver specific.  | [optional]
**dns_names** | Option<**Vec<String>**> | List of all DNS names an endpoint has on a specific network. This list is based on the container name, network aliases, container short ID, and hostname.  These DNS names are non-fully qualified but can contain several dots. You can get fully qualified DNS names by appending `.<network-name>`. For instance, if container name is `my.ctr` and the network is named `testnet`, `DNSNames` will contain `my.ctr` and the FQDN will be `my.ctr.testnet`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


