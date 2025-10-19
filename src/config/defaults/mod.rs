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
        // Options: "auto" or a distribution name
        "ascii_distro": "auto",

        // Choose the color palette for the ASCII art and color blocks.
        // Use "distro" for automatic colors or a comma-separated list of numbers.
        // Options: "distro" or eg. "0,2,3,1,5,4"
        "ascii_colors": "distro",

        // Path to a custom ASCII art file. Leave blank to use the built-in art.
        // Options: "" or a file path
        "custom_ascii_path": "",

        // How to display battery information.
        // Options: "off", "bar", "infobar", "barinfo"
        "battery_display": "off",

        // String used to draw color blocks in the palette preview.
        "color_blocks": "●",

        // Show the brand name of your CPU.
        // options: true or false
        "cpu_brand": true,

        // Show the number of CPU cores.
        // options: true or false
        "cpu_cores": true,

        // Show the CPU frequency.
        // options: true or false
        "cpu_frequency": true,

        // Show the current CPU speed.
        // options: true or false
        "cpu_speed": true,

        // Which temperature unit to use for CPU temperature.
        // Options: "C", "F"
        "cpu_temp": "C",

        // Show the CPU temperature.
        // options: true or false
        "cpu_show_temp": false,

        // Show the version of your desktop environment.
        // options: true or false
        "de_version": true,

        // How much detail to show about your operating system.
        // Options: "name", "name_version", "name_arch", "name_model", "name_model_version", "name_model_arch", "name_model_version_arch"
        "distro_display": "name",

        // How to display disk usage information.
        // Options: "info", "percentage", "infobar", "barinfo", or "bar".
        "disk_display": "info",

        // What label to show for each disk.
        // Options: "name", "dir", "none", "mount"
        "disk_subtitle": "dir",

        // Show memory usage as a percentage.
        // options: true or false

        // Which unit to use for memory display.
        // Options: "mib", "gib", "kib"
        "memory_unit": "mib",

        // How to display package manager information.
        // Options: "off", "on", "tiny"
        "package_managers": "tiny",

        // Show the display's refresh rate.
        // options: true or false
        "refresh_rate": true,

        // Show the full path to your shell.
        // options: true or false
        "shell_path": true,

        // Show the version of your shell.
        // options: true or false
        "shell_version": true,

        // How to display system uptime.
        // Options: "full", "tiny", or "seconds".
        "uptime_shorthand": "tiny",

        // How to display os age.
        // Options: "full", "tiny", or "seconds".
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
        { "type": "custom", "format": "${reset}└────────────────────────────────────────────────┘" },
        // "break",
        { "type": "custom", "format": "${reset}┌────────────────────${c1}Hardware${reset}────────────────────┐" },
        { "type": "model", "key": "${reset}│${c1} ┌:"},
        { "type": "cpu", "key": "${reset}│${c1} ├:"},
        { "type": "gpu", "key": "${reset}│${c1} ├:"},
        { "type": "disk", "key": "${reset}│${c1} ├󰋊:"},
        { "type": "resolution", "key": "${reset}│${c1} ├󱄄:"},
        // { "type": "battery", "key": "${reset}│${c1} ├:"},
        { "type": "memory", "key": "${reset}│${c1} └:"},
        { "type": "custom", "format": "${reset}└────────────────────────────────────────────────┘" },
        // "break",
        { "type": "custom", "format": "${reset}┌───────────────────${c1}Age/Uptime${reset}───────────────────┐" },
        { "type": "os_age", "key": "${reset}│${c1} ┌󱦟 OS Age"},
        { "type": "uptime", "key": "${reset}│${c1} └󱫐 Uptime"},
        { "type": "custom", "format": "${reset}└────────────────────────────────────────────────┘" }
    ]
}

"#;
