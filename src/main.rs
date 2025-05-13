mod core;
mod modules;

use modules::helper::list_options;

use core::core::Core;
use regex::Regex;
use std::collections::HashMap;
use unicode_width::UnicodeWidthStr;

fn main() {
    let mut args = std::env::args().skip(1); // skip binary name

    let mut override_map: HashMap<&'static str, String> = HashMap::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--list-options" | "-l" => {
                list_options();
                return;
            }
            "--init" | "-i" => {
                if !Core::ensure_config_exists() {
                    println!("⚠️ Config file already exists");
                }
                return;
            }
            "--reinit" | "-r" => {
                Core::force_create_config();
                println!("🔁 Reinitialized config file.");
                return;
            }
            "--ascii_distro" => {
                if let Some(val) = args.next() {
                    override_map.insert("ascii_distro", val);
                } else {
                    println!("❌ ascii_distro cannot be empty");
                    println!("⚠️ use --help for more info");
                    return;
                }
            }
            "--ascii_colors" => {
                if let Some(val) = args.next() {
                    override_map.insert("ascii_colors", val);
                } else {
                    println!("❌ ascii_colors cannot be empty");
                    println!("⚠️ use --help for more info");
                    return;
                }
            }
            "--custom_ascii_path" => {
                if let Some(val) = args.next() {
                    override_map.insert("custom_ascii_path", val);
                } else {
                    println!("❌ custom_ascii_path cannot be empty");
                    println!("⚠️ use --help for more info");
                    return;
                }
            }
            _ => {
                println!("❌ Unknown argument: {}", arg);
                print_help();
                return;
            }
        }
    }

    Core::ensure_config_exists();

    let cfg = Core::load();

    if let Some(layout) = cfg.get_from_cfg("layout") {
        let (filled, ascii) = &Core::fill_layout(layout, &cfg, override_map);

        print_ascii_and_info(
            &ascii,
            &filled.lines().map(|l| l.to_string()).collect::<Vec<_>>(),
        );
    } else {
        print!("Ensures that config.conf exists in proper location\nUse --init to create it if needed.\nUse --help for more info\n");
    }
}

fn print_help() {
    println!(
        r#"📦 leenfetch — Minimal, Stylish System Info for Your Terminal

USAGE:
  leenfetch [OPTIONS]

OPTIONS:
  -h, --help               Show this help message and exit
  -i, --init               Create a default config file at ~/.config/leenfetch/config.conf
  -r, --reinit             Reinitialize the config file
  -l, --list-options       Show all available config options and values
      --ascii_distro <s>   Override detected distro (e.g., ubuntu, fedora, arch)
      --ascii_colors <i>   Override detected distro colors (e.g., 2,7,3)
      --ascii-size   <s>   Override ASCII size (small, large, or off)

DESCRIPTION:
  leenfetch is a fast, modern, and minimal system info tool,
  written in Rust, designed for terminal enthusiasts.

  It fetches and prints system information like:
    • OS, Kernel, Uptime
    • CPU, GPU, Memory, Disks
    • Shell, WM, DE, Theme
    • Resolution, Battery, Current Song

  It’s highly customizable through a config file with layout control,
  theming, and modular display toggles.

EXAMPLES:
leenfetch                         🚀 Run normally with your config
  leenfetch --init                🔧 Create the config file if it doesn’t exist
  leenfetch --ascii-size small    🖼 Force small ASCII size
  leenfetch --ascii_distro arch   🎨 Use Arch logo manually
  leenfetch --ascii_colors debian 🎨 Use Debian colors manually
  leenfetch --ascii_colors 2,7,3  🎨 Manually set custom colors
  leenfetch --list-options        📜 View all available configuration keys

CUSTOMIZATION:
🛠️  Config path:
    • Linux:   ~/.config/leenfetch/config.conf
    • Windows: %USERPROFILE%\.config\leenfetch\config.conf

🎨 Output Layout:
    The layout is controlled using a templated multi-line string:
    
    Example block:
      [cpu]
      ${{bold.c5}}CPU:${{reset}} {{cpu_index}}
      [/cpu]

    Use placeholders like {{cpu_index}}, {{gpu}}, {{uptime_index}}, etc.
    Full layout templates can be found in the example config.

💡 Tips:
  • Toggle individual modules using keys like `show_cpu=on`
  • Change themes using `disk_display=barinfo`, `battery_display=infobar`
  • Combine with shell aliases for quick use!

MORE:
  Run with `--list-options` to see every supported config key.
"#
    );
}

fn print_ascii_and_info(ascii: &str, info_lines: &[String]) {
    println!();
    let ascii_lines: Vec<&str> = ascii.lines().collect();
    let info_lines = info_lines.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let ascii_count = ascii_lines.len();
    let info_count = info_lines.len();
    let mut total_lines = ascii_count.max(info_count);

    // Strip ANSI escape codes for accurate display width calculation
    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();

    // Calculate the max visible width of ASCII lines
    let max_ascii_width = ascii_lines
        .iter()
        .map(|line| {
            let stripped = ansi_regex.replace_all(line, "");
            UnicodeWidthStr::width(stripped.as_ref())
        })
        .max()
        .unwrap_or(0);

    let print_column = max_ascii_width + 4; // info column start

    for line in &ascii_lines {
        println!("{line}");
    }

    // Move cursor back up to start of ASCII block
    let move_up = ascii_lines.len();
    print!("\x1b[{}A", move_up);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    total_lines -= info_lines.len();
    // Print each info line aligned to calculated column
    for (_, info_line) in info_lines.iter().enumerate() {
        print!("\x1b[{}G", print_column); // move to column
        println!("{info_line}");
    }

    for _ in 0..total_lines {
        println!();
    }
}
