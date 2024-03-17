# ExecConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attach_stdin** | Option<**bool**> | Attach to `stdin` of the exec command. | [optional]
**attach_stdout** | Option<**bool**> | Attach to `stdout` of the exec command. | [optional]
**attach_stderr** | Option<**bool**> | Attach to `stderr` of the exec command. | [optional]
**console_size** | Option<**Vec<i32>**> | Initial console size, as an `[height, width]` array. | [optional]
**detach_keys** | Option<**String**> | Override the key sequence for detaching a container. Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.  | [optional]
**tty** | Option<**bool**> | Allocate a pseudo-TTY. | [optional]
**env** | Option<**Vec<String>**> | A list of environment variables in the form `[\"VAR=value\", ...]`.  | [optional]
**cmd** | Option<**Vec<String>**> | Command to run, as a string or array of strings. | [optional]
**privileged** | Option<**bool**> | Runs the exec process with extended privileges. | [optional][default to false]
**user** | Option<**String**> | The user, and optionally, group to run the exec process inside the container. Format is one of: `user`, `user:group`, `uid`, or `uid:gid`.  | [optional]
**working_dir** | Option<**String**> | The working directory for the exec process inside the container.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


