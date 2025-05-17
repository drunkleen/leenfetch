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

pub fn load_print_layout() -> Vec<Layout> {
    let path = config_file("print_layout.ron");
    let data = fs::read_to_string(path).expect("Failed to read print_order.ron");
    from_str(&data).expect("Invalid RON in print_order")
}

pub fn load_flags() -> settings::Flags {
    let path = config_file("flags.ron");
    let data = fs::read_to_string(path).expect("Failed to read flags.ron");
    from_str(&data).expect("Invalid RON in settings")
}

pub fn load_toggles() -> settings::Toggles {
    let path = config_file("toggles.ron");
    let data = fs::read_to_string(path).expect("Failed to read toggles.ron");
    from_str(&data).expect("Invalid RON in toggles")
}

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

fn save_to_config_file(file_name: &str, content: &str) -> std::io::Result<()> {
    let path = config_file(file_name);

    // مطمئن شو دایرکتوری وجود داره
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("Creating config file: {}", path.display());

    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn delete_config_files() -> HashMap<String, bool> {
    let filenames = ["print_layout.ron", "flags.ron", "toggles.ron"];
    let mut results = HashMap::new();

    for file in filenames {
        let result = delete_config_file(file).is_ok();
        results.insert(file.to_string(), result);
    }

    results
}

fn delete_config_file(file_name: &str) -> std::io::Result<()> {
    let path = config_file(file_name);

    if path.exists() {
        std::fs::remove_file(path)?;
    }

    Ok(())
}

fn config_file(name: &str) -> PathBuf {
    config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("leenfetch")
        .join(name)
}

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

fn ensure_config_file_exists(file_name: &str, default_content: &str) -> std::io::Result<bool> {
    let path = config_file(file_name);

    if path.exists() {
        return Ok(false); // Already exists
    }

    save_to_config_file(file_name, default_content)?;
    Ok(true) // Created
}
