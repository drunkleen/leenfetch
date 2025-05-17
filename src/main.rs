mod config;
mod core;
mod modules;

use modules::helper::{list_options, print_help};

// use core::core::Core;
use core::Core;
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
                let results = config::ensure_config_files_exist();

                for (file, created) in results {
                    if created {
                        println!("✅ Created missing config: {file}");
                    } else {
                        println!("✔️ Config already exists: {file}");
                    }
                }

                return;
            }
            "--reinit" | "-r" => {
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

    let results = config::ensure_config_files_exist();
    for (file, created) in results {
        if created {
            println!("✅ Created missing config: {file}");
        }
    }

    let mut core = Core::new();
    let (info, ascii) = core.fill_layout(override_map);

    print_ascii_and_info(
        &ascii,
        &info.lines().map(|l| l.to_string()).collect::<Vec<_>>(),
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
