# TaskSpecContainerSpecPrivileges

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credential_spec** | Option<[**models::TaskSpecContainerSpecPrivilegesCredentialSpec**](TaskSpec_ContainerSpec_Privileges_CredentialSpec.md)> |  | [optional]
**se_linux_context** | Option<[**models::TaskSpecContainerSpecPrivilegesSeLinuxContext**](TaskSpec_ContainerSpec_Privileges_SELinuxContext.md)> |  | [optional]
**seccomp** | Option<[**models::TaskSpecContainerSpecPrivilegesSeccomp**](TaskSpec_ContainerSpec_Privileges_Seccomp.md)> |  | [optional]
**app_armor** | Option<[**models::TaskSpecContainerSpecPrivilegesAppArmor**](TaskSpec_ContainerSpec_Privileges_AppArmor.md)> |  | [optional]
**no_new_privileges** | Option<**bool**> | Configuration of the no_new_privs bit in the container | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


