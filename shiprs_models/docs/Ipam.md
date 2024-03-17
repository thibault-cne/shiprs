# Ipam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**driver** | Option<**String**> | Name of the IPAM driver to use. | [optional][default to default]
**config** | Option<[**Vec<models::IpamConfig>**](IPAMConfig.md)> | List of IPAM configuration options, specified as a map:  ``` {\"Subnet\": <CIDR>, \"IPRange\": <CIDR>, \"Gateway\": <IP address>, \"AuxAddress\": <device_name:IP address>} ```  | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> | Driver-specific options, specified as a map. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


