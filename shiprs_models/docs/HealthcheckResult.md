# HealthcheckResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start** | Option<**String**> | Date and time at which this check started in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  | [optional]
**end** | Option<**String**> | Date and time at which this check ended in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  | [optional]
**exit_code** | Option<**i32**> | ExitCode meanings:  - `0` healthy - `1` unhealthy - `2` reserved (considered unhealthy) - other values: error running probe  | [optional]
**output** | Option<**String**> | Output from last check | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


