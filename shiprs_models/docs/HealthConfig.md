# HealthConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**test** | Option<**Vec<String>**> | The test to perform. Possible values are:  - `[]` inherit healthcheck from image or parent image - `[\"NONE\"]` disable healthcheck - `[\"CMD\", args...]` exec arguments directly - `[\"CMD-SHELL\", command]` run command with system's default shell  | [optional]
**interval** | Option<**i64**> | The time to wait between checks in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.  | [optional]
**timeout** | Option<**i64**> | The time to wait before considering the check to have hung. It should be 0 or at least 1000000 (1 ms). 0 means inherit.  | [optional]
**retries** | Option<**i32**> | The number of consecutive failures needed to consider a container as unhealthy. 0 means inherit.  | [optional]
**start_period** | Option<**i64**> | Start period for the container to initialize before starting health-retries countdown in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.  | [optional]
**start_interval** | Option<**i64**> | The time to wait between checks in nanoseconds during the start period. It should be 0 or at least 1000000 (1 ms). 0 means inherit.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


