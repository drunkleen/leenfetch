pub mod config;
pub mod core;
pub mod modules;
pub mod system_info;
#[cfg(test)]
pub mod test_utils;

pub use system_info::{PROTOCOL_VERSION, SystemInfo};

use anyhow::Result;
use config::settings::Config;
use core::Core;

/// Collects system information using the provided configuration and returns the stable JSON model.
pub fn gather_system_info(config: &Config) -> Result<SystemInfo> {
    let mut effective = config.clone();
    if effective.layout.is_empty() {
        effective.layout = crate::config::default_layout();
    }

    let core = Core::new_with(effective.flags.clone(), effective.layout.clone());
    let data = core.collect_data();
    Ok(SystemInfo::from(data))
}
