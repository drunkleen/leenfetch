pub fn process_loop_block<T>(
    output: &mut String,
    tag: &str,
    data: &[T],
    enabled: bool,
    render: impl Fn(&str, &T) -> String,
) {
    let start_tag = format!("[{tag}]");
    let end_tag = format!("[/{tag}]");

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
    let start_tag = format!("[{tag}]");
    let end_tag = format!("[/{tag}]");

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

pub fn load_layout_values(layout: &str) -> String {
    let layout = layout
        .lines()
        .into_iter()
        .map(|l| l.trim())
        .filter(|l| !l.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    // if let Some((key, value)) = line.split_once('=') {
    //     let key = key.trim().to_string();
    //     let val = value.trim().trim_matches('"');

    //     return val.to_string();
    // }

    layout
}
