# TaskSpecContainerSpecPrivilegesCredentialSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<**String**> | Load credential spec from a Swarm Config with the given ID. The specified config must also be present in the Configs field with the Runtime property set.  <p><br /></p>   > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.  | [optional]
**file** | Option<**String**> | Load credential spec from this file. The file is read by the daemon, and must be present in the `CredentialSpecs` subdirectory in the docker data directory, which defaults to `C:\\ProgramData\\Docker\\` on Windows.  For example, specifying `spec.json` loads `C:\\ProgramData\\Docker\\CredentialSpecs\\spec.json`.  <p><br /></p>  > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.  | [optional]
**registry** | Option<**String**> | Load credential spec from this value in the Windows registry. The specified registry value must be located in:  `HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Virtualization\\Containers\\CredentialSpecs`  <p><br /></p>   > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


