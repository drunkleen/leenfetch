pub mod defaults;
pub mod settings;

use self::{
    defaults::DEFAULT_CONFIG,
    settings::{Config, Flags, LayoutItem},
};
use dirs::config_dir;
use json5;
use std::io::Write;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    fs::{self, File},
};

/// Loads the unified configuration from `config.jsonc`.
fn load_config() -> Config {
    let path = config_file("config.jsonc");
    let data = fs::read_to_string(path).expect("Failed to read config.jsonc");
    load_config_from_str(&data)
}

fn load_config_from_str(data: &str) -> Config {
    json5::from_str(data).expect("Invalid JSONC in config.jsonc")
}

/// Loads configuration from a custom path when provided.
pub fn load_config_at(path: Option<&str>) -> Result<Config, String> {
    match path {
        Some(custom_path) => {
            let data = fs::read_to_string(custom_path)
                .map_err(|err| format!("Failed to read config at {}: {}", custom_path, err))?;
            Ok(load_config_from_str(&data))
        }
        None => Ok(load_config()),
    }
}

/// Returns the built-in default configuration.
pub fn default_config() -> Config {
    load_config_from_str(DEFAULT_CONFIG)
}

/// Returns the built-in default layout section.
pub fn default_layout() -> Vec<LayoutItem> {
    default_config().layout
}

/// Loads the modules section from `config.jsonc`.
///
/// # Returns
///
/// A `Vec<LayoutItem>` containing the loaded module configuration.
pub fn load_print_layout() -> Vec<LayoutItem> {
    let layout = load_config().layout;
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
    load_config().flags
}

/// Generates the default unified configuration file.
///
/// Writes `config.jsonc` with the default contents. Returns a map with the filename
/// and whether the operation succeeded, matching the previous multi-file API.
pub fn generate_config_files() -> HashMap<String, bool> {
    let mut results = HashMap::new();

    let result = save_to_config_file("config.jsonc", DEFAULT_CONFIG).is_ok();
    results.insert("config.jsonc".to_string(), result);

    results
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
