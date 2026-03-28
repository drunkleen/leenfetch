pub mod cache;
pub mod config;
pub mod core;
pub mod modules;
pub mod system_info;
#[cfg(test)]
pub mod test_utils;

pub use system_info::{SystemInfo, PROTOCOL_VERSION};

use anyhow::Result;
use cache::Cache;
use config::settings::Config;
use core::Core;
use std::sync::{LazyLock, Mutex};

static SYSTEM_INFO_CACHE: LazyLock<Cache<SystemInfo>> = LazyLock::new(|| Cache::new(5));
static CACHE_MUTEX: Mutex<()> = Mutex::new(());

/// Collects system information using the provided configuration and returns the stable JSON model.
/// Results are cached for 5 seconds to avoid redundant computation on quick successive calls.
pub fn gather_system_info(config: &Config) -> Result<SystemInfo> {
    // Use a lock to prevent cache stampede
    let _lock = CACHE_MUTEX.lock().unwrap();

    // Create a cache key based on config (flags and layout)
    let cache_key = format!("{:?}_{:?}", config.flags, config.layout.len());

    // Try to get from cache
    let system_info = SYSTEM_INFO_CACHE.get_or_compute(&cache_key, || {
        let mut effective = config.clone();
        if effective.layout.is_empty() {
            effective.layout = crate::config::default_layout();
        }

        let core = Core::new_with(effective.flags.clone(), effective.layout.clone());
        let data = core.collect_data();
        SystemInfo::from(data)
    });

    Ok(system_info.clone())
}

/// Clears the system info cache. Useful for testing or when fresh data is needed.
pub fn clear_system_info_cache() {
    SYSTEM_INFO_CACHE.clear();
}
