use serde::{Deserialize, Serialize};

use crate::{config::settings::Config, core::Data, modules::enums::SongInfo};

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

/// Remote SSH payload containing the effective remote config and collected system data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSshPayload {
    pub config: Config,
    pub system_info: SystemInfo,
}

impl RemoteSshPayload {
    pub fn new(config: Config, system_info: SystemInfo) -> Self {
        Self {
            config,
            system_info,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::default_config;

    fn sample_system_info() -> SystemInfo {
        SystemInfo {
            protocol_version: PROTOCOL_VERSION,
            username: Some("anakin".to_string()),
            hostname: Some("tattooine".to_string()),
            host: Some("anakin@tattooine".to_string()),
            os: Some("Linux".to_string()),
            distro: Some("Arch Linux".to_string()),
            model: Some("X1".to_string()),
            kernel: Some("6.0.0".to_string()),
            os_age: None,
            uptime: None,
            packages: None,
            shell: None,
            wm: None,
            de: None,
            cpu: None,
            gpu: None,
            memory: None,
            disk: None,
            resolution: None,
            theme: None,
            battery: None,
            song: None,
            colors: None,
        }
    }

    #[test]
    fn remote_ssh_payload_round_trips() {
        let payload = RemoteSshPayload::new(default_config(), sample_system_info());
        let encoded = serde_json::to_string(&payload).unwrap();
        let decoded: RemoteSshPayload = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded.system_info.username.as_deref(), Some("anakin"));
        assert_eq!(decoded.system_info.hostname.as_deref(), Some("tattooine"));
        assert!(!decoded.config.layout.is_empty());
    }
}
