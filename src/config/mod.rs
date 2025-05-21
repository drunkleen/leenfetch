pub mod defaults;
pub mod settings;

use self::{
    defaults::{DEFAULT_FLAGS, DEFAULT_PRINT_LAYOUT, DEFAULT_TOGGLES},
    settings::Layout,
};
use dirs::config_dir;
use ron::de::from_str;
use std::io::Write;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    fs::{self, File},
};

/// Loads the print layout configuration from the "print_layout.ron" file.
///
/// If the file does not exist, this function will panic with the message "Failed to read print_order.ron".
/// If the file exists but contains invalid RON, this function will panic with the message "Invalid RON in print_order".
///
/// # Returns
///
/// A `Vec<Layout>` containing the loaded print layout configuration.

pub fn load_print_layout() -> Vec<Layout> {
    let path = config_file("print_layout.ron");
    let data = fs::read_to_string(path).expect("Failed to read print_order.ron");
    from_str(&data).expect("Invalid RON in print_order")
}

/// Loads the configuration flags from the "flags.ron" file.
///
/// If the file does not exist, this function will panic with the message "Failed to read flags.ron".
/// If the file exists but contains invalid RON, this function will panic with the message "Invalid RON in settings".
///
/// # Returns
///
/// A `settings::Flags` struct containing the loaded configuration.
pub fn load_flags() -> settings::Flags {
    let path = config_file("flags.ron");
    let data = fs::read_to_string(path).expect("Failed to read flags.ron");
    from_str(&data).expect("Invalid RON in settings")
}

/// Loads the configuration for which blocks of information to show from the "toggles.ron" file.
///
/// If the file does not exist, this function will panic with the message "Failed to read toggles.ron".
/// If the file exists but contains invalid RON, this function will panic with the message "Invalid RON in toggles".
///
/// # Returns
///
/// A `settings::Toggles` struct containing the loaded configuration.
pub fn load_toggles() -> settings::Toggles {
    let path = config_file("toggles.ron");
    let data = fs::read_to_string(path).expect("Failed to read toggles.ron");
    from_str(&data).expect("Invalid RON in toggles")
}

/// Generates the default configuration files.
///
/// This function iterates over a predefined list of configuration files and their default contents.
/// It attempts to save each file using `save_to_config_file`, which writes the content to the file
/// system. The function returns a `HashMap` where the keys are the names of the config files and the
/// values indicate whether the file was successfully saved.
///
/// # Returns
///
/// A `HashMap<String, bool>` where each key is a config file name and the value is a boolean indicating
/// whether the file was successfully saved.
pub fn generate_config_files() -> HashMap<String, bool> {
    let mut results = HashMap::new();

    let files = [
        ("print_layout.ron", &DEFAULT_PRINT_LAYOUT),
        ("flags.ron", &DEFAULT_FLAGS),
        ("toggles.ron", &DEFAULT_TOGGLES),
    ];

    for (filename, content) in files {
        let result = save_to_config_file(filename, content).is_ok();
        results.insert(filename.to_string(), result);
    }

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

    // مطمئن شو دایرکتوری وجود داره
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

/// Deletes all default config files.
///
/// This function is used in the `--clean-config` flag, and it removes all default config files.
/// It returns a HashMap where the keys are the names of the config files and the values indicate
/// whether the file was deleted.
pub fn delete_config_files() -> HashMap<String, bool> {
    let filenames = ["print_layout.ron", "flags.ron", "toggles.ron"];
    let mut results = HashMap::new();

    for file in filenames {
        let result = delete_config_file(file).is_ok();
        results.insert(file.to_string(), result);
    }

    results
}

/// Deletes the given configuration file, returning an error if the operation fails.
///
/// This function is used by `delete_config_files` to delete all configuration files.
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

/// Ensures that all of the configuration files exist.
///
/// This function creates the `leenfetch` directory and all config files if they do not exist.
/// It will not overwrite existing config files.
///
/// Returns a `HashMap` where the keys are the names of the config files and the values indicate
/// whether the file was created.
pub fn ensure_config_files_exist() -> HashMap<String, bool> {
    let files = [
        ("print_layout.ron", &DEFAULT_PRINT_LAYOUT),
        ("flags.ron", &DEFAULT_FLAGS),
        ("toggles.ron", &DEFAULT_TOGGLES),
    ];

    let mut results = HashMap::new();

    for (filename, content) in files {
        let created = ensure_config_file_exists(filename, content).unwrap_or(false);
        results.insert(filename.to_string(), created);
    }

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
