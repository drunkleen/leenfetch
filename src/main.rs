use anyhow::{Context, Result, anyhow};
use atty::Stream;
use clap::Parser;
use leenfetch_core::{
    SystemInfo, config,
    core::{Core, Data},
    gather_system_info,
    modules::{
        helper::{Args, CliOverrides, OutputFormat, list_options, print_custom_help},
        utils::colorize_text,
    },
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::HashSet,
    io::{self, Read},
    process::Command,
};
use unicode_width::UnicodeWidthStr;

static ANSI_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\x1b\[[0-9;]*m").unwrap_or_else(|e| panic!("Invalid ANSI regex: {e}"))
});

fn main() {
    if let Err(err) = run() {
        eprintln!("{err:?}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    if args.help {
        print_custom_help();
        return Ok(());
    }
    if args.list_options {
        list_options();
        return Ok(());
    }
    if args.init {
        let results = config::ensure_config_files_exist();
        for (file, created) in results {
            if created {
                println!("✅ Created missing config: {file}");
            } else {
                println!("✔️ Config already exists: {file}");
            }
        }
        return Ok(());
    }
    if args.reinit {
        let result = config::delete_config_files();
        for (file, ok) in result {
            println!(
                "{} {}\n use --help for more info",
                if ok {
                    "🗑️ Deleted"
                } else {
                    "⚠️ Failed to delete"
                },
                file
            );
        }

        let result = config::generate_config_files();
        for (file, ok) in result {
            println!(
                "{} {}\n use --help for more info",
                if ok {
                    "✅ Generated"
                } else {
                    "⚠️ Failed to generate"
                },
                file
            );
        }
        return Ok(());
    }

    let overrides = args.into_overrides();

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
            .context("Failed to read from stdin")?;
    }

    let mut config = if overrides.use_defaults {
        config::default_config()
    } else {
        match config::load_config_at(overrides.config_path.as_deref()) {
            Ok(cfg) => cfg,
            Err(err) => return Err(anyhow!(err)),
        }
    };

    let mut flags = config.flags.clone();
    let mut layout = if config.layout.is_empty() {
        config::default_layout()
    } else {
        config.layout.clone()
    };

    if let Err(err) = apply_flag_overrides(&mut flags, &overrides) {
        return Err(anyhow!(err));
    }
    apply_layout_overrides(&mut layout, &overrides);

    config.flags = flags.clone();
    config.layout = layout.clone();

    let core = Core::new_with(flags, layout);

    if !overrides.ssh_hosts.is_empty() {
        return run_remote(&core, &overrides, &pipe_input);
    }

    let system_info = gather_system_info(&config).context("Failed to gather system information")?;

    if matches!(overrides.output_format, OutputFormat::Json) {
        let json = serde_json::to_string_pretty(&system_info)
            .context("Failed to serialize system info to JSON")?;
        println!("{json}");
        return Ok(());
    }

    let data = Data::from(&system_info);
    let info_layout = core.render_layout(&data);
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

    Ok(())
}

fn run_remote(core: &Core, overrides: &CliOverrides, pipe_input: &str) -> Result<()> {
    let is_json = matches!(overrides.output_format, OutputFormat::Json);

    if is_json {
        for (index, host) in overrides.ssh_hosts.iter().enumerate() {
            if index > 0 {
                println!();
            }
            let info = fetch_remote_system_info(host)?;
            let mut data = Data::from(&info);
            if let Some(parsed) = parse_ssh_target_parts(host) {
                if let Some(ssh_user) = parsed.user {
                    if !ssh_user.is_empty() {
                        data.username = Some(ssh_user.to_string());
                    }
                }
                if !parsed.host.is_empty() {
                    data.hostname = Some(parsed.host.to_string());
                }
            }
            // Emit JSON per host
            let json = serde_json::to_string_pretty(&SystemInfo::from(data))
                .context("Failed to serialize remote system info to JSON")?;
            println!("{json}");
        }
        return Ok(());
    }

    for (index, host) in overrides.ssh_hosts.iter().enumerate() {
        if index > 0 {
            println!();
        }
        println!("=== Remote: {host} ===");
        let info = fetch_remote_system_info(host)?;
        let mut data = Data::from(&info);

        let distro_hint = info.distro.as_deref().or(info.os.as_deref());
        let (ascii, colors) = core.get_ascii_and_colors_for_distro(distro_hint);
        let ascii_block = if !pipe_input.is_empty() {
            pipe_input.to_string()
        } else {
            colorize_text(ascii.clone(), &colors)
        };

        if let Some(parsed) = parse_ssh_target_parts(host) {
            if let Some(ssh_user) = parsed.user {
                if !ssh_user.is_empty() {
                    data.username = Some(ssh_user.to_string());
                }
            }
            if !parsed.host.is_empty() {
                data.hostname = Some(parsed.host.to_string());
            }
        }

        let info_layout = core.render_layout(&data);
        let info_lines = colorize_text(info_layout, &colors)
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();

        print_ascii_and_info(&ascii_block, &info_lines);
    }

    Ok(())
}

fn fetch_remote_system_info(target: &str) -> Result<SystemInfo> {
    let ssh_bin = detect_ssh_binary();
    let parsed = parse_ssh_target_parts(target).unwrap_or_else(|| ParsedSshTarget {
        user: None,
        host: target,
        port: None,
    });

    let mut cmd = Command::new(ssh_bin);
    cmd
        // .arg("-o")
        // .arg("BatchMode=yes") // avoid interactive password prompts
        .arg("-o")
        .arg("ConnectTimeout=5");

    if let Some(port) = parsed.port {
        cmd.arg("-p").arg(port);
    }

    let destination = if let Some(user) = parsed.user {
        format!("{user}@{}", parsed.host)
    } else {
        parsed.host.to_string()
    };

    let output = cmd
        .arg(destination)
        .arg("leenfetch")
        .arg("--format")
        .arg("json")
        .output()
        .with_context(|| format!("Failed to spawn ssh for target {target}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "ssh to {target} failed (status {}): {stderr}",
            output
                .status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "unknown".into())
        ));
    }

    let stdout = String::from_utf8(output.stdout)
        .with_context(|| format!("Invalid UTF-8 in ssh output from {target}"))?;
    let info: SystemInfo = serde_json::from_str(&stdout)
        .with_context(|| format!("Failed to parse JSON from {target}: {}", stdout.trim()))?;
    Ok(info)
}

#[cfg(target_os = "windows")]
fn detect_ssh_binary() -> &'static str {
    "ssh.exe"
}

#[cfg(not(target_os = "windows"))]
fn detect_ssh_binary() -> &'static str {
    "ssh"
}

struct ParsedSshTarget<'a> {
    user: Option<&'a str>,
    host: &'a str,
    port: Option<&'a str>,
}

fn parse_ssh_target_parts(target: &str) -> Option<ParsedSshTarget<'_>> {
    if target.is_empty() {
        return None;
    }

    let (user, host_port) = if let Some((u, rest)) = target.split_once('@') {
        (Some(u), rest)
    } else {
        (None, target)
    };

    // Handle [ipv6]:port
    if let Some(stripped) = host_port.strip_prefix('[') {
        if let Some(end) = stripped.find(']') {
            let host = &stripped[..end];
            let port = stripped[end + 1..].strip_prefix(':');
            return Some(ParsedSshTarget { user, host, port });
        }
    }

    // Handle host:port (avoid false split on bare IPv6 without brackets)
    if let Some((host, port)) = host_port.rsplit_once(':') {
        if host.contains(':') {
            // Likely bare IPv6 address without brackets; treat whole as host
            return Some(ParsedSshTarget {
                user,
                host: host_port,
                port: None,
            });
        }
        return Some(ParsedSshTarget {
            user,
            host,
            port: Some(port),
        });
    }

    Some(ParsedSshTarget {
        user,
        host: host_port,
        port: None,
    })
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
    let _ = std::io::Write::flush(&mut std::io::stdout());

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
