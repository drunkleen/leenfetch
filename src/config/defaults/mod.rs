/// Default configuration for the combined `config.jsonc` file.
///
/// The JSONC format accepts both standard JSON and `//` comments, mirroring what Fastfetch ships.
/// Users can keep everything in one place while still enjoying inline descriptions.
pub const DEFAULT_CONFIG: &str = r#"// ===============================
// config.jsonc - Unified configuration for leenfetch
//
// This file combines the previous flags.ron, toggles.ron, and print_layout.ron files.
// Edit the sections below to customize appearance, enabled modules, and output ordering.
// ===============================
{
    // -------------------------------
    // flags - Display and formatting options
    // -------------------------------
    "flags": {
        // Select which distribution's ASCII art to display.
        // "auto" picks your current distro, any other value selects a specific artwork.
        "ascii_distro": "auto",

        // Choose the color palette for the ASCII art and color blocks.
        // Use "distro" for automatic colors or a comma-separated list of numbers.
        "ascii_colors": "distro",

        // Path to a custom ASCII art file. Leave blank to use the built-in art.
        "custom_ascii_path": "",

        // How to display battery information: "off", "bar", "infobar", or "barinfo".
        "battery_display": "off",

        // String used to draw color blocks in the palette preview.
        "color_blocks": "●",

        // Show the brand name of your CPU.
        "cpu_brand": true,

        // Show the number of CPU cores.
        "cpu_cores": true,

        // Show the CPU frequency.
        "cpu_frequency": true,

        // Show the current CPU speed.
        "cpu_speed": true,

        // Which temperature unit to use for CPU temperature: "C" or "F".
        "cpu_temp": "C",

        // Show the CPU temperature.
        "cpu_show_temp": false,

        // Show the version of your desktop environment.
        "de_version": true,

        // How much detail to show about your operating system.
        "distro_display": "name_model_arch",

        // How to display disk usage information.
        "disk_display": "info",

        // What label to show for each disk.
        "disk_subtitle": "dir",

        // Show memory usage as a percentage.
        "memory_percent": true,

        // Which unit to use for memory display: "kib", "mib", or "gib".
        "memory_unit": "mib",

        // How to display package manager information: "off", "on", or "tiny".
        "package_managers": "tiny",

        // Show the display's refresh rate.
        "refresh_rate": true,

        // Show the full path to your shell.
        "shell_path": true,

        // Show the version of your shell.
        "shell_version": true,

        // How to display system uptime: "full", "tiny", or "seconds".
        "uptime_shorthand": "tiny",

        // How to display os age: "full", "tiny", or "seconds".
        "os_age_shorthand": "tiny"
    },

    
    // -------------------------------
    // modules - Output order and custom rows
    // -------------------------------
    "modules": [
        { "type": "custom", "format": "${reset}┌─────────────────────${c1}System${reset}─────────────────────┐" },
        { "type": "titles", "key": "${reset}│${c1} ┌:"},
        // { "type": "os", "key": "${c16}│${c1} ├OS"},
        { "type": "distro", "key": "${reset}│${c1} ├:"},
        { "type": "kernel", "key": "${reset}│${c1} ├:"},
        { "type": "packages", "key": "${reset}│${c1} ├󰏖:"},
        { "type": "shell", "key": "${reset}│ ${c1}├:"},
        { "type": "wm", "key": "${reset}│${c1} ├:"},
        { "type": "de", "key": "${reset}│${c1} ├󰇄:"},
            { "type": "song", "key": "${reset}│${c1} ├"},
        { "type": "theme", "key": "${reset}│${c1} ├󰸌:"},
        { "type": "colors", "key": "${reset}│${c1} └:"},
        { "type": "custom", "format": "${c16}└────────────────────────────────────────────────┘" },
        // "break",
        { "type": "custom", "format": "${reset}┌────────────────────${c1}Hardware${reset}────────────────────┐" },
        { "type": "model", "key": "${reset}│${c1} ┌:"},
        { "type": "cpu", "key": "${reset}│${c1} ├:"},
        { "type": "gpu", "key": "${reset}│${c1} ├:"},
        { "type": "disk", "key": "${reset}│${c1} ├󰋊:"},
        { "type": "resolution", "key": "${reset}│${c1} ├󱄄:"},
        { "type": "battery", "key": "${reset}│${c1} ├:"},
        { "type": "memory", "key": "${reset}│${c1} └:"},
        { "type": "custom", "format": "${reset}└────────────────────────────────────────────────┘" },
        // "break",
        { "type": "custom", "format": "${reset}┌───────────────────${c1}Age/Uptime${reset}───────────────────┐" },
        { "type": "os_age", "key": "${c16}│${c1} ┌󱦟 OS Age"},
        { "type": "uptime", "key": "${c16}│${c1} └󱫐 Uptime"},
        { "type": "custom", "format": "${c16}└────────────────────────────────────────────────┘" }
    ]
}

"#;
