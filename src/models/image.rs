use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::model;

model! {
    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ContainerConfig {
        pub hostname: String,
        pub domainname: String,
        pub user: String,
        pub attach_stdin: bool,
        pub attach_stdout: bool,
        pub attach_stderr: bool,
        pub exposed_ports: HashMap<String, HashMap<String, String>>,
        pub tty: bool,
        pub open_stdin: bool,
        pub stdin_once: bool,
        pub env: Vec<String>,
        pub cmd: Vec<String>,
        pub args_escaped: bool,
        pub image: String,
        pub volumes: HashMap<String, HashMap<String, String>>,
        pub working_dir: String,
        pub entrypoint: Vec<String>,
        pub network_disabled: bool,
        pub mac_address: String,
        pub on_build: Vec<String>,
        pub labels: HashMap<String, String>,
        pub stop_signal: String,
        pub stop_timeout: u32,
        pub shell: Vec<String>,
    }

}
