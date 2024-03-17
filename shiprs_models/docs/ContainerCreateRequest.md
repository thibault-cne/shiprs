# ContainerCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hostname** | Option<**String**> | The hostname to use for the container, as a valid RFC 1123 hostname.  | [optional]
**domainname** | Option<**String**> | The domain name to use for the container.  | [optional]
**user** | Option<**String**> | The user that commands are run as inside the container. | [optional]
**attach_stdin** | Option<**bool**> | Whether to attach to `stdin`. | [optional][default to false]
**attach_stdout** | Option<**bool**> | Whether to attach to `stdout`. | [optional][default to true]
**attach_stderr** | Option<**bool**> | Whether to attach to `stderr`. | [optional][default to true]
**exposed_ports** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`  | [optional]
**tty** | Option<**bool**> | Attach standard streams to a TTY, including `stdin` if it is not closed.  | [optional][default to false]
**open_stdin** | Option<**bool**> | Open `stdin` | [optional][default to false]
**stdin_once** | Option<**bool**> | Close `stdin` after one attached client disconnects | [optional][default to false]
**env** | Option<**Vec<String>**> | A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.  | [optional]
**cmd** | Option<**Vec<String>**> | Command to run specified as a string or an array of strings.  | [optional]
**healthcheck** | Option<[**models::HealthConfig**](HealthConfig.md)> |  | [optional]
**args_escaped** | Option<**bool**> | Command is already escaped (Windows only) | [optional][default to false]
**image** | Option<**String**> | The name (or reference) of the image to use when creating the container, or which was used when the container was created.  | [optional]
**volumes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An object mapping mount point paths inside the container to empty objects.  | [optional]
**working_dir** | Option<**String**> | The working directory for commands to run in. | [optional]
**entrypoint** | Option<**Vec<String>**> | The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).  | [optional]
**network_disabled** | Option<**bool**> | Disable networking for the container. | [optional]
**mac_address** | Option<**String**> | MAC address of the container.  Deprecated: this field is deprecated in API v1.44 and up. Use EndpointSettings.MacAddress instead.  | [optional]
**on_build** | Option<**Vec<String>**> | `ONBUILD` metadata that were defined in the image's `Dockerfile`.  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]
**stop_signal** | Option<**String**> | Signal to stop a container as a string or unsigned integer.  | [optional]
**stop_timeout** | Option<**i32**> | Timeout to stop a container in seconds. | [optional]
**shell** | Option<**Vec<String>**> | Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.  | [optional]
**host_config** | Option<[**models::HostConfig**](HostConfig.md)> |  | [optional]
**networking_config** | Option<[**models::NetworkingConfig**](NetworkingConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


