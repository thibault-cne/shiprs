# Task

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the task. | [optional]
**version** | Option<[**models::ObjectVersion**](ObjectVersion.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**name** | Option<**String**> | Name of the task. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]
**spec** | Option<[**models::TaskSpec**](TaskSpec.md)> |  | [optional]
**service_id** | Option<**String**> | The ID of the service this task is part of. | [optional]
**slot** | Option<**i32**> |  | [optional]
**node_id** | Option<**String**> | The ID of the node that this task is on. | [optional]
**assigned_generic_resources** | Option<[**Vec<models::GenericResourcesInner>**](GenericResources_inner.md)> | User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, `GPU=UUID1`).  | [optional]
**status** | Option<[**models::TaskStatus**](TaskStatus.md)> |  | [optional]
**desired_state** | Option<[**models::TaskState**](TaskState.md)> |  | [optional]
**job_iteration** | Option<[**models::ObjectVersion**](ObjectVersion.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


