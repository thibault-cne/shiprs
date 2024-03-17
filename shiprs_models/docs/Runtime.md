# Runtime

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | Option<**String**> | Name and, optional, path, of the OCI executable binary.  If the path is omitted, the daemon searches the host's `$PATH` for the binary and uses the first result.  | [optional]
**runtime_args** | Option<**Vec<String>**> | List of command-line arguments to pass to the runtime when invoked.  | [optional]
**status** | Option<**std::collections::HashMap<String, String>**> | Information specific to the runtime.  While this API specification does not define data provided by runtimes, the following well-known properties may be provided by runtimes:  `org.opencontainers.runtime-spec.features`: features structure as defined in the [OCI Runtime Specification](https://github.com/opencontainers/runtime-spec/blob/main/features.md), in a JSON string representation.  <p><br /></p>  > **Note**: The information returned in this field, including the > formatting of values and labels, should not be considered stable, > and may change without notice.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


