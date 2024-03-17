# SwarmSpecCaConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_cert_expiry** | Option<**i64**> | The duration node certificates are issued for. | [optional]
**external_cas** | Option<[**Vec<models::SwarmSpecCaConfigExternalCasInner>**](SwarmSpec_CAConfig_ExternalCAs_inner.md)> | Configuration for forwarding signing requests to an external certificate authority.  | [optional]
**signing_ca_cert** | Option<**String**> | The desired signing CA certificate for all swarm node TLS leaf certificates, in PEM format.  | [optional]
**signing_ca_key** | Option<**String**> | The desired signing CA key for all swarm node TLS leaf certificates, in PEM format.  | [optional]
**force_rotate** | Option<**i32**> | An integer whose purpose is to force swarm to generate a new signing CA certificate and key, if none have been specified in `SigningCACert` and `SigningCAKey`  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


