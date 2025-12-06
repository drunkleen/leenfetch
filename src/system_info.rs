use serde::{Deserialize, Serialize};

use crate::{core::Data, modules::enums::SongInfo};

pub const PROTOCOL_VERSION: u32 = 1;

/// Stable JSON-friendly representation of all collected system information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub protocol_version: u32,
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub host: Option<String>,
    pub os: Option<String>,
    pub distro: Option<String>,
    pub model: Option<String>,
    pub kernel: Option<String>,
    pub os_age: Option<String>,
    pub uptime: Option<String>,
    pub packages: Option<String>,
    pub shell: Option<String>,
    pub wm: Option<String>,
    pub de: Option<String>,
    pub cpu: Option<String>,
    pub gpu: Option<Vec<String>>,
    pub memory: Option<String>,
    pub disk: Option<Vec<(String, String)>>,
    pub resolution: Option<String>,
    pub theme: Option<String>,
    pub battery: Option<Vec<String>>,
    pub song: Option<SongInfo>,
    pub colors: Option<String>,
}

impl From<Data> for SystemInfo {
    fn from(value: Data) -> Self {
        let host = match (&value.username, &value.hostname) {
            (Some(user), Some(host)) => Some(format!("{user}@{host}")),
            (_, Some(host)) => Some(host.clone()),
            _ => None,
        };

        Self {
            protocol_version: PROTOCOL_VERSION,
            username: value.username,
            hostname: value.hostname,
            host,
            os: value.os,
            distro: value.distro,
            model: value.model,
            kernel: value.kernel,
            os_age: value.os_age,
            uptime: value.uptime,
            packages: value.packages,
            shell: value.shell,
            wm: value.wm,
            de: value.de,
            cpu: value.cpu,
            gpu: value.gpu,
            memory: value.memory,
            disk: value.disk,
            resolution: value.resolution,
            theme: value.theme,
            battery: value.battery,
            song: value.song,
            colors: value.colors,
        }
    }
}

impl From<&SystemInfo> for Data {
    fn from(value: &SystemInfo) -> Self {
        Data {
            username: value.username.clone(),
            hostname: value.hostname.clone(),
            os: value.os.clone(),
            distro: value.distro.clone(),
            model: value.model.clone(),
            kernel: value.kernel.clone(),
            os_age: value.os_age.clone(),
            uptime: value.uptime.clone(),
            packages: value.packages.clone(),
            shell: value.shell.clone(),
            wm: value.wm.clone(),
            de: value.de.clone(),
            cpu: value.cpu.clone(),
            gpu: value.gpu.clone(),
            memory: value.memory.clone(),
            disk: value.disk.clone(),
            resolution: value.resolution.clone(),
            theme: value.theme.clone(),
            battery: value.battery.clone(),
            song: value.song.clone(),
            colors: value.colors.clone(),
        }
    }
}
