# SystemVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**platform** | Option<[**models::SystemVersionPlatform**](SystemVersion_Platform.md)> |  | [optional]
**components** | Option<[**Vec<models::SystemVersionComponentsInner>**](SystemVersion_Components_inner.md)> | Information about system components  | [optional]
**version** | Option<**String**> | The version of the daemon | [optional]
**api_version** | Option<**String**> | The default (and highest) API version that is supported by the daemon  | [optional]
**min_api_version** | Option<**String**> | The minimum API version that is supported by the daemon  | [optional]
**git_commit** | Option<**String**> | The Git commit of the source code that was used to build the daemon  | [optional]
**go_version** | Option<**String**> | The version Go used to compile the daemon, and the version of the Go runtime in use.  | [optional]
**os** | Option<**String**> | The operating system that the daemon is running on (\"linux\" or \"windows\")  | [optional]
**arch** | Option<**String**> | The architecture that the daemon is running on  | [optional]
**kernel_version** | Option<**String**> | The kernel version (`uname -r`) that the daemon is running on.  This field is omitted when empty.  | [optional]
**experimental** | Option<**bool**> | Indicates if the daemon is started with experimental features enabled.  This field is omitted when empty / false.  | [optional]
**build_time** | Option<**String**> | The date and time that the daemon was compiled.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


