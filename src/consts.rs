use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct InfoPanel {
    pub name: String,
    pub os: String,
    pub host_ipaddress: String,
    pub uptime: String,
    pub resolution: String,
    pub cpu: String,
    pub gpu: String,
    pub ram_used: u64,
    pub ram_total: u64,
    pub memory_used: u64,
    pub memory_total: u64,
}