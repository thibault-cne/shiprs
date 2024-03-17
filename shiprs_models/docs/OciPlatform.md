# OciPlatform

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**architecture** | Option<**String**> | The CPU architecture, for example `amd64` or `ppc64`.  | [optional]
**os** | Option<**String**> | The operating system, for example `linux` or `windows`.  | [optional]
**os_period_version** | Option<**String**> | Optional field specifying the operating system version, for example on Windows `10.0.19041.1165`.  | [optional]
**os_period_features** | Option<**Vec<String>**> | Optional field specifying an array of strings, each listing a required OS feature (for example on Windows `win32k`).  | [optional]
**variant** | Option<**String**> | Optional field specifying a variant of the CPU, for example `v7` to specify ARMv7 when architecture is `arm`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


