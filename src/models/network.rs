use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::model;

type PortDescription = HashMap<String, Option<Vec<HashMap<String, String>>>>;

model! {
    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct NetworkSettings {
        pub bridge: String,
        pub gateway: String,
        #[serde(rename = "IPAddress")]
        pub ip_address: String,
        #[serde(rename = "IPPrefixLen")]
        pub ip_prefix_len: u64,
        pub mac_address: String,
        pub ports: PortDescription,
        pub networks: HashMap<String, NetworkEntry>,
    }
}

model! {
    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct NetworkEntry {
        #[serde(rename = "NetworkID")]
        pub network_id: String,
        #[serde(rename = "EndpointID")]
        pub endpoint_id: String,
        pub gateway: String,
        #[serde(rename = "IPAddress")]
        pub ip_address: String,
        #[serde(rename = "IPPrefixLen")]
        pub ip_prefix_len: u64,
        #[serde(rename = "IPv6Gateway")]
        pub ipv6_gateway: String,
        #[serde(rename = "GlobalIPv6Address")]
        pub global_ipv6_address: String,
        #[serde(rename = "GlobalIPv6PrefixLen")]
        pub global_ipv6_prefix_len: u64,
        pub mac_address: String,
        pub links: Vec<String>,
        pub aliases: Vec<String>,
        #[serde(rename = "IPAMConfig")]
        pub ipam_config: EndpointIPAMConfig,
    }
}

model! {
    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[allow(clippy::upper_case_acronyms)]
    pub struct EndpointIPAMConfig {
        #[serde(rename = "IPv4Address")]
        pub ipv4_address: String,
        #[serde(rename = "IPv6Address")]
        pub ipv6_address: String,
        #[serde(rename = "LinkLocalIPs")]
        pub link_local_ips: Vec<String>,
    }
}
