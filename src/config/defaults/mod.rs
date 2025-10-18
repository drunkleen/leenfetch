/// Default configuration for the combined `config.jsonc` file.
///
/// The JSONC format accepts both standard JSON and `//` comments, mirroring what Fastfetch ships.
/// Users can keep everything in one place while still enjoying inline descriptions.
pub const DEFAULT_CONFIG: &str = r#"
// ===============================
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
        "battery_display": "barinfo",

        // String used to draw color blocks in the palette preview.
        "color_blocks": "███",

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
        "cpu_show_temp": true,

        // Show the version of your desktop environment.
        "de_version": true,

        // How much detail to show about your operating system.
        "distro_display": "name_model_arch",

        // How to display disk usage information.
        "disk_display": "barinfo",

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
        "uptime_shorthand": "full"
    },

    // -------------------------------
    // toggles - Enable or disable information blocks
    // -------------------------------
    "toggles": {
        "show_titles": true,
        "show_os": true,
        "show_distro": true,
        "show_model": true,
        "show_uptime": true,
        "show_packages": true,
        "show_shell": true,
        "show_wm": true,
        "show_de": true,
        "show_wm_theme": true,
        "show_kernel": true,
        "show_cpu": true,
        "show_gpu": true,
        "show_memory": true,
        "show_song": true,
        "show_resolution": true,
        "show_theme": true,
        "show_disks": true,
        "show_battery": true,
        "show_terminal_colors": true
    },

    // -------------------------------
    // layout - Output order and labels
    // -------------------------------
    "layout": [
        { "label": "Titles", "field": "titles" },
        { "label": "Distro", "field": "distro" },
        { "label": "Model", "field": "model" },
        { "label": "Kernel", "field": "kernel" },
        { "label": "Uptime", "field": "uptime" },
        { "label": "Packages", "field": "packages" },
        { "label": "Shell", "field": "shell" },
        { "label": "WM", "field": "wm" },
        { "label": "DE", "field": "de" },
        { "label": "WM theme", "field": "wm_theme" },
        { "label": "CPU", "field": "cpu" },
        { "label": "GPU", "field": "gpu" },
        { "label": "Memory", "field": "memory" },
        { "label": "Disk", "field": "disk" },
        { "label": "Resolution", "field": "resolution" },
        { "label": "Theme", "field": "theme" },
        { "label": "Battery", "field": "battery" },
        { "label": "", "field": "song" },
        { "label": "", "field": "colors" }
    ]
}
"#;
