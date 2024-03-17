# EventMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of object emitting the event | [optional]
**action** | Option<**String**> | The type of event | [optional]
**actor** | Option<[**models::EventActor**](EventActor.md)> |  | [optional]
**scope** | Option<**String**> | Scope of the event. Engine events are `local` scope. Cluster (Swarm) events are `swarm` scope.  | [optional]
**time** | Option<**i64**> | Timestamp of event | [optional]
**time_nano** | Option<**i64**> | Timestamp of event, with nanosecond accuracy | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


