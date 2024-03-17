# SwarmSpecCaConfigExternalCasInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | Option<**String**> | Protocol for communication with the external CA (currently only `cfssl` is supported).  | [optional][default to Cfssl]
**url** | Option<**String**> | URL where certificate signing requests should be sent.  | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> | An object with key/value pairs that are interpreted as protocol-specific options for the external CA driver.  | [optional]
**ca_cert** | Option<**String**> | The root CA certificate (in PEM format) this external CA uses to issue TLS certificates (assumed to be to the current swarm root CA certificate if not provided).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


