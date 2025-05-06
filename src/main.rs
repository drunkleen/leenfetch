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
        r#"📦 leenfetch — Minimal, stylish system info for your terminal

USAGE:
  leenfetch [OPTIONS]

DESCRIPTION:
  A fast and customizable alternative to neofetch, written in Rust.
  It shows system information alongside optional ASCII art, with rich theming support.

OPTIONS:
  -h, --help           Show this help message and exit
  -i, --init           Create a default config file at ~/.config/leenfetch/config.conf
  -l, --list-options   Show all available config options and accepted values

EXAMPLES:
  leenfetch --init           🔧 Generate config file if it doesn't exist
  leenfetch                  🚀 Run with current config
  leenfetch --list-options   📜 See what you can tweak

CUSTOMIZATION:
✍️  Edit the config file to customize output:
     ~/.config/leenfetch/config.conf

💡 Pro Tip:
    Use `show_*` keys to toggle modules, and adjust layout using the `layout` string block!

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
