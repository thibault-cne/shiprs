use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use crate::datetime::datetime_from_unix_timestamp;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

use crate::{
    docker::Docker, error::Result, http::request::RequestBuilder, image::ContainerConfig,
    network::NetworkSettings,
};

pub struct Container<'docker> {
    docker: &'docker Docker,
    id: String,
}

impl<'docker> Container<'docker> {
    pub fn new<S: Into<String>>(docker: &'docker Docker, id: S) -> Self {
        Container {
            docker,
            id: id.into(),
        }
    }

    pub fn inspect(&self) -> Result<ContainerDetails> {
        let request = RequestBuilder::get(format!("/containers/{}/json", self.id)).build();
        let resp = self.docker.request(request)?;
        Ok(resp.into_body())
    }
}

pub struct Containers<'docker> {
    docker: &'docker Docker,
}

impl<'docker> Containers<'docker> {
    pub fn new(docker: &'docker Docker) -> Self {
        Containers { docker }
    }

    pub fn list(&self) -> Result<Vec<ContainerInfo>> {
        let request = RequestBuilder::get("/containers/json").build();
        let resp = self.docker.request(request)?;
        Ok(resp.into_body())
    }

    pub fn get<S: Into<String>>(&self, id: S) -> Container {
        Container::new(self.docker, id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_unix_timestamp")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: u64,
    pub command: String,
    pub id: String,
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    pub labels: HashMap<String, String>,
    pub names: Vec<String>,
    pub ports: Vec<Port>,
    pub state: String,
    pub status: String,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerDetails {
    pub id: String,
    #[cfg(feature = "chrono")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: String,
    pub path: String,
    pub args: Vec<String>,
    pub state: Option<State>,
    pub image: String,
    pub resolv_conf_path: String,
    pub hostname_path: String,
    pub hosts_path: String,
    pub log_path: String,
    pub name: String,
    pub restart_count: i64,
    pub driver: String,
    pub platform: String,
    pub mount_label: String,
    pub process_label: String,
    pub app_armor_profile: String,
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Option<Vec<String>>,
    pub host_config: HostConfig,
    pub graph_driver: GraphDriverData,
    pub size_rw: Option<i64>,
    pub mounts: Vec<MountPoint>,
    pub config: ContainerConfig,
    pub network_settings: NetworkSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GraphDriverData {
    pub name: String,
    pub data: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    pub target: String,
    pub source: String,
    #[serde(rename = "Type")]
    pub mode: String,
    pub read_only: Option<bool>,
    pub consistency: Option<String>,
    pub bind_options: Option<BindOptions>,
    pub volume_options: Option<VolumeOptions>,
    pub tmpfs_options: Option<TmpfsOptions>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MountPoint {
    #[serde(rename = "Type")]
    pub ty: String,
    pub name: Option<String>,
    pub source: String,
    pub destination: String,
    pub driver: Option<String>,
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    pub propagation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BindOptions {
    pub propagation: String,
    pub non_recursive: bool,
    pub create_mountpoint: bool,
    pub read_only_non_recursive: bool,
    pub read_only_force_recursive: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeOptions {
    pub no_copy: Option<bool>,
    pub labels: Option<HashMap<String, String>>,
    pub driver_config: Option<DriverConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DriverConfig {
    pub name: String,
    pub options: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TmpfsOptions {
    pub size_bytes: i64,
    pub mode: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub status: String,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    pub dead: bool,
    pub pid: u64,
    pub exit_code: u64,
    pub error: String,
    #[cfg(feature = "chrono")]
    pub started_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub started_at: String,
    #[cfg(feature = "chrono")]
    pub finished_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub finished_at: String,
    pub health: Option<Health>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Health {
    pub status: String,
    pub failing_streak: u64,
    pub log: Option<Vec<HealthcheckResult>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthcheckResult {
    pub start: String,
    pub end: String,
    pub exit_code: i64,
    pub output: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub cpu_shares: i64,
    pub memory: i64,
    pub cgroup_parent: String,
    pub blkio_weight: u16,
    pub blkio_weight_device: Option<Vec<ThrottleDevice>>,
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Option<Vec<ThrottleDevice>>,
    pub cpu_period: i64,
    pub cpu_quota: i64,
    pub cpu_realtime_period: i64,
    pub cpu_realtime_runtime: i64,
    pub cpuset_cpus: String,
    pub cpuset_mems: String,
    pub devices: Option<Vec<DeviceMapping>>,
    pub device_cgroup_rules: Option<String>,
    pub device_requests: Option<Vec<DeviceRequest>>,
    #[serde(rename = "KernelMemoryTCP")]
    pub kernel_memory_tcp: Option<i64>,
    pub memory_reservation: i64,
    pub memory_swap: i64,
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCPUs")]
    pub nano_cpus: Option<i64>,
    pub oom_kill_disable: Option<bool>,
    pub init: Option<bool>,
    pub pids_limit: Option<i64>,
    pub ulimits: Option<Vec<Ulimit>>,
    pub cpu_count: i64,
    pub cpu_percent: i64,
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_iops: Option<u64>,
    #[serde(rename = "IOMaximumBandwith")]
    pub io_maximum_bandwith: Option<u64>,
    pub binds: Vec<String>,
    #[serde(rename = "ContainerIDFile")]
    pub container_id_file: String,
    pub log_config: LogConfig,
    pub network_mode: String,
    pub port_bindings: PortMap,
    pub restart_policy: RestartPolicy,
    pub auto_remove: bool,
    pub volume_driver: String,
    pub volumes_from: Option<Vec<String>>,
    pub mounts: Vec<Mount>,
    pub cap_add: Option<Vec<String>>,
    pub cap_drop: Option<Vec<String>>,
    #[serde(rename = "CgroupnsMode")]
    pub c_groupns_mode: String,
    pub dns: Vec<String>,
    pub dns_options: Vec<String>,
    pub dns_search: Vec<String>,
    pub extra_hosts: Vec<String>,
    pub group_add: Option<Vec<String>>,
    pub ipc_mode: String,
    pub cgroup: String,
    pub links: Option<Vec<String>>,
    pub oom_score_adj: i64,
    pub pid_mode: String,
    pub privileged: bool,
    pub publish_all_ports: bool,
    pub readonly_rootfs: bool,
    pub security_opt: Option<Vec<String>>,
    pub storage_opt: Option<HashMap<String, String>>,
    pub tmpfs: Option<HashMap<String, String>>,
    #[serde(rename = "UTSMode")]
    pub uts_mode: String,
    pub userns_mode: String,
    pub shm_size: u64,
    pub sysctls: Option<HashMap<String, String>>,
    pub runtime: String,
    pub isolation: String,
    pub masked_paths: Vec<String>,
    pub readonly_paths: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ThrottleDevice {
    Weight { path: String, weight: u64 },
    Rate { path: String, rate: u64 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThrottleDeviceRate {
    pub path: String,
    pub rate: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestartPolicy {
    pub name: String,
    pub maximum_retry_count: u64,
}

pub type PortMap = HashMap<String, Option<Vec<PortBinding>>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortBinding {
    pub host_ip: String,
    pub host_port: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Config")]
    pub config: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ulimit {
    pub name: String,
    pub soft: u64,
    pub hard: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceMapping {
    pub path_on_host: String,
    pub path_in_container: String,
    pub cgroup_permissions: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceRequest {
    pub driver: String,
    pub count: u64,
    #[serde(rename = "DeviceIDs")]
    pub device_ids: Vec<String>,
    pub capabilities: Vec<String>,
    pub options: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub ip: Option<String>,
    pub private_port: u64,
    pub public_port: Option<u64>,
    #[serde(rename = "Type")]
    pub typ: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub read: String,
    pub networks: HashMap<String, NetworkStats>,
    pub memory_stats: MemoryStats,
    pub blkio_stats: BlkioStats,
    pub cpu_stats: CpuStats,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    pub rx_dropped: u64,
    pub rx_bytes: u64,
    pub rx_errors: u64,
    pub tx_packets: u64,
    pub tx_dropped: u64,
    pub rx_packets: u64,
    pub tx_errors: u64,
    pub tx_bytes: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryStats {
    pub max_usage: u64,
    pub usage: u64,
    pub failcnt: Option<u64>,
    pub limit: u64,
    pub stats: MemoryStat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryStat {
    pub total_pgmajfault: u64,
    pub cache: u64,
    pub mapped_file: u64,
    pub total_inactive_file: u64,
    pub pgpgout: u64,
    pub rss: u64,
    pub total_mapped_file: u64,
    pub writeback: u64,
    pub unevictable: u64,
    pub pgpgin: u64,
    pub total_unevictable: u64,
    pub pgmajfault: u64,
    pub total_rss: u64,
    pub total_rss_huge: u64,
    pub total_writeback: u64,
    pub total_inactive_anon: u64,
    pub rss_huge: u64,
    pub hierarchical_memory_limit: u64,
    pub hierarchical_memsw_limit: u64,
    pub total_pgfault: u64,
    pub total_active_file: u64,
    pub active_anon: u64,
    pub total_active_anon: u64,
    pub total_pgpgout: u64,
    pub total_cache: u64,
    pub inactive_anon: u64,
    pub active_file: u64,
    pub pgfault: u64,
    pub inactive_file: u64,
    pub total_pgpgin: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuStats {
    pub cpu_usage: CpuUsage,
    pub system_cpu_usage: u64,
    pub throttling_data: ThrottlingData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuUsage {
    pub percpu_usage: Vec<u64>,
    pub usage_in_usermode: u64,
    pub total_usage: u64,
    pub usage_in_kernelmode: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThrottlingData {
    pub periods: u64,
    pub throttled_periods: u64,
    pub throttled_time: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlkioStats {
    pub io_service_bytes_recursive: Vec<BlkioStat>,
    pub io_serviced_recursive: Vec<BlkioStat>,
    pub io_queue_recursive: Vec<BlkioStat>,
    pub io_service_time_recursive: Vec<BlkioStat>,
    pub io_wait_time_recursive: Vec<BlkioStat>,
    pub io_merged_recursive: Vec<BlkioStat>,
    pub io_time_recursive: Vec<BlkioStat>,
    pub sectors_recursive: Vec<BlkioStat>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlkioStat {
    pub major: u64,
    pub minor: u64,
    pub op: String,
    pub value: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Change {
    pub kind: u64,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Top {
    pub titles: Vec<String>,
    pub processes: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreateInfo {
    pub id: String,
    pub warnings: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Exit {
    pub status_code: u64,
}
