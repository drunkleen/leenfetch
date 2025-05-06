use std::collections::HashMap;
use std::sync::LazyLock;

#[allow(dead_code)]

pub static COLORS: LazyLock<HashMap<&str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        // Foreground (normal)
        ("fg.c1", "\x1b[30m"),
        ("fg.c2", "\x1b[31m"),
        ("fg.c3", "\x1b[32m"),
        ("fg.c4", "\x1b[33m"),
        ("fg.c5", "\x1b[34m"),
        ("fg.c6", "\x1b[35m"),
        ("fg.c7", "\x1b[36m"),
        ("fg.c8", "\x1b[37m"),
        // Foreground (bold)
        ("bold.c1", "\x1b[1;30m"),
        ("bold.c2", "\x1b[1;31m"),
        ("bold.c3", "\x1b[1;32m"),
        ("bold.c4", "\x1b[1;33m"),
        ("bold.c5", "\x1b[1;34m"),
        ("bold.c6", "\x1b[1;35m"),
        ("bold.c7", "\x1b[1;36m"),
        ("bold.c8", "\x1b[1;37m"),
        // Background
        ("bg.c1", "\x1b[40m"),
        ("bg.c2", "\x1b[41m"),
        ("bg.c3", "\x1b[42m"),
        ("bg.c4", "\x1b[43m"),
        ("bg.c5", "\x1b[44m"),
        ("bg.c6", "\x1b[45m"),
        ("bg.c7", "\x1b[46m"),
        ("bg.c8", "\x1b[47m"),
        // Reset
        ("reset", "\x1b[0m"),
    ])
});

pub fn colorize_text(input: &str) -> String {
    let mut result = String::new();

    for line in input.lines() {
        let mut colored = line.to_owned();

        for (key, code) in COLORS.iter() {
            colored = colored.replace(&format!("${{{}}}", key), code);
        }

        colored.push_str(COLORS["reset"]);
        result.push_str(&colored);
        result.push('\n');
    }

    if result.ends_with('\n') {
        result.pop();
    }

    result
}

pub fn get_bar(percent: u8) -> String {
    let total_blocks = 14;
    let filled_blocks = (percent as usize * total_blocks) / 100;
    let empty_blocks = total_blocks - filled_blocks;

    let filled = "█".repeat(filled_blocks);
    let empty = "░".repeat(empty_blocks);
    format!("[{}{}]", filled, empty)
}

pub fn get_terminal_color() -> Vec<String> {
    let color_blocks: [u8; 8] = [30, 31, 32, 33, 34, 35, 36, 37]; // ANSI foreground colors

    let mut terminal_colors = vec![];

    for code in color_blocks.iter() {
        terminal_colors.push(format!("\x1b[{}m{}", code, "███"));
    }

    let mut terminal_colors_bold = vec![];

    for code in color_blocks.iter() {
        terminal_colors_bold.push(format!("\x1b[1;{}m{}", code, "███")); // bright/bold version
    }

    vec![terminal_colors.join(""), terminal_colors_bold.join("")]
}

// ---------------------------------
//        Run Helper Functions
// ---------------------------------

pub fn process_loop_block<T>(
    output: &mut String,
    tag: &str,
    data: &[T],
    enabled: bool,
    render: impl Fn(&str, &T) -> String,
) {
    let start_tag = format!("[[{tag}]]");
    let end_tag = format!("[[/{tag}]]");

    if let Some(start_idx) = output.find(&start_tag) {
        if let Some(end_rel) = output[start_idx + start_tag.len()..].find(&end_tag) {
            let block_start = start_idx + start_tag.len();
            let block_end = block_start + end_rel;
            let full_end = block_end + end_tag.len();

            let block = &output[block_start..block_end];
            let rendered = if enabled {
                data.iter()
                    .map(|item| render(block.trim(), item))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                String::new()
            };

            output.replace_range(start_idx..full_end, &rendered);
        }
    }
}

pub fn process_single_block(output: &mut String, tag: &str, enabled: bool, value: Option<String>) {
    let start_tag = format!("[[{tag}]]");
    let end_tag = format!("[[/{tag}]]");

    if let Some(start_idx) = output.find(&start_tag) {
        if let Some(end_rel) = output[start_idx + start_tag.len()..].find(&end_tag) {
            let block_start = start_idx + start_tag.len();
            let block_end = block_start + end_rel;
            let full_end = block_end + end_tag.len();

            let block = &output[block_start..block_end];
            let rendered = if enabled {
                let val = value.unwrap_or_default();
                block.replace(&format!("{{{tag}_index}}"), &val).to_string()
            } else {
                String::new()
            };

            let final_rendered = if rendered.trim().is_empty() {
                // Remove whole line including newlines around the tag
                let mut before = &output[..start_idx];
                let mut after = &output[full_end..];

                // Remove trailing newline before tag
                if before.ends_with('\n') {
                    before = &before[..before.len() - 1];
                }

                // Remove leading newline after tag
                if after.starts_with('\n') {
                    after = &after[1..];
                }

                format!("{}{}", before, after)
            } else {
                let mut rendered = rendered;
                if !rendered.ends_with('\n') {
                    rendered.push('\n');
                }
                let mut result = String::new();
                result.push_str(&output[..start_idx]);
                result.push_str(&rendered);
                result.push_str(&output[full_end..]);
                result
            };

            *output = final_rendered;
        }
    }
}
