mod config;
mod core;
mod modules;
#[cfg(test)]
mod test_utils;

use atty::Stream;
use core::Core;
use modules::{
    helper::{CliOverrides, handle_args},
    utils::colorize_text,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::HashSet,
    io::{self, Read},
};
use unicode_width::UnicodeWidthStr;

static ANSI_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b\[[0-9;]*m").expect("valid ANSI regex"));

fn main() {
    let mut args = std::env::args();
    args.next(); // skip binary name

    let overrides = match handle_args(&mut args) {
        Ok(map) => map,
        Err(_) => return,
    };

    if !overrides.use_defaults && overrides.config_path.is_none() {
        let results = config::ensure_config_files_exist();
        for (file, created) in results {
            if created {
                println!("✅ Created missing config: {file}");
            }
        }
    }

    let mut pipe_input = String::new();
    if !atty::is(Stream::Stdin) {
        io::stdin()
            .read_to_string(&mut pipe_input)
            .expect("Failed to read from stdin");
    }

    let simple_run = overrides.flags.is_empty()
        && overrides.only_modules.is_none()
        && overrides.hide_modules.is_empty()
        && overrides.config_path.is_none()
        && !overrides.use_defaults;

    let mut core = if simple_run {
        Core::new()
    } else {
        let config = if overrides.use_defaults {
            config::default_config()
        } else {
            match config::load_config_at(overrides.config_path.as_deref()) {
                Ok(cfg) => cfg,
                Err(err) => {
                    eprintln!("{err}");
                    return;
                }
            }
        };

        let mut flags = config.flags.clone();
        let mut layout = if config.layout.is_empty() {
            config::default_layout()
        } else {
            config.layout.clone()
        };

        if let Err(err) = apply_flag_overrides(&mut flags, &overrides) {
            eprintln!("{err}");
            return;
        }
        apply_layout_overrides(&mut layout, &overrides);

        Core::new_with(flags, layout)
    };

    let info_layout = core.get_info_layout();
    let (ascii, colors) = core.get_ascii_and_colors();

    if !pipe_input.is_empty() {
        print_ascii_and_info(
            &pipe_input,
            &colorize_text(info_layout, &colors)
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<_>>(),
        );
    } else {
        print_ascii_and_info(
            &colorize_text(ascii, &colors),
            &colorize_text(info_layout, &colors)
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<_>>(),
        );
    }
}

/// Prints the ASCII art block and info lines side-by-side.
///
/// This function is responsible for rendering the ASCII art block and
/// the info lines side-by-side. If the ASCII art block is taller than
/// the info block, the remaining lines are filled with whitespace.
/// Otherwise, the info lines are printed below the ASCII art block.
///
/// The function takes two arguments: `ascii` is the ASCII art block as
/// a string, and `info_lines` is a vector of strings representing the
/// info lines. The function will split the ASCII art block into lines
/// and calculate the maximum visible width of the lines. It will then
/// print the ASCII art block and info lines side-by-side, with the
/// info lines starting at a column determined by the maximum visible
/// width of the ASCII art block.
fn print_ascii_and_info(ascii: &str, info_lines: &[String]) {
    // println!();
    let ascii_lines: Vec<&str> = ascii.lines().collect();
    let info_lines = info_lines.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let ascii_count = ascii_lines.len();
    let info_count = info_lines.len();
    let mut total_lines = ascii_count.max(info_count);

    // Calculate the max visible width of ASCII lines
    let max_ascii_width = ascii_lines
        .iter()
        .map(|line| {
            let stripped = ANSI_REGEX.replace_all(line, "");
            UnicodeWidthStr::width(stripped.as_ref())
        })
        .max()
        .unwrap_or(0);

    let print_column = if max_ascii_width > 0 {
        max_ascii_width + 4 // info column start
    } else {
        0
    };

    for line in &ascii_lines {
        println!("{line}");
    }

    // Move cursor back up to start of ASCII block
    let move_up = ascii_lines.len();
    print!("\x1b[{}A", move_up);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    total_lines -= info_lines.len();

    for info_line in info_lines.iter() {
        print!("\x1b[{}G", print_column);
        println!("{info_line}");
    }

    for _ in 0..total_lines {
        println!();
    }
}

fn apply_flag_overrides(
    flags: &mut config::settings::Flags,
    overrides: &CliOverrides,
) -> Result<(), String> {
    if let Some(value) = overrides.flags.get("ascii_distro") {
        flags.ascii_distro = value.clone();
    }

    if let Some(value) = overrides.flags.get("ascii_colors") {
        flags.ascii_colors = value.clone();
    }

    if let Some(value) = overrides.flags.get("custom_ascii_path") {
        flags.custom_ascii_path = value.clone();
    }

    if let Some(value) = overrides.flags.get("color_blocks") {
        flags.color_blocks = value.clone();
    }

    if let Some(value) = overrides.flags.get("battery_display") {
        let normalized = value.to_ascii_lowercase();
        match normalized.as_str() {
            "off" | "bar" | "infobar" | "barinfo" => {
                flags.battery_display = normalized;
            }
            _ => return Err(format!("Invalid value for --battery-display: {}", value)),
        }
    }

    if let Some(value) = overrides.flags.get("disk_display") {
        let normalized = value.to_ascii_lowercase();
        match normalized.as_str() {
            "info" | "percentage" | "infobar" | "barinfo" | "bar" => {
                flags.disk_display = normalized;
            }
            _ => {
                return Err(format!("Invalid value for --disk-display: {}", value));
            }
        }
    }

    if let Some(value) = overrides.flags.get("disk_subtitle") {
        let normalized = value.to_ascii_lowercase();
        match normalized.as_str() {
            "name" | "dir" | "none" | "mount" => {
                flags.disk_subtitle = normalized;
            }
            _ => {
                return Err(format!("Invalid value for --disk-subtitle: {}", value));
            }
        }
    }

    if let Some(value) = overrides.flags.get("memory_unit") {
        let normalized = value.to_ascii_lowercase();
        match normalized.as_str() {
            "kib" | "mib" | "gib" => {
                flags.memory_unit = normalized;
            }
            _ => {
                return Err(format!("Invalid value for --memory-unit: {}", value));
            }
        }
    }

    if let Some(value) = overrides.flags.get("package_managers") {
        let normalized = value.to_ascii_lowercase();
        match normalized.as_str() {
            "off" | "on" | "tiny" => flags.package_managers = normalized,
            _ => {
                return Err(format!("Invalid value for --packages: {}", value));
            }
        }
    }

    if let Some(value) = overrides.flags.get("uptime_shorthand") {
        let normalized = value.to_ascii_lowercase();
        match normalized.as_str() {
            "full" | "tiny" | "seconds" => flags.uptime_shorthand = normalized,
            _ => {
                return Err(format!("Invalid value for --uptime: {}", value));
            }
        }
    }

    if let Some(value) = overrides.flags.get("os_age_shorthand") {
        let normalized = value.to_ascii_lowercase();
        match normalized.as_str() {
            "full" | "tiny" | "seconds" => flags.os_age_shorthand = normalized,
            _ => {
                return Err(format!("Invalid value for --os-age: {}", value));
            }
        }
    }

    if let Some(value) = overrides.flags.get("distro_display") {
        let normalized = value.to_ascii_lowercase();
        match normalized.as_str() {
            "name"
            | "name_version"
            | "name_arch"
            | "name_model"
            | "name_model_version"
            | "name_model_arch"
            | "name_model_version_arch" => flags.distro_display = normalized,
            _ => {
                return Err(format!("Invalid value for --distro-display: {}", value));
            }
        }
    }

    if let Some(value) = overrides.flags.get("cpu_temp") {
        let normalized = value.to_ascii_lowercase();
        match normalized.as_str() {
            "c" | "celsius" => {
                flags.cpu_temp = 'C';
                flags.cpu_show_temp = true;
            }
            "f" | "fahrenheit" => {
                flags.cpu_temp = 'F';
                flags.cpu_show_temp = true;
            }
            "off" | "none" => {
                flags.cpu_show_temp = false;
            }
            _ if normalized.len() == 1 => {
                if let Some(ch) = normalized.chars().next() {
                    flags.cpu_temp = ch.to_ascii_uppercase();
                    flags.cpu_show_temp = true;
                }
            }
            _ => {
                return Err(format!("Invalid value for --cpu-temp-unit: {}", value));
            }
        }
    }

    apply_bool_override(flags, overrides, "memory_percent", |f, v| {
        f.memory_percent = v
    })?;
    apply_bool_override(flags, overrides, "cpu_show_temp", |f, v| {
        f.cpu_show_temp = v
    })?;
    apply_bool_override(flags, overrides, "cpu_speed", |f, v| f.cpu_speed = v)?;
    apply_bool_override(flags, overrides, "cpu_frequency", |f, v| {
        f.cpu_frequency = v
    })?;
    apply_bool_override(flags, overrides, "cpu_cores", |f, v| f.cpu_cores = v)?;
    apply_bool_override(flags, overrides, "cpu_brand", |f, v| f.cpu_brand = v)?;
    apply_bool_override(flags, overrides, "shell_path", |f, v| f.shell_path = v)?;
    apply_bool_override(flags, overrides, "shell_version", |f, v| {
        f.shell_version = v
    })?;
    apply_bool_override(flags, overrides, "refresh_rate", |f, v| f.refresh_rate = v)?;
    apply_bool_override(flags, overrides, "de_version", |f, v| f.de_version = v)?;

    Ok(())
}

fn apply_bool_override<F>(
    flags: &mut config::settings::Flags,
    overrides: &CliOverrides,
    key: &str,
    mut apply: F,
) -> Result<(), String>
where
    F: FnMut(&mut config::settings::Flags, bool),
{
    if let Some(value) = overrides.flags.get(key) {
        match value.as_str() {
            "true" => apply(flags, true),
            "false" => apply(flags, false),
            _ => return Err(format!("Invalid boolean value for {key}: {value}")),
        }
    }
    Ok(())
}

fn apply_layout_overrides(
    layout: &mut Vec<config::settings::LayoutItem>,
    overrides: &CliOverrides,
) {
    if let Some(only_modules) = &overrides.only_modules {
        let allowed: HashSet<String> = only_modules
            .iter()
            .map(|name| normalize_module_name(name))
            .filter(|name| !name.is_empty())
            .collect();

        if !allowed.is_empty() {
            *layout = layout
                .iter()
                .filter(|item| {
                    layout_item_key(item)
                        .map(|key| allowed.contains(&key))
                        .unwrap_or(false)
                })
                .cloned()
                .collect();
        }
    }

    if !overrides.hide_modules.is_empty() {
        let disallowed: HashSet<String> = overrides
            .hide_modules
            .iter()
            .map(|name| normalize_module_name(name))
            .filter(|name| !name.is_empty())
            .collect();

        if !disallowed.is_empty() {
            *layout = layout
                .iter()
                .filter(|item| {
                    !layout_item_key(item)
                        .map(|key| disallowed.contains(&key))
                        .unwrap_or(false)
                })
                .cloned()
                .collect();
        }
    }
}

fn normalize_module_name(name: &str) -> String {
    name.trim().to_ascii_lowercase().replace('-', "_")
}

fn layout_item_key(item: &config::settings::LayoutItem) -> Option<String> {
    match item {
        config::settings::LayoutItem::Break(value) => value
            .eq_ignore_ascii_case("break")
            .then(|| "break".to_string()),
        config::settings::LayoutItem::Module(module) => {
            module.field_name().map(|name| normalize_module_name(name))
        }
    }
}
