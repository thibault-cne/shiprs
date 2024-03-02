use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerConfig {
    pub hostname: String,
    pub domainname: String,
    pub user: String,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub attach_stderr: bool,
    pub exposed_ports: Option<HashMap<String, HashMap<String, String>>>,
    pub tty: bool,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub env: Vec<String>,
    pub cmd: Option<Vec<String>>,
    pub args_escaped: Option<bool>,
    pub image: String,
    pub volumes: Option<HashMap<String, HashMap<String, String>>>,
    pub working_dir: String,
    pub entrypoint: Option<Vec<String>>,
    pub network_disabled: Option<bool>,
    pub mac_address: Option<String>,
    pub on_build: Option<Vec<String>>,
    pub labels: Option<HashMap<String, String>>,
    pub stop_signal: Option<String>,
    pub stop_timeout: Option<u32>,
    pub shell: Option<Vec<String>>,
}
