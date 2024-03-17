# \ContainerApi

All URIs are relative to *http://localhost/v1.44*

Method | HTTP request | Description
------------- | ------------- | -------------
[**container_archive**](ContainerApi.md#container_archive) | **GET** /containers/{id}/archive | Get an archive of a filesystem resource in a container
[**container_archive_info**](ContainerApi.md#container_archive_info) | **HEAD** /containers/{id}/archive | Get information about files in a container
[**container_attach**](ContainerApi.md#container_attach) | **POST** /containers/{id}/attach | Attach to a container
[**container_attach_websocket**](ContainerApi.md#container_attach_websocket) | **GET** /containers/{id}/attach/ws | Attach to a container via a websocket
[**container_changes**](ContainerApi.md#container_changes) | **GET** /containers/{id}/changes | Get changes on a container’s filesystem
[**container_create**](ContainerApi.md#container_create) | **POST** /containers/create | Create a container
[**container_delete**](ContainerApi.md#container_delete) | **DELETE** /containers/{id} | Remove a container
[**container_export**](ContainerApi.md#container_export) | **GET** /containers/{id}/export | Export a container
[**container_inspect**](ContainerApi.md#container_inspect) | **GET** /containers/{id}/json | Inspect a container
[**container_kill**](ContainerApi.md#container_kill) | **POST** /containers/{id}/kill | Kill a container
[**container_list**](ContainerApi.md#container_list) | **GET** /containers/json | List containers
[**container_logs**](ContainerApi.md#container_logs) | **GET** /containers/{id}/logs | Get container logs
[**container_pause**](ContainerApi.md#container_pause) | **POST** /containers/{id}/pause | Pause a container
[**container_prune**](ContainerApi.md#container_prune) | **POST** /containers/prune | Delete stopped containers
[**container_rename**](ContainerApi.md#container_rename) | **POST** /containers/{id}/rename | Rename a container
[**container_resize**](ContainerApi.md#container_resize) | **POST** /containers/{id}/resize | Resize a container TTY
[**container_restart**](ContainerApi.md#container_restart) | **POST** /containers/{id}/restart | Restart a container
[**container_start**](ContainerApi.md#container_start) | **POST** /containers/{id}/start | Start a container
[**container_stats**](ContainerApi.md#container_stats) | **GET** /containers/{id}/stats | Get container stats based on resource usage
[**container_stop**](ContainerApi.md#container_stop) | **POST** /containers/{id}/stop | Stop a container
[**container_top**](ContainerApi.md#container_top) | **GET** /containers/{id}/top | List processes running inside a container
[**container_unpause**](ContainerApi.md#container_unpause) | **POST** /containers/{id}/unpause | Unpause a container
[**container_update**](ContainerApi.md#container_update) | **POST** /containers/{id}/update | Update a container
[**container_wait**](ContainerApi.md#container_wait) | **POST** /containers/{id}/wait | Wait for a container
[**put_container_archive**](ContainerApi.md#put_container_archive) | **PUT** /containers/{id}/archive | Extract an archive of files or folders to a directory in a container



## container_archive

> container_archive(id, path)
Get an archive of a filesystem resource in a container

Get a tar archive of a resource in the filesystem of container id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**path** | **String** | Resource in the container’s filesystem to archive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-tar, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_archive_info

> container_archive_info(id, path)
Get information about files in a container

A response header `X-Docker-Container-Path-Stat` is returned, containing a base64 - encoded JSON object with some filesystem header information about the path. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**path** | **String** | Resource in the container’s filesystem to archive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_attach

> container_attach(id, detach_keys, logs, stream, stdin, stdout, stderr)
Attach to a container

Attach to a container to read its output or send it input. You can attach to the same container multiple times and you can reattach to containers that have been detached.  Either the `stream` or `logs` parameter must be `true` for this endpoint to do anything.  See the [documentation for the `docker attach` command](https://docs.docker.com/engine/reference/commandline/attach/) for more details.  ### Hijacking  This endpoint hijacks the HTTP connection to transport `stdin`, `stdout`, and `stderr` on the same socket.  This is the response from the daemon for an attach request:  ``` HTTP/1.1 200 OK Content-Type: application/vnd.docker.raw-stream  [STREAM] ```  After the headers and two new lines, the TCP connection can now be used for raw, bidirectional communication between the client and server.  To hint potential proxies about connection hijacking, the Docker client can also optionally send connection upgrade headers.  For example, the client sends this request to upgrade the connection:  ``` POST /containers/16253994b7c4/attach?stream=1&stdout=1 HTTP/1.1 Upgrade: tcp Connection: Upgrade ```  The Docker daemon will respond with a `101 UPGRADED` response, and will similarly follow with the raw stream:  ``` HTTP/1.1 101 UPGRADED Content-Type: application/vnd.docker.raw-stream Connection: Upgrade Upgrade: tcp  [STREAM] ```  ### Stream format  When the TTY setting is disabled in [`POST /containers/create`](#operation/ContainerCreate), the HTTP Content-Type header is set to application/vnd.docker.multiplexed-stream and the stream over the hijacked connected is multiplexed to separate out `stdout` and `stderr`. The stream consists of a series of frames, each containing a header and a payload.  The header contains the information which the stream writes (`stdout` or `stderr`). It also contains the size of the associated frame encoded in the last four bytes (`uint32`).  It is encoded on the first eight bytes like this:  ```go header := [8]byte{STREAM_TYPE, 0, 0, 0, SIZE1, SIZE2, SIZE3, SIZE4} ```  `STREAM_TYPE` can be:  - 0: `stdin` (is written on `stdout`) - 1: `stdout` - 2: `stderr`  `SIZE1, SIZE2, SIZE3, SIZE4` are the four bytes of the `uint32` size encoded as big endian.  Following the header is the payload, which is the specified number of bytes of `STREAM_TYPE`.  The simplest way to implement this protocol is the following:  1. Read 8 bytes. 2. Choose `stdout` or `stderr` depending on the first byte. 3. Extract the frame size from the last four bytes. 4. Read the extracted size and output it on the correct output. 5. Goto 1.  ### Stream format when using a TTY  When the TTY setting is enabled in [`POST /containers/create`](#operation/ContainerCreate), the stream is not multiplexed. The data exchanged over the hijacked connection is simply the raw data from the process PTY and client's `stdin`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**detach_keys** | Option<**String**> | Override the key sequence for detaching a container.Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.  |  |
**logs** | Option<**bool**> | Replay previous logs from the container.  This is useful for attaching to a container that has started and you want to output everything since the container started.  If `stream` is also enabled, once all the previous output has been returned, it will seamlessly transition into streaming current output.  |  |[default to false]
**stream** | Option<**bool**> | Stream attached streams from the time the request was made onwards.  |  |[default to false]
**stdin** | Option<**bool**> | Attach to `stdin` |  |[default to false]
**stdout** | Option<**bool**> | Attach to `stdout` |  |[default to false]
**stderr** | Option<**bool**> | Attach to `stderr` |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.docker.raw-stream, application/vnd.docker.multiplexed-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_attach_websocket

> container_attach_websocket(id, detach_keys, logs, stream, stdin, stdout, stderr)
Attach to a container via a websocket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**detach_keys** | Option<**String**> | Override the key sequence for detaching a container.Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,`, or `_`.  |  |
**logs** | Option<**bool**> | Return logs |  |[default to false]
**stream** | Option<**bool**> | Return stream |  |[default to false]
**stdin** | Option<**bool**> | Attach to `stdin` |  |[default to false]
**stdout** | Option<**bool**> | Attach to `stdout` |  |[default to false]
**stderr** | Option<**bool**> | Attach to `stderr` |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_changes

> Vec<models::FilesystemChange> container_changes(id)
Get changes on a container’s filesystem

Returns which files in a container's filesystem have been added, deleted, or modified. The `Kind` of modification can be one of:  - `0`: Modified (\"C\") - `1`: Added (\"A\") - `2`: Deleted (\"D\") 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |

### Return type

[**Vec<models::FilesystemChange>**](FilesystemChange.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_create

> models::ContainerCreateResponse container_create(body, name, platform)
Create a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ContainerCreateRequest**](ContainerCreateRequest.md) | Container to create | [required] |
**name** | Option<**String**> | Assign the specified name to the container. Must match `/?[a-zA-Z0-9][a-zA-Z0-9_.-]+`.  |  |
**platform** | Option<**String**> | Platform in the format `os[/arch[/variant]]` used for image lookup.  When specified, the daemon checks if the requested image is present in the local image cache with the given OS and Architecture, and otherwise returns a `404` status.  If the option is not set, the host's native OS and Architecture are used to look up the image in the image cache. However, if no platform is passed and the given image does exist in the local image cache, but its OS or architecture does not match, the container is created with the available image, and a warning is added to the `Warnings` field in the response, for example;      WARNING: The requested image's platform (linux/arm64/v8) does not              match the detected host platform (linux/amd64) and no              specific platform was requested  |  |

### Return type

[**models::ContainerCreateResponse**](ContainerCreateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_delete

> container_delete(id, v, force, link)
Remove a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**v** | Option<**bool**> | Remove anonymous volumes associated with the container. |  |[default to false]
**force** | Option<**bool**> | If the container is running, kill it before removing it. |  |[default to false]
**link** | Option<**bool**> | Remove the specified link associated with the container. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_export

> container_export(id)
Export a container

Export the contents of a container as a tarball.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_inspect

> models::ContainerInspectResponse container_inspect(id, size)
Inspect a container

Return low-level information about a container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**size** | Option<**bool**> | Return the size of container as fields `SizeRw` and `SizeRootFs` |  |[default to false]

### Return type

[**models::ContainerInspectResponse**](ContainerInspectResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_kill

> container_kill(id, signal)
Kill a container

Send a POSIX signal to a container, defaulting to killing to the container. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**signal** | Option<**String**> | Signal to send to the container as an integer or string (e.g. `SIGINT`).  |  |[default to SIGKILL]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_list

> Vec<models::ContainerSummary> container_list(all, limit, size, filters)
List containers

Returns a list of containers. For details on the format, see the [inspect endpoint](#operation/ContainerInspect).  Note that it uses a different, smaller representation of a container than inspecting a single container. For example, the list of linked containers is not propagated . 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Return all containers. By default, only running containers are shown.  |  |[default to false]
**limit** | Option<**i32**> | Return this number of most recently created containers, including non-running ones.  |  |
**size** | Option<**bool**> | Return the size of container as fields `SizeRw` and `SizeRootFs`.  |  |[default to false]
**filters** | Option<**String**> | Filters to process on the container list, encoded as JSON (a `map[string][]string`). For example, `{\"status\": [\"paused\"]}` will only return paused containers.  Available filters:  - `ancestor`=(`<image-name>[:<tag>]`, `<image id>`, or `<image@digest>`) - `before`=(`<container id>` or `<container name>`) - `expose`=(`<port>[/<proto>]`|`<startport-endport>/[<proto>]`) - `exited=<int>` containers with exit code of `<int>` - `health`=(`starting`|`healthy`|`unhealthy`|`none`) - `id=<ID>` a container's ID - `isolation=`(`default`|`process`|`hyperv`) (Windows daemon only) - `is-task=`(`true`|`false`) - `label=key` or `label=\"key=value\"` of a container label - `name=<name>` a container's name - `network`=(`<network id>` or `<network name>`) - `publish`=(`<port>[/<proto>]`|`<startport-endport>/[<proto>]`) - `since`=(`<container id>` or `<container name>`) - `status=`(`created`|`restarting`|`running`|`removing`|`paused`|`exited`|`dead`) - `volume`=(`<volume name>` or `<mount point destination>`)  |  |

### Return type

[**Vec<models::ContainerSummary>**](ContainerSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_logs

> std::path::PathBuf container_logs(id, follow, stdout, stderr, since, until, timestamps, tail)
Get container logs

Get `stdout` and `stderr` logs from a container.  Note: This endpoint works only for containers with the `json-file` or `journald` logging driver. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**follow** | Option<**bool**> | Keep connection after returning logs. |  |[default to false]
**stdout** | Option<**bool**> | Return logs from `stdout` |  |[default to false]
**stderr** | Option<**bool**> | Return logs from `stderr` |  |[default to false]
**since** | Option<**i32**> | Only return logs since this time, as a UNIX timestamp |  |[default to 0]
**until** | Option<**i32**> | Only return logs before this time, as a UNIX timestamp |  |[default to 0]
**timestamps** | Option<**bool**> | Add timestamps to every log line |  |[default to false]
**tail** | Option<**String**> | Only return this number of log lines from the end of the logs. Specify as an integer or `all` to output all log lines.  |  |[default to all]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.docker.raw-stream, application/vnd.docker.multiplexed-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_pause

> container_pause(id)
Pause a container

Use the freezer cgroup to suspend all processes in a container.  Traditionally, when suspending a process the `SIGSTOP` signal is used, which is observable by the process being suspended. With the freezer cgroup the process is unaware, and unable to capture, that it is being suspended, and subsequently resumed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_prune

> models::ContainerPruneResponse container_prune(filters)
Delete stopped containers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | Filters to process on the prune list, encoded as JSON (a `map[string][]string`).  Available filters: - `until=<timestamp>` Prune containers created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time. - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune containers with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**models::ContainerPruneResponse**](ContainerPruneResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_rename

> container_rename(id, name)
Rename a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**name** | **String** | New name for the container | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_resize

> container_resize(id, h, w)
Resize a container TTY

Resize the TTY for a container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**h** | Option<**i32**> | Height of the TTY session in characters |  |
**w** | Option<**i32**> | Width of the TTY session in characters |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_restart

> container_restart(id, signal, t)
Restart a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**signal** | Option<**String**> | Signal to send to the container as an integer or string (e.g. `SIGINT`).  |  |
**t** | Option<**i32**> | Number of seconds to wait before killing the container |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_start

> container_start(id, detach_keys)
Start a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**detach_keys** | Option<**String**> | Override the key sequence for detaching a container. Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_stats

> serde_json::Value container_stats(id, stream, one_shot)
Get container stats based on resource usage

This endpoint returns a live stream of a container’s resource usage statistics.  The `precpu_stats` is the CPU statistic of the *previous* read, and is used to calculate the CPU usage percentage. It is not an exact copy of the `cpu_stats` field.  If either `precpu_stats.online_cpus` or `cpu_stats.online_cpus` is nil then for compatibility with older daemons the length of the corresponding `cpu_usage.percpu_usage` array should be used.  On a cgroup v2 host, the following fields are not set * `blkio_stats`: all fields other than `io_service_bytes_recursive` * `cpu_stats`: `cpu_usage.percpu_usage` * `memory_stats`: `max_usage` and `failcnt` Also, `memory_stats.stats` fields are incompatible with cgroup v1.  To calculate the values shown by the `stats` command of the docker cli tool the following formulas can be used: * used_memory = `memory_stats.usage - memory_stats.stats.cache` * available_memory = `memory_stats.limit` * Memory usage % = `(used_memory / available_memory) * 100.0` * cpu_delta = `cpu_stats.cpu_usage.total_usage - precpu_stats.cpu_usage.total_usage` * system_cpu_delta = `cpu_stats.system_cpu_usage - precpu_stats.system_cpu_usage` * number_cpus = `lenght(cpu_stats.cpu_usage.percpu_usage)` or `cpu_stats.online_cpus` * CPU usage % = `(cpu_delta / system_cpu_delta) * number_cpus * 100.0` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**stream** | Option<**bool**> | Stream the output. If false, the stats will be output once and then it will disconnect.  |  |[default to true]
**one_shot** | Option<**bool**> | Only get a single stat instead of waiting for 2 cycles. Must be used with `stream=false`.  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_stop

> container_stop(id, signal, t)
Stop a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**signal** | Option<**String**> | Signal to send to the container as an integer or string (e.g. `SIGINT`).  |  |
**t** | Option<**i32**> | Number of seconds to wait before killing the container |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_top

> models::ContainerTopResponse container_top(id, ps_args)
List processes running inside a container

On Unix systems, this is done by running the `ps` command. This endpoint is not supported on Windows. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**ps_args** | Option<**String**> | The arguments to pass to `ps`. For example, `aux` |  |[default to -ef]

### Return type

[**models::ContainerTopResponse**](ContainerTopResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_unpause

> container_unpause(id)
Unpause a container

Resume a container which has been paused.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_update

> models::ContainerUpdateResponse container_update(id, update)
Update a container

Change various configuration options of a container without having to recreate it. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**update** | [**ContainerUpdateRequest**](ContainerUpdateRequest.md) |  | [required] |

### Return type

[**models::ContainerUpdateResponse**](ContainerUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_wait

> models::ContainerWaitResponse container_wait(id, condition)
Wait for a container

Block until a container stops, then returns the exit code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**condition** | Option<**String**> | Wait until a container state reaches the given condition.  Defaults to `not-running` if omitted or empty.  |  |[default to not-running]

### Return type

[**models::ContainerWaitResponse**](ContainerWaitResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_container_archive

> put_container_archive(id, path, input_stream, no_overwrite_dir_non_dir, copy_uidgid)
Extract an archive of files or folders to a directory in a container

Upload a tar archive to be extracted to a path in the filesystem of container id. `path` parameter is asserted to be a directory. If it exists as a file, 400 error will be returned with message \"not a directory\". 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or name of the container | [required] |
**path** | **String** | Path to a directory in the container to extract the archive’s contents into.  | [required] |
**input_stream** | **std::path::PathBuf** | The input stream must be a tar archive compressed with one of the following algorithms: `identity` (no compression), `gzip`, `bzip2`, or `xz`.  | [required] |
**no_overwrite_dir_non_dir** | Option<**String**> | If `1`, `true`, or `True` then it will be an error if unpacking the given content would cause an existing directory to be replaced with a non-directory and vice versa.  |  |
**copy_uidgid** | Option<**String**> | If `1`, `true`, then it will copy UID/GID maps to the dest file or dir  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-tar, application/octet-stream
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

