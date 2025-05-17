use std::{collections::HashMap, fs, path::Path};

use super::{ascii::get_builtin_ascii_art, colors::get_builtin_distro_colors};

pub const DEFAULT_ANSI_ALL_COLORS: [&str; 16] = [
    "\x1b[1;30m", // Black
    "\x1b[1;31m", // Red
    "\x1b[1;32m", // Green
    "\x1b[1;33m", // Yellow
    "\x1b[1;34m", // Blue
    "\x1b[1;35m", // Magenta
    "\x1b[1;36m", // Cyan
    "\x1b[1;37m", // White
    "\x1b[1;90m", // Bright Black
    "\x1b[1;91m", // Bright Red
    "\x1b[1;92m", // Bright Green
    "\x1b[1;93m", // Bright Yellow
    "\x1b[1;94m", // Bright Blue
    "\x1b[1;95m", // Bright Magenta
    "\x1b[1;96m", // Bright Cyan
    "\x1b[1;97m", // Bright White
];

pub fn get_bar(percent: u8) -> String {
    let total_blocks = 14;
    let filled_blocks = (percent as usize * total_blocks) / 100;
    let empty_blocks = total_blocks - filled_blocks;

    let filled = "█".repeat(filled_blocks);
    let empty = "░".repeat(empty_blocks);
    format!("[{}{}]", filled, empty)
}

pub fn get_terminal_color(color_blocks: &str) -> Vec<String> {
    let color_codes: [u8; 8] = [30, 31, 32, 33, 34, 35, 36, 37]; // ANSI foreground colors

    let mut normal = Vec::with_capacity(8);
    let mut bold = Vec::with_capacity(8);

    for &code in &color_codes {
        normal.push(format!("\x1b[{}m{}\x1b[0m", code, color_blocks)); // normal
        bold.push(format!("\x1b[1;{}m{}\x1b[0m", code, color_blocks)); // bold
    }

    vec![normal.join(""), bold.join("")]
}

// ---------------------------------
//        ASCII ART Functions
// ---------------------------------

pub fn get_custom_ascii(custom_path: &str) -> String {
    if let Ok(content) = fs::read_to_string(Path::new(custom_path)) {
        return content;
    }

    "\n".to_string()
}

pub fn get_ascii_and_colors(ascii_distro: &str) -> String {
    if ascii_distro == "off" {
        return "".to_string();
    }

    let ascii_art = get_builtin_ascii_art(ascii_distro);
    // ascii_art.push_str("${reset}");

    ascii_art.to_string()
}

// ---------------------------------
//        Color Functions
// ---------------------------------

pub fn colorize_text(input: String, colors: &HashMap<&str, &str>) -> String {
    let mut result = String::new();

    for line in input.lines() {
        let mut colored = line.to_owned();
        for (key, code) in colors {
            let placeholder = format!("${{{}}}", key);
            colored = colored.replace(&placeholder, code);
        }
        result.push_str(&colored);
        result.push('\n');
    }

    result
}

pub fn color_palette(
    entries: &[(&'static str, &'static str)],
) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    for (k, v) in entries {
        map.insert(*k, *v);

        // Add bold variant: bold.c1 → \x1b[1;31m if c1 is \x1b[0;31m
        if let Some(code) = v.strip_prefix("\x1b[0;") {
            let bold_code = format!("\x1b[1;{}", code);
            let bold_key = format!("bold.{}", k);
            map.insert(
                Box::leak(bold_key.into_boxed_str()),
                Box::leak(bold_code.into_boxed_str()),
            );
        }
    }

    map.insert("reset", "\x1b[0m");
    map
}

pub fn get_colors_in_order(color_order: &[u8]) -> HashMap<&'static str, &'static str> {
    // Start with c0 = bold black
    let mut entries: Vec<(&'static str, &'static str)> = vec![("c0", "\x1b[1;30m")];

    let mut used = vec![false; 16]; // support 0–15

    // Fill c1 to cX using given color_order
    for (i, &idx) in color_order.iter().enumerate() {
        if idx < 16 {
            let key: &'static str = Box::leak(format!("c{}", i + 1).into_boxed_str());
            entries.push((key, DEFAULT_ANSI_ALL_COLORS[idx as usize]));
            used[idx as usize] = true;
        }
    }

    // Fill remaining cX from unused colors
    let mut next_index = color_order.len() + 1;
    for (i, &color) in DEFAULT_ANSI_ALL_COLORS.iter().enumerate() {
        if !used[i] {
            let key: &'static str = Box::leak(format!("c{}", next_index).into_boxed_str());
            entries.push((key, color));
            next_index += 1;
        }
    }

    // Generate HashMap with bold.* variants and reset
    let mut map = color_palette(&entries);
    map.insert("reset", "\x1b[0m");
    map
}

pub fn get_custom_colors_order(colors_str_order: &str) -> HashMap<&'static str, &'static str> {
    let custom_color_str_list: Vec<&str> = colors_str_order.split(',').map(str::trim).collect();

    // Try to parse all color indices
    let all_parsed: Option<Vec<u8>> = custom_color_str_list
        .iter()
        .map(|s| s.parse::<u8>().ok())
        .collect();

    let color_list: Vec<u8> = if let Some(list) = all_parsed {
        list
    } else {
        // Fallback: interpret the string as a distro name
        get_builtin_distro_colors(colors_str_order).to_vec()
    };

    get_colors_in_order(&color_list)
}

pub fn get_distro_colors(distro: &str) -> HashMap<&'static str, &'static str> {
    let dist_color = get_builtin_distro_colors(distro);
    // ascii_art.push_str("${reset}");

    get_colors_in_order(dist_color)
}
