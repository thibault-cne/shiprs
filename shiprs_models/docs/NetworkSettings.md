# NetworkSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bridge** | Option<**String**> | Name of the default bridge interface when dockerd's --bridge flag is set.  | [optional]
**sandbox_id** | Option<**String**> | SandboxID uniquely represents a container's network stack. | [optional]
**hairpin_mode** | Option<**bool**> | Indicates if hairpin NAT should be enabled on the virtual interface.  Deprecated: This field is never set and will be removed in a future release.  | [optional]
**link_local_ipv6_address** | Option<**String**> | IPv6 unicast address using the link-local prefix.  Deprecated: This field is never set and will be removed in a future release.  | [optional]
**link_local_ipv6_prefix_len** | Option<**i32**> | Prefix length of the IPv6 unicast address.  Deprecated: This field is never set and will be removed in a future release.  | [optional]
**ports** | Option<[**std::collections::HashMap<String, Vec<models::PortBinding>>**](Vec.md)> | PortMap describes the mapping of container ports to host ports, using the container's port-number and protocol as key in the format `<port>/<protocol>`, for example, `80/udp`.  If a container's port is mapped for multiple protocols, separate entries are added to the mapping table.  | [optional]
**sandbox_key** | Option<**String**> | SandboxKey is the full path of the netns handle | [optional]
**secondary_ip_addresses** | Option<[**Vec<models::Address>**](Address.md)> | Deprecated: This field is never set and will be removed in a future release. | [optional]
**secondary_ipv6_addresses** | Option<[**Vec<models::Address>**](Address.md)> | Deprecated: This field is never set and will be removed in a future release. | [optional]
**endpoint_id** | Option<**String**> | EndpointID uniquely represents a service endpoint in a Sandbox.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0  | [optional]
**gateway** | Option<**String**> | Gateway address for the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0  | [optional]
**global_ipv6_address** | Option<**String**> | Global IPv6 address for the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0  | [optional]
**global_ipv6_prefix_len** | Option<**i32**> | Mask length of the global IPv6 address.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0  | [optional]
**ip_address** | Option<**String**> | IPv4 address for the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0  | [optional]
**ip_prefix_len** | Option<**i32**> | Mask length of the IPv4 address.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0  | [optional]
**ipv6_gateway** | Option<**String**> | IPv6 gateway address for this network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0  | [optional]
**mac_address** | Option<**String**> | MAC address for the container on the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0  | [optional]
**networks** | Option<[**std::collections::HashMap<String, models::EndpointSettings>**](EndpointSettings.md)> | Information about all networks that the container is connected to.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


