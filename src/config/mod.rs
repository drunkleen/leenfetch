pub mod defaults;
pub mod settings;

use self::{
    defaults::{
        CLASSIC_NEOFETCH_CONFIG, CLEAN_MONO_CONFIG, DEFAULT_CONFIG, HARDWARE_HEAVY_CONFIG,
        MINIMAL_CONFIG, NEOFETCH_CONFIG, SCREENSHOT_CONFIG, SYSTEM_ADMIN_CONFIG, TINY_CONFIG,
    },
    settings::{Config, Flags, LayoutItem},
};
use dirs::config_dir;
use json5;
use once_cell::sync::Lazy;
use std::io::Write;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    fs::{self, File},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigPreset {
    Default,
    Neofetch,
    ClassicNeofetch,
    Minimal,
    Screenshot,
    HardwareHeavy,
    SystemAdmin,
    CleanMono,
    Tiny,
}

static DEFAULT_CONFIG_CACHE: Lazy<Config> = Lazy::new(|| {
    json5::from_str(DEFAULT_CONFIG)
        .unwrap_or_else(|e| panic!("Built-in default config is invalid JSON: {e}"))
});

static NEOFETCH_CONFIG_CACHE: Lazy<Config> = Lazy::new(|| {
    json5::from_str(NEOFETCH_CONFIG)
        .unwrap_or_else(|e| panic!("Built-in neofetch config is invalid JSON: {e}"))
});

static CLASSIC_NEOFETCH_CONFIG_CACHE: Lazy<Config> = Lazy::new(|| {
    json5::from_str(CLASSIC_NEOFETCH_CONFIG)
        .unwrap_or_else(|e| panic!("Built-in classic neofetch config is invalid JSON: {e}"))
});

static MINIMAL_CONFIG_CACHE: Lazy<Config> = Lazy::new(|| {
    json5::from_str(MINIMAL_CONFIG)
        .unwrap_or_else(|e| panic!("Built-in minimal config is invalid JSON: {e}"))
});

static SCREENSHOT_CONFIG_CACHE: Lazy<Config> = Lazy::new(|| {
    json5::from_str(SCREENSHOT_CONFIG)
        .unwrap_or_else(|e| panic!("Built-in screenshot config is invalid JSON: {e}"))
});

static HARDWARE_HEAVY_CONFIG_CACHE: Lazy<Config> = Lazy::new(|| {
    json5::from_str(HARDWARE_HEAVY_CONFIG)
        .unwrap_or_else(|e| panic!("Built-in hardware-heavy config is invalid JSON: {e}"))
});

static SYSTEM_ADMIN_CONFIG_CACHE: Lazy<Config> = Lazy::new(|| {
    json5::from_str(SYSTEM_ADMIN_CONFIG)
        .unwrap_or_else(|e| panic!("Built-in system-admin config is invalid JSON: {e}"))
});

static CLEAN_MONO_CONFIG_CACHE: Lazy<Config> = Lazy::new(|| {
    json5::from_str(CLEAN_MONO_CONFIG)
        .unwrap_or_else(|e| panic!("Built-in clean mono config is invalid JSON: {e}"))
});

static TINY_CONFIG_CACHE: Lazy<Config> = Lazy::new(|| {
    json5::from_str(TINY_CONFIG)
        .unwrap_or_else(|e| panic!("Built-in tiny config is invalid JSON: {e}"))
});

/// Loads the unified configuration from `config.jsonc`.
fn load_config() -> Result<Config, String> {
    let path = config_file("config.jsonc");
    let data = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read config.jsonc ({}): {}", path.display(), e))?;
    load_config_from_str(&data)
}

fn load_config_from_str(data: &str) -> Result<Config, String> {
    json5::from_str(data).map_err(|e| format!("Invalid JSONC in config.jsonc: {}", e))
}

pub fn load_preset_config(preset: ConfigPreset) -> Config {
    match preset {
        ConfigPreset::Default => default_config(),
        ConfigPreset::Neofetch => NEOFETCH_CONFIG_CACHE.clone(),
        ConfigPreset::ClassicNeofetch => CLASSIC_NEOFETCH_CONFIG_CACHE.clone(),
        ConfigPreset::Minimal => MINIMAL_CONFIG_CACHE.clone(),
        ConfigPreset::Screenshot => SCREENSHOT_CONFIG_CACHE.clone(),
        ConfigPreset::HardwareHeavy => HARDWARE_HEAVY_CONFIG_CACHE.clone(),
        ConfigPreset::SystemAdmin => SYSTEM_ADMIN_CONFIG_CACHE.clone(),
        ConfigPreset::CleanMono => CLEAN_MONO_CONFIG_CACHE.clone(),
        ConfigPreset::Tiny => TINY_CONFIG_CACHE.clone(),
    }
}

pub fn preset_label(preset: ConfigPreset) -> &'static str {
    match preset {
        ConfigPreset::Default => "leenfetch default",
        ConfigPreset::Neofetch => "neofetch-compatible",
        ConfigPreset::ClassicNeofetch => "classic neofetch",
        ConfigPreset::Minimal => "minimal",
        ConfigPreset::Screenshot => "screenshot",
        ConfigPreset::HardwareHeavy => "hardware-heavy",
        ConfigPreset::SystemAdmin => "system-admin",
        ConfigPreset::CleanMono => "clean mono",
        ConfigPreset::Tiny => "tiny",
    }
}

pub fn preset_content(preset: ConfigPreset) -> &'static str {
    match preset {
        ConfigPreset::Default => DEFAULT_CONFIG,
        ConfigPreset::Neofetch => NEOFETCH_CONFIG,
        ConfigPreset::ClassicNeofetch => CLASSIC_NEOFETCH_CONFIG,
        ConfigPreset::Minimal => MINIMAL_CONFIG,
        ConfigPreset::Screenshot => SCREENSHOT_CONFIG,
        ConfigPreset::HardwareHeavy => HARDWARE_HEAVY_CONFIG,
        ConfigPreset::SystemAdmin => SYSTEM_ADMIN_CONFIG,
        ConfigPreset::CleanMono => CLEAN_MONO_CONFIG,
        ConfigPreset::Tiny => TINY_CONFIG,
    }
}

pub fn config_path() -> PathBuf {
    config_file("config.jsonc")
}

pub fn config_file_path(name: &str) -> PathBuf {
    config_file(name)
}

pub fn config_exists() -> bool {
    config_path().exists()
}

/// Loads configuration from a custom path when provided.
pub fn load_config_at(path: Option<&str>) -> Result<Config, String> {
    match path {
        Some(custom_path) => {
            let data = fs::read_to_string(custom_path)
                .map_err(|err| format!("Failed to read config at {}: {}", custom_path, err))?;
            load_config_from_str(&data)
        }
        None => load_config(),
    }
}

/// Returns the built-in default configuration.
pub fn default_config() -> Config {
    DEFAULT_CONFIG_CACHE.clone()
}

/// Returns the built-in default layout section.
pub fn default_layout() -> Vec<LayoutItem> {
    default_config().layout
}

/// Loads the effective configuration, falling back to built-in defaults on any error.
pub fn load_effective_config_at(path: Option<&str>) -> Config {
    let mut config = load_config_at(path).unwrap_or_else(|_| default_config());
    if config.layout.is_empty() {
        config.layout = default_layout();
    }
    config
}

/// Loads the effective configuration from the default path, falling back to built-in defaults.
pub fn load_effective_config() -> Config {
    load_effective_config_at(None)
}

/// Loads the modules section from `config.jsonc`.
///
/// # Returns
///
/// A `Vec<LayoutItem>` containing the loaded module configuration.
pub fn load_print_layout() -> Vec<LayoutItem> {
    let layout = load_config()
        .unwrap_or_else(|e| {
            eprintln!("leenfetch: config error: {e}; using defaults");
            default_config()
        })
        .layout;
    if layout.is_empty() {
        default_layout()
    } else {
        layout
    }
}

/// Loads the configuration flags from `config.jsonc`.
///
/// # Returns
///
/// A `Flags` struct containing the loaded configuration.
pub fn load_flags() -> Flags {
    load_config()
        .unwrap_or_else(|e| {
            eprintln!("leenfetch: config error: {e}; using defaults");
            default_config()
        })
        .flags
}

/// Generates the default unified configuration file.
///
/// Writes `config.jsonc` with the default contents. Returns a map with the filename
/// and whether the operation succeeded, matching the previous multi-file API.
pub fn generate_config_files() -> HashMap<String, bool> {
    let mut results = HashMap::new();

    let result = write_config_preset(ConfigPreset::Default).is_ok();
    results.insert("config.jsonc".to_string(), result);

    results
}

pub fn write_config_preset(preset: ConfigPreset) -> std::io::Result<()> {
    save_to_config_file("config.jsonc", preset_content(preset))
}

/// Saves the provided content to a configuration file with the specified file name.
///
/// The function ensures that the directory for the file exists, creating it if necessary.
/// It then writes the content to the file, overwriting any existing content.
///
/// # Arguments
///
/// * `file_name` - A string slice that holds the name of the file to be created or overwritten.
/// * `content` - A string slice containing the content to write to the file.
///
/// # Returns
///
/// A `Result` which is:
///
/// * `Ok(())` if the operation is successful.
/// * `Err` if an error occurs during directory creation or file writing.
fn save_to_config_file(file_name: &str, content: &str) -> std::io::Result<()> {
    let path = config_file(file_name);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

/// Deletes the generated configuration file.
///
/// This function is used in the `--clean-config` flag, and it removes the default config file.
/// It returns a HashMap where the key is the config file name and the value indicates
/// whether the file was deleted.
pub fn delete_config_files() -> HashMap<String, bool> {
    let mut results = HashMap::new();

    let file = "config.jsonc";
    let result = delete_config_file(file).is_ok();
    results.insert(file.to_string(), result);

    results
}

/// Deletes the given configuration file, returning an error if the operation fails.
///
/// This function is used by `delete_config_files` to remove the generated configuration.
/// It does not report an error if the file does not exist.
fn delete_config_file(file_name: &str) -> std::io::Result<()> {
    let path = config_file(file_name);

    if path.exists() {
        std::fs::remove_file(path)?;
    }

    Ok(())
}

/// Returns a `PathBuf` for the configuration file with the given `name`.
///
/// If `XDG_CONFIG_HOME` is set, the function will return a path in that directory.
/// If `XDG_CONFIG_HOME` is not set, the function will return a path in the current directory.
///
/// The returned path will have the "leenfetch" directory as its parent, and the given `name` as its file name.
fn config_file(name: &str) -> PathBuf {
    config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("leenfetch")
        .join(name)
}

/// Ensures that the configuration file exists.
///
/// This function creates the `leenfetch` directory and the config file if they do not exist.
/// It will not overwrite an existing config file.
///
/// Returns a `HashMap` where the key is the config file name and the value indicates
/// whether the file was created.
pub fn ensure_config_files_exist() -> HashMap<String, bool> {
    let mut results = HashMap::new();

    let filename = "config.jsonc";
    let created = ensure_config_file_exists(filename, DEFAULT_CONFIG).unwrap_or(false);
    results.insert(filename.to_string(), created);

    results
}

/// Ensures that the configuration file with the given `file_name` exists.
///
/// If the file does not exist, the function will create it with the given `default_content`.
/// If the file already exists, the function will return `false` without modifying it.
///
/// # Returns
///
/// A `Result` which is `Ok(true)` if the file was created, or `Ok(false)` if the file already existed.
/// If an error occurs while attempting to create the file, the function will return `Err`.
fn ensure_config_file_exists(file_name: &str, default_content: &str) -> std::io::Result<bool> {
    let path = config_file(file_name);

    if path.exists() {
        return Ok(false); // Already exists
    }

    save_to_config_file(file_name, default_content)?;
    Ok(true) // Created
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::EnvLock;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn load_effective_config_falls_back_to_defaults_for_invalid_custom_file() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("leenfetch_invalid_config_{unique}.jsonc"));
        fs::write(&path, "{ invalid jsonc").unwrap();

        let config = load_effective_config_at(path.to_str());
        assert!(!config.layout.is_empty());
        assert_eq!(
            config.flags.ascii_distro,
            default_config().flags.ascii_distro
        );

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn preset_configs_parse_successfully() {
        for preset in [
            ConfigPreset::Default,
            ConfigPreset::Neofetch,
            ConfigPreset::ClassicNeofetch,
            ConfigPreset::Minimal,
            ConfigPreset::Screenshot,
            ConfigPreset::HardwareHeavy,
            ConfigPreset::SystemAdmin,
            ConfigPreset::CleanMono,
            ConfigPreset::Tiny,
        ] {
            let config = load_preset_config(preset);
            assert!(
                !config.layout.is_empty(),
                "{} had an empty layout",
                preset_label(preset)
            );
        }
    }

    #[test]
    fn write_config_preset_overwrites_existing_file() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("leenfetch_config_write_{unique}"));
        fs::create_dir_all(&temp_dir).unwrap();

        let env = EnvLock::acquire(&["XDG_CONFIG_HOME"]);
        env.set_var("XDG_CONFIG_HOME", temp_dir.to_str().unwrap());

        write_config_preset(ConfigPreset::Tiny).unwrap();

        let written = fs::read_to_string(config_path()).unwrap();
        assert!(written.contains("\"ascii_distro\": \"off\""));
        assert!(written.contains("\"modules\""));

        fs::remove_dir_all(temp_dir).unwrap();
    }
}
