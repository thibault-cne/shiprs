# Resources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_shares** | Option<**i32**> | An integer value representing this container's relative CPU weight versus other containers.  | [optional]
**memory** | Option<**i64**> | Memory limit in bytes. | [optional][default to 0]
**cgroup_parent** | Option<**String**> | Path to `cgroups` under which the container's `cgroup` is created. If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process. Cgroups are created if they do not already exist.  | [optional]
**blkio_weight** | Option<**i32**> | Block IO weight (relative weight). | [optional]
**blkio_weight_device** | Option<[**Vec<models::ResourcesBlkioWeightDeviceInner>**](Resources_BlkioWeightDevice_inner.md)> | Block IO weight (relative device weight) in the form:  ``` [{\"Path\": \"device_path\", \"Weight\": weight}] ```  | [optional]
**blkio_device_read_bps** | Option<[**Vec<models::ThrottleDevice>**](ThrottleDevice.md)> | Limit read rate (bytes per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```  | [optional]
**blkio_device_write_bps** | Option<[**Vec<models::ThrottleDevice>**](ThrottleDevice.md)> | Limit write rate (bytes per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```  | [optional]
**blkio_device_read_i_ops** | Option<[**Vec<models::ThrottleDevice>**](ThrottleDevice.md)> | Limit read rate (IO per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```  | [optional]
**blkio_device_write_i_ops** | Option<[**Vec<models::ThrottleDevice>**](ThrottleDevice.md)> | Limit write rate (IO per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```  | [optional]
**cpu_period** | Option<**i64**> | The length of a CPU period in microseconds. | [optional]
**cpu_quota** | Option<**i64**> | Microseconds of CPU time that the container can get in a CPU period.  | [optional]
**cpu_realtime_period** | Option<**i64**> | The length of a CPU real-time period in microseconds. Set to 0 to allocate no time allocated to real-time tasks.  | [optional]
**cpu_realtime_runtime** | Option<**i64**> | The length of a CPU real-time runtime in microseconds. Set to 0 to allocate no time allocated to real-time tasks.  | [optional]
**cpuset_cpus** | Option<**String**> | CPUs in which to allow execution (e.g., `0-3`, `0,1`).  | [optional]
**cpuset_mems** | Option<**String**> | Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.  | [optional]
**devices** | Option<[**Vec<models::DeviceMapping>**](DeviceMapping.md)> | A list of devices to add to the container. | [optional]
**device_cgroup_rules** | Option<**Vec<String>**> | a list of cgroup rules to apply to the container | [optional]
**device_requests** | Option<[**Vec<models::DeviceRequest>**](DeviceRequest.md)> | A list of requests for devices to be sent to device drivers.  | [optional]
**kernel_memory_tcp** | Option<**i64**> | Hard limit for kernel TCP buffer memory (in bytes). Depending on the OCI runtime in use, this option may be ignored. It is no longer supported by the default (runc) runtime.  This field is omitted when empty.  | [optional]
**memory_reservation** | Option<**i64**> | Memory soft limit in bytes. | [optional]
**memory_swap** | Option<**i64**> | Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.  | [optional]
**memory_swappiness** | Option<**i64**> | Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.  | [optional]
**nano_cpus** | Option<**i64**> | CPU quota in units of 10<sup>-9</sup> CPUs. | [optional]
**oom_kill_disable** | Option<**bool**> | Disable OOM Killer for the container. | [optional]
**init** | Option<**bool**> | Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.  | [optional]
**pids_limit** | Option<**i64**> | Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null` to not change.  | [optional]
**ulimits** | Option<[**Vec<models::ResourcesUlimitsInner>**](Resources_Ulimits_inner.md)> | A list of resource limits to set in the container. For example:  ``` {\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048} ```  | [optional]
**cpu_count** | Option<**i64**> | The number of usable CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.  | [optional]
**cpu_percent** | Option<**i64**> | The usable percentage of the available CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.  | [optional]
**io_maximum_i_ops** | Option<**i64**> | Maximum IOps for the container system drive (Windows only) | [optional]
**io_maximum_bandwidth** | Option<**i64**> | Maximum IO in bytes per second for the container system drive (Windows only).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


