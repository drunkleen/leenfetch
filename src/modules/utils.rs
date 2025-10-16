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

/// Generates a visual bar representation of a percentage using Unicode blocks.
///
/// # Arguments
///
/// * `percent` - An unsigned 8-bit integer representing the percentage (0-100) to be visualized.
///
/// # Returns
///
/// * A `String` containing a visual bar representation, with filled blocks (`█`) indicating the
///   percentage, and empty blocks (`░`) for the remainder. The total length of the bar is 14
///   characters, enclosed in square brackets.
pub fn get_bar(percent: u8) -> String {
    let total_blocks = 14;
    let filled_blocks = (percent as usize * total_blocks) / 100;
    let empty_blocks = total_blocks - filled_blocks;

    let filled = "█".repeat(filled_blocks);
    let empty = "░".repeat(empty_blocks);
    format!("[{}{}]", filled, empty)
}

/// Generates a vector of 2 strings, each containing a row of 8 blocks
/// colored with different ANSI foreground colors. The first string has
/// normal colors, the second has bold colors.
///
/// The input string `color_blocks` should contain 8 identical block characters
/// (e.g. █, ░, ▓, ▒, etc.). The output strings will have these blocks
/// colored with different ANSI colors.
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

/// Reads a file at the given custom path and returns its content as a string.
/// If there is an error while reading the file, an empty string is returned.
///
/// # Arguments
///
/// * `custom_path`: The path to the custom ASCII art file.
///
/// # Returns
///
/// A string containing the content of the file, or an empty string if there was an error.
pub fn get_custom_ascii(custom_path: &str) -> String {
    if let Ok(content) = fs::read_to_string(Path::new(custom_path)) {
        return content;
    }

    "".to_string()
}

/// Given a distro name, returns a string of its corresponding ASCII art.
/// If the distro isn't found, an empty string is returned.
/// If the distro is "off", an empty string is returned.
pub fn get_ascii_and_colors(ascii_distro: &str) -> String {
    if ascii_distro == "off" {
        return "".to_string();
    }

    let ascii_art = get_builtin_ascii_art(ascii_distro);

    ascii_art.to_string()
}

// ---------------------------------
//        Color Functions
// ---------------------------------

/// Replaces placeholders in a string with ANSI escape codes to colorize
/// the output.
///
/// Placeholders are in the form of `${{key}}`, where `key` is the key in
/// the provided `colors` HashMap. The value associated with the `key` is
/// the ANSI escape code for the color.
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

/// Creates a `HashMap` of ANSI color codes from the given entries.
///
/// Each entry is a tuple containing a key and a corresponding ANSI
/// color code. The function populates the `HashMap` with these entries
/// and also adds bold variants for each color code that starts with
/// `\x1b[0;`. The bold variant key is prefixed with "bold." (e.g.,
/// `bold.c1` for `c1`).
///
/// Additionally, a "reset" key is included in the map, which maps to
/// the ANSI reset code `\x1b[0m`.
///
/// # Arguments
///
/// * `entries` - A slice of tuples where each tuple contains a string
///   key and an ANSI color code.
///
/// # Returns
///
/// * A `HashMap` with the original entries, their bold variants, and a
///   reset entry.
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

/// Given a slice of color indices or a distro name, generates a HashMap
/// of `cX` keys (where `X` is the index, starting from 1) mapped to the
/// corresponding ANSI foreground color codes.
///
/// The color order is determined by the input slice. The first color is
/// assigned to `c1`, the second to `c2`, and so on. If the input slice is
/// shorter than 16 elements, the remaining colors are filled from the
/// default color palette in the order they appear.
///
/// If the input slice is empty, the function returns an empty HashMap.
///
/// The HashMap also includes bold variants for each color, which can be
/// accessed using the `bold.*` keys. For example, `bold.c1` would be the
/// bold variant of `c1`.
///
/// The `reset` key is also included, which resets the text to the default
/// color.
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

/// Given a string of comma-separated color indices or a distro name, returns a
/// HashMap of color codes c0 to cX as found in the given distro's color
/// definition. The color codes are from the ANSI color palette.
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

/// Given a distro name, returns a HashMap of color codes c0 to cX as found
/// in the given distro's color definition. The color codes are from the
/// ANSI color palette. If the distro isn't found, an empty HashMap is returned.
pub fn get_distro_colors(distro: &str) -> HashMap<&'static str, &'static str> {
    let dist_color = get_builtin_distro_colors(distro);

    get_colors_in_order(dist_color)
}
