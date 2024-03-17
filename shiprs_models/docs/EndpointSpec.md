# EndpointSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | Option<**String**> | The mode of resolution to use for internal load balancing between tasks.  | [optional][default to Vip]
**ports** | Option<[**Vec<models::EndpointPortConfig>**](EndpointPortConfig.md)> | List of exposed ports that this service is accessible on from the outside. Ports can only be provided if `vip` resolution mode is used.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


