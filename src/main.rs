mod config;
mod modules;

use crate::config::default::ConfigsExt;
use modules::ascii::get_ascii_art;
use modules::helper::list_options;
use modules::system::distro::{get_distro, DistroDisplay};

use config::default::Configs;

use std::cmp;

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("--help" | "-h") => {
            print_help();
            return;
        }
        Some("--list-options" | "-l") => {
            list_options();
            return;
        }
        Some("--init" | "-i") => {
            if !Configs::ensure_config_exists() {
                println!("⚠️ Config file already exists");
            }
            return;
        }
        _ => {}
    }

    Configs::ensure_config_exists();

    let cfg = Configs::load();

    if let Some(layout) = cfg.get("layout") {
        let filled = Configs::fill_layout(layout, &cfg);
        let ascii_path = cfg.get("ascii_path");
        let distro = get_distro(DistroDisplay::Name);
        let ascii = get_ascii_art(&distro, ascii_path);
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
    let max_lines = cmp::max(ascii_lines.len(), info_lines.len());

    for i in 0..max_lines {
        // let ascii_part = ascii_lines.get(i).unwrap_or(&"");
        let info_part = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");

        // println!("{:<20} {}", ascii_part, info_part);
        println!(" {}", info_part);
    }
}
