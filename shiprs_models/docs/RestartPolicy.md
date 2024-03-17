# RestartPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | - Empty string means not to restart - `no` Do not automatically restart - `always` Always restart - `unless-stopped` Restart always except when the user has manually stopped the container - `on-failure` Restart only when the container exit code is non-zero  | [optional]
**maximum_retry_count** | Option<**i32**> | If `on-failure` is used, the number of times to retry before giving up.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


