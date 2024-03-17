# ImageInspect

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.  | [optional]
**repo_tags** | Option<**Vec<String>**> | List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is \"untagged\", in which case it can still be referenced by its ID.  | [optional]
**repo_digests** | Option<**Vec<String>**> | List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.  | [optional]
**parent** | Option<**String**> | ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry.  | [optional]
**comment** | Option<**String**> | Optional message that was set when committing or importing the image.  | [optional]
**created** | Option<**String**> | Date and time at which the image was created, formatted in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  This information is only available if present in the image, and omitted otherwise.  | [optional]
**container** | Option<**String**> | The ID of the container that was used to create the image.  Depending on how the image was created, this field may be empty.  **Deprecated**: this field is kept for backward compatibility, but will be removed in API v1.45.  | [optional]
**container_config** | Option<[**models::ContainerConfig**](ContainerConfig.md)> |  | [optional]
**docker_version** | Option<**String**> | The version of Docker that was used to build the image.  Depending on how the image was created, this field may be empty.  | [optional]
**author** | Option<**String**> | Name of the author that was specified when committing the image, or as specified through MAINTAINER (deprecated) in the Dockerfile.  | [optional]
**config** | Option<[**models::ContainerConfig**](ContainerConfig.md)> |  | [optional]
**architecture** | Option<**String**> | Hardware CPU architecture that the image runs on.  | [optional]
**variant** | Option<**String**> | CPU architecture variant (presently ARM-only).  | [optional]
**os** | Option<**String**> | Operating System the image is built to run on.  | [optional]
**os_version** | Option<**String**> | Operating System version the image is built to run on (especially for Windows).  | [optional]
**size** | Option<**i64**> | Total size of the image including all layers it is composed of.  | [optional]
**virtual_size** | Option<**i64**> | Total size of the image including all layers it is composed of.  Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead.  | [optional]
**graph_driver** | Option<[**models::GraphDriverData**](GraphDriverData.md)> |  | [optional]
**root_fs** | Option<[**models::ImageInspectRootFs**](ImageInspect_RootFS.md)> |  | [optional]
**metadata** | Option<[**models::ImageInspectMetadata**](ImageInspect_Metadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


