mod config;
mod modules;
use regex::Regex;

use modules::helper::list_options;

use config::run::Run;
use std::collections::HashMap;

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
                if !Run::ensure_config_exists() {
                    println!("⚠️ Config file already exists");
                }
                return;
            }
            "--distro" => {
                if let Some(val) = args.next() {
                    override_map.insert("distro", val);
                }
            }
            "--ascii-size" => {
                if let Some(val) = args.next() {
                    override_map.insert("ascii-size", val);
                }
            }
            _ => {
                println!("❌ Unknown argument: {}", arg);
                print_help();
                return;
            }
        }
    }

    Run::ensure_config_exists();

    let cfg = Run::load();

    if let Some(layout) = cfg.get("layout") {
        let (filled, ascii) = &Run::fill_layout(layout, &cfg, override_map);

        print_ascii_and_info(
            &ascii,
            &filled.lines().map(|l| l.to_string()).collect::<Vec<_>>(),
        );
    } else {
        print!("Ensures that config.conf exists in proper location\nUse --init to create it if needed.\n\n");
        print_help();
    }
}

fn print_help() {
    println!(
        r#"📦 leenfetch — Minimal, Stylish System Info for Your Terminal

USAGE:
  leenfetch [OPTIONS]

OPTIONS:
  -h, --help           Show this help message and exit
  -i, --init           Create a default config file at ~/.config/leenfetch/config.conf
  -l, --list-options   Show all available config options and values

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
  leenfetch --init           🔧 Create the config file if it doesn’t exist
  leenfetch                  🚀 Run normally with your config
  leenfetch --list-options   📜 View all available configuration keys

CUSTOMIZATION:
🛠️  Config path:
    • Linux:   ~/.config/leenfetch/config.conf
    • Windows: %USERPROFILE%\.config\leenfetch\config.conf

🎨 Output Layout:
    The layout is controlled using a templated multi-line string:
    
    Example block:
      [[cpu]]
      ${{bold.c5}}CPU:${{reset}} {{cpu_index}}
      [[/cpu]]

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
    let ascii_lines: Vec<&str> = ascii.lines().collect();
    let info_lines = info_lines.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    // Regex to strip ANSI escape codes (matches \x1b[ ... m)
    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();

    let max_ascii_width = ascii_lines
        .iter()
        .map(|line| ansi_regex.replace_all(line, "").len())
        .max()
        .unwrap_or(0);

    let max_lines = std::cmp::max(ascii_lines.len(), info_lines.len());

    for i in 0..max_lines {
        let ascii_part = *ascii_lines.get(i).unwrap_or(&"");
        let info_part = info_lines.get(i).copied().unwrap_or("");

        // Calculate visible padding
        let visible_len = ansi_regex.replace_all(ascii_part, "").len();
        let padding = " ".repeat(max_ascii_width.saturating_sub(visible_len) + 2); // +2 space

        print!("{ascii_part}{}", padding);
        println!("{info_part}");
    }
}
