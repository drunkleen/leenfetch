mod config;
mod core;
mod modules;

use atty::Stream;
use core::Core;
use modules::{helper::handle_args, utils::colorize_text};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use unicode_width::UnicodeWidthStr;

static ANSI_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b\[[0-9;]*m").expect("valid ANSI regex"));

fn main() {
    let mut args = std::env::args();
    args.next(); // skip binary name

    let override_map = match handle_args(&mut args) {
        Ok(map) => map,
        Err(_) => return,
    };

    let results = config::ensure_config_files_exist();
    for (file, created) in results {
        if created {
            println!("✅ Created missing config: {file}");
        }
    }

    let mut pipe_input = String::new();
    if !atty::is(Stream::Stdin) {
        io::stdin()
            .read_to_string(&mut pipe_input)
            .expect("Failed to read from stdin");
    }

    let mut core = Core::new();

    let info_layout = core.get_info_layout();
    let (ascii, colors) = core.get_ascii_and_colors(override_map);

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
    println!();
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
