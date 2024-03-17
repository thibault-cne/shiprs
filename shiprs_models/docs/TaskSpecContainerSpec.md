# TaskSpecContainerSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | Option<**String**> | The image name to use for the container | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value data. | [optional]
**command** | Option<**Vec<String>**> | The command to be run in the image. | [optional]
**args** | Option<**Vec<String>**> | Arguments to the command. | [optional]
**hostname** | Option<**String**> | The hostname to use for the container, as a valid [RFC 1123](https://tools.ietf.org/html/rfc1123) hostname.  | [optional]
**env** | Option<**Vec<String>**> | A list of environment variables in the form `VAR=value`.  | [optional]
**dir** | Option<**String**> | The working directory for commands to run in. | [optional]
**user** | Option<**String**> | The user inside the container. | [optional]
**groups** | Option<**Vec<String>**> | A list of additional groups that the container process will run as.  | [optional]
**privileges** | Option<[**models::TaskSpecContainerSpecPrivileges**](TaskSpec_ContainerSpec_Privileges.md)> |  | [optional]
**tty** | Option<**bool**> | Whether a pseudo-TTY should be allocated. | [optional]
**open_stdin** | Option<**bool**> | Open `stdin` | [optional]
**read_only** | Option<**bool**> | Mount the container's root filesystem as read only. | [optional]
**mounts** | Option<[**Vec<models::Mount>**](Mount.md)> | Specification for mounts to be added to containers created as part of the service.  | [optional]
**stop_signal** | Option<**String**> | Signal to stop the container. | [optional]
**stop_grace_period** | Option<**i64**> | Amount of time to wait for the container to terminate before forcefully killing it.  | [optional]
**health_check** | Option<[**models::HealthConfig**](HealthConfig.md)> |  | [optional]
**hosts** | Option<**Vec<String>**> | A list of hostname/IP mappings to add to the container's `hosts` file. The format of extra hosts is specified in the [hosts(5)](http://man7.org/linux/man-pages/man5/hosts.5.html) man page:      IP_address canonical_hostname [aliases...]  | [optional]
**dns_config** | Option<[**models::TaskSpecContainerSpecDnsConfig**](TaskSpec_ContainerSpec_DNSConfig.md)> |  | [optional]
**secrets** | Option<[**Vec<models::TaskSpecContainerSpecSecretsInner>**](TaskSpec_ContainerSpec_Secrets_inner.md)> | Secrets contains references to zero or more secrets that will be exposed to the service.  | [optional]
**configs** | Option<[**Vec<models::TaskSpecContainerSpecConfigsInner>**](TaskSpec_ContainerSpec_Configs_inner.md)> | Configs contains references to zero or more configs that will be exposed to the service.  | [optional]
**isolation** | Option<**String**> | Isolation technology of the containers running the service. (Windows only)  | [optional]
**init** | Option<**bool**> | Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.  | [optional]
**sysctls** | Option<**std::collections::HashMap<String, String>**> | Set kernel namedspaced parameters (sysctls) in the container. The Sysctls option on services accepts the same sysctls as the are supported on containers. Note that while the same sysctls are supported, no guarantees or checks are made about their suitability for a clustered environment, and it's up to the user to determine whether a given sysctl will work properly in a Service.  | [optional]
**capability_add** | Option<**Vec<String>**> | A list of kernel capabilities to add to the default set for the container.  | [optional]
**capability_drop** | Option<**Vec<String>**> | A list of kernel capabilities to drop from the default set for the container.  | [optional]
**ulimits** | Option<[**Vec<models::ResourcesUlimitsInner>**](Resources_Ulimits_inner.md)> | A list of resource limits to set in the container. For example: `{\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048}`\"  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


