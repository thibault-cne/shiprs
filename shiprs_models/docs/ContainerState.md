# ContainerState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | String representation of the container state. Can be one of \"created\", \"running\", \"paused\", \"restarting\", \"removing\", \"exited\", or \"dead\".  | [optional]
**running** | Option<**bool**> | Whether this container is running.  Note that a running container can be _paused_. The `Running` and `Paused` booleans are not mutually exclusive:  When pausing a container (on Linux), the freezer cgroup is used to suspend all processes in the container. Freezing the process requires the process to be running. As a result, paused containers are both `Running` _and_ `Paused`.  Use the `Status` field instead to determine if a container's state is \"running\".  | [optional]
**paused** | Option<**bool**> | Whether this container is paused. | [optional]
**restarting** | Option<**bool**> | Whether this container is restarting. | [optional]
**oom_killed** | Option<**bool**> | Whether a process within this container has been killed because it ran out of memory since the container was last started.  | [optional]
**dead** | Option<**bool**> |  | [optional]
**pid** | Option<**i32**> | The process ID of this container | [optional]
**exit_code** | Option<**i32**> | The last exit code of this container | [optional]
**error** | Option<**String**> |  | [optional]
**started_at** | Option<**String**> | The time when this container was last started. | [optional]
**finished_at** | Option<**String**> | The time when this container last exited. | [optional]
**health** | Option<[**models::Health**](Health.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


