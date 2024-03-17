# Health

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Status is one of `none`, `starting`, `healthy` or `unhealthy`  - \"none\"      Indicates there is no healthcheck - \"starting\"  Starting indicates that the container is not yet ready - \"healthy\"   Healthy indicates that the container is running correctly - \"unhealthy\" Unhealthy indicates that the container has a problem  | [optional]
**failing_streak** | Option<**i32**> | FailingStreak is the number of consecutive failures | [optional]
**log** | Option<[**Vec<models::HealthcheckResult>**](HealthcheckResult.md)> | Log contains the last few results (oldest first)  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


