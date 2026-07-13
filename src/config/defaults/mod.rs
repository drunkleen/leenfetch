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
        "custom_logo_path": "",
        // Scale factor for image logos. 1.0 keeps the original size.
        // Options: any positive float
        "logo_scale": 1.0,

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

        // Which temperature unit to use for CPU temperature, or off.
        // Options: "C", "F", "off"
        "cpu_temp": "C",

        // Show the version of your desktop environment.
        // options: true or false
        "de_version": true,

        // Show the GPU vendor name.
        // options: true or false
        "gpu_brand": true,

        // Which GPU to display.
        // Options: "all", "dedicated", "integrated"
        "gpu_type": "all",

        // Shorten the kernel output.
        // options: true or false
        "kernel_shorthand": true,

        // How much detail to show about your operating system.
        // Options: "name", "name_version", "name_arch", "name_model", "name_model_version", "name_model_arch", "name_model_version_arch"
        "distro_shorthand": "name",

        // How to display disk usage information.
        // Options: comma-separated list of "info", "percentage", and "bar".
        "disk_display": "info",

        // What label to show for each disk.
        // Options: "name", "dir", "none", "mount"
        "disk_subtitle": "dir",

        // Show disk usage percentage.
        // options: true or false
        "disk_percent": true,

        // Which disks to display.
        // Options: "/" or comma-separated mount points/paths
        "disk_show": "/",

        // Show memory usage as a percentage.
        // options: true or false
        "memory_percent": true,

        // Which unit to use for memory display.
        // Options: "mib", "gib", "kib"
        "memory_unit": "mib",

        // How to display package manager information.
        // Options: "off", "on", "tiny"
        "package_managers": "tiny",

        // Show the full path to your shell.
        // options: true or false
        "shell_path": false,

        // Show the version of your shell.
        // options: true or false
        "shell_version": true,

        // Show CPU speed without decimals.
        // options: true or false
        "speed_shorthand": false,

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
        { "type": "custom", "format": "${reset}╭─────────────────────${c1}System${reset}─────────────────────╮" },
        { "type": "titles", "key": "${reset}│${c1} ╭:"},
        // { "type": "os", "key": "${c16}│${c1} ├OS"},
        { "type": "distro", "key": "${reset}│${c1} ├:"},
        { "type": "kernel", "key": "${reset}│${c1} ├:"},
        { "type": "packages", "key": "${reset}│${c1} ├󰏖:"},
        { "type": "shell", "key": "${reset}│ ${c1}├:"},
        { "type": "wm", "key": "${reset}│${c1} ├:"},
        { "type": "de", "key": "${reset}│${c1} ├󰇄:"},
        // { "type": "song", "key": "${reset}│${c1} ├"},
        { "type": "theme", "key": "${reset}│${c1} ├󰸌:"},
        { "type": "colors", "key": "${reset}│${c1} ╰:"},
        { "type": "custom", "format": "${reset}╰────────────────────────────────────────────────╯" },
        // "break",
        { "type": "custom", "format": "${reset}╭────────────────────${c1}Hardware${reset}────────────────────╮" },
        { "type": "model", "key": "${reset}│${c1} ╭:"},
        { "type": "cpu", "key": "${reset}│${c1} ├:"},
        { "type": "gpu", "key": "${reset}│${c1} ├:"},
        { "type": "disk", "key": "${reset}│${c1} ├󰋊:"},
        { "type": "resolution", "key": "${reset}│${c1} ├󱄄:"},
        // { "type": "battery", "key": "${reset}│${c1} ├:"},
        { "type": "memory", "key": "${reset}│${c1} ╰:"},
        { "type": "custom", "format": "${reset}╰────────────────────────────────────────────────╯" },
        // "break",
        { "type": "custom", "format": "${reset}╭───────────────────${c1}Age/Uptime${reset}───────────────────╮" },
        { "type": "os_age", "key": "${reset}│${c1} ╭󱦟 OS Age"},
        { "type": "uptime", "key": "${reset}│${c1} ╰󱫐 Uptime"},
        { "type": "custom", "format": "${reset}╰────────────────────────────────────────────────╯" }
    ]
}
"#;
pub const CLASSIC_NEOFETCH_CONFIG: &str = r#"// ===============================
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
        "custom_logo_path": "",
        "logo_scale": 1.0,

        // How to display battery information.
        // Options: "off", "bar", "infobar", "barinfo"
        "battery_display": "off",

        // String used to draw color blocks in the palette preview.
        "color_blocks": "██",

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

        // Which temperature unit to use for CPU temperature, or off.
        // Options: "C", "F", "off"
        "cpu_temp": "C",

        // Show the version of your desktop environment.
        // options: true or false
        "de_version": true,

        // Show the GPU vendor name.
        // options: true or false
        "gpu_brand": true,

        // Which GPU to display.
        // Options: "all", "dedicated", "integrated"
        "gpu_type": "all",

        // Shorten the kernel output.
        // options: true or false
        "kernel_shorthand": true,

        // How much detail to show about your operating system.
        // Options: "name", "name_version", "name_arch", "name_model", "name_model_version", "name_model_arch", "name_model_version_arch"
        "distro_shorthand": "name",

        // How to display disk usage information.
        // Options: comma-separated list of "info", "percentage", and "bar".
        "disk_display": "info",

        // What label to show for each disk.
        // Options: "name", "dir", "none", "mount"
        "disk_subtitle": "dir",

        // Show disk usage percentage.
        // options: true or false
        "disk_percent": true,

        // Which disks to display.
        // Options: "/" or comma-separated mount points
        "disk_show": "/",

        // Show memory usage as a percentage.
        // options: true or false
        "memory_percent": true,

        // Which unit to use for memory display.
        // Options: "mib", "gib", "kib"
        "memory_unit": "mib",

        // How to display package manager information.
        // Options: "off", "on", "tiny"
        "package_managers": "tiny",

        // Show the full path to your shell.
        // options: true or false
        "shell_path": false,

        // Show the version of your shell.
        // options: true or false
        "shell_version": true,

        // Show CPU speed without decimals.
        // options: true or false
        "speed_shorthand": false,

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
        { "type": "titles", "key": "${reset}${c1}"},
        { "type": "custom", "format": "${reset}----------" },
        { "type": "model", "key": "${c1}Host:"},
        // { "type": "os", "key": "${c1}OS:"},
        { "type": "distro", "key": "${c1}Distro:"},
        { "type": "kernel", "key": "${c1}Kernel:"},
        { "type": "packages", "key": "${c1}Packages:"},
        { "type": "shell", "key": "${c1}Shell:"},
        // { "type": "song", "key": "${reset}│${c1} ├"},
        { "type": "cpu", "key": "${c1}CPU:"},
        { "type": "gpu", "key": "${c1}GPU:"},
        { "type": "memory", "key": "${c1}Memory:"},
        { "type": "disk", "key": "${c1}Disk:"},
        // { "type": "battery", "key": "${c1}Battery:"},
        { "type": "os_age", "key": "${c1}OS Age:"},
        { "type": "uptime", "key": "${c1}Uptime:"},
        { "type": "resolution", "key": "${c1}Resolution:"},
        { "type": "wm", "key": "${c1}WM:"},
        { "type": "de", "key": "${c1}DE:"},
        { "type": "theme", "key": "${c1}Theme:"},
        { "type": "colors", "key": "${c1}Colors:"}
    ]
}
"#;

pub const NEOFETCH_CONFIG: &str = r#"// ===============================
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
        "custom_logo_path": "",
        "logo_scale": 1.0,

        // How to display battery information.
        // Options: "off", "bar", "infobar", "barinfo"
        "battery_display": "off",

        // String used to draw color blocks in the palette preview.
        "color_blocks": "██",

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

        // Which temperature unit to use for CPU temperature, or off.
        // Options: "C", "F", "off"
        "cpu_temp": "C",

        // Show the version of your desktop environment.
        // options: true or false
        "de_version": true,

        // Show the GPU vendor name.
        // options: true or false
        "gpu_brand": true,

        // Which GPU to display.
        // Options: "all", "dedicated", "integrated"
        "gpu_type": "all",

        // Shorten the kernel output.
        // options: true or false
        "kernel_shorthand": true,

        // How much detail to show about your operating system.
        // Options: "name", "name_version", "name_arch", "name_model", "name_model_version", "name_model_arch", "name_model_version_arch"
        "distro_shorthand": "name",

        // How to display disk usage information.
        // Options: comma-separated list of "info", "percentage", and "bar".
        "disk_display": "info",

        // What label to show for each disk.
        // Options: "name", "dir", "none", "mount"
        "disk_subtitle": "dir",

        // Show disk usage percentage.
        // options: true or false
        "disk_percent": true,

        // Which disks to display.
        // Options: "/" or comma-separated mount points
        "disk_show": "/",

        // Show memory usage as a percentage.
        // options: true or false
        "memory_percent": true,

        // Which unit to use for memory display.
        // Options: "mib", "gib", "kib"
        "memory_unit": "mib",

        // How to display package manager information.
        // Options: "off", "on", "tiny"
        "package_managers": "tiny",

        // Show the full path to your shell.
        // options: true or false
        "shell_path": false,

        // Show the version of your shell.
        // options: true or false
        "shell_version": true,

        // Show CPU speed without decimals.
        // options: true or false
        "speed_shorthand": false,

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
        { "type": "titles", "key": "${reset}${c1}"},
        { "type": "custom", "format": "${reset}----------" },
        { "type": "model", "key": "${c1}Host:"},
        // { "type": "os", "key": "${c1}OS:"},
        { "type": "distro", "key": "${c1}Distro:"},
        { "type": "kernel", "key": "${c1}Kernel:"},
        { "type": "packages", "key": "${c1}Packages:"},
        { "type": "shell", "key": "${c1}Shell:"},
        // { "type": "song", "key": "${reset}│${c1} ├"},
        { "type": "cpu", "key": "${c1}CPU:"},
        { "type": "gpu", "key": "${c1}GPU:"},
        { "type": "memory", "key": "${c1}Memory:"},
        { "type": "disk", "key": "${c1}Disk:"},
        // { "type": "battery", "key": "${c1}Battery:"},
        { "type": "os_age", "key": "${c1}OS Age:"},
        { "type": "uptime", "key": "${c1}Uptime:"},
        { "type": "resolution", "key": "${c1}Resolution:"},
        { "type": "wm", "key": "${c1}WM:"},
        { "type": "de", "key": "${c1}DE:"},
        { "type": "theme", "key": "${c1}Theme:"},
        { "type": "colors", "key": "${c1}Colors:"}
    ]
}
"#;

pub const MINIMAL_CONFIG: &str = r#"{
    "flags": {
        "ascii_distro": "auto",
        "ascii_colors": "distro",
        "custom_logo_path": "",
        "logo_scale": 1.0,
        "battery_display": "off",
        "color_blocks": "●",
        "cpu_brand": true,
        "cpu_cores": false,
        "cpu_frequency": false,
        "cpu_speed": true,
        "cpu_temp": "C",
        "de_version": false,
        "gpu_brand": true,
        "gpu_type": "all",
        "kernel_shorthand": true,
        "distro_shorthand": "name",
        "disk_display": "info",
        "disk_subtitle": "dir",
        "disk_percent": false,
        "disk_show": "/",
        "memory_percent": true,
        "memory_unit": "mib",
        "package_managers": "off",
        "shell_path": false,
        "shell_version": false,
        "speed_shorthand": true,
        "uptime_shorthand": "tiny",
        "os_age_shorthand": "tiny"
    },
    "modules": [
        { "type": "titles", "key": "${reset}${c1}" },
        { "type": "distro", "key": "${c1}Distro:" },
        { "type": "kernel", "key": "${c1}Kernel:" },
        { "type": "memory", "key": "${c1}Memory:" },
        { "type": "uptime", "key": "${c1}Uptime:" }
    ]
}"#;

pub const SCREENSHOT_CONFIG: &str = r#"{
    "flags": {
        "ascii_distro": "auto",
        "ascii_colors": "distro",
        "custom_logo_path": "",
        "logo_scale": 1.0,
        "battery_display": "off",
        "color_blocks": "██",
        "cpu_brand": true,
        "cpu_cores": true,
        "cpu_frequency": true,
        "cpu_speed": true,
        "cpu_temp": "C",
        "de_version": true,
        "gpu_brand": true,
        "gpu_type": "all",
        "kernel_shorthand": true,
        "distro_shorthand": "name",
        "disk_display": "bar, info",
        "disk_subtitle": "dir",
        "disk_percent": true,
        "disk_show": "/",
        "memory_percent": true,
        "memory_unit": "mib",
        "package_managers": "tiny",
        "shell_path": false,
        "shell_version": true,
        "speed_shorthand": false,
        "uptime_shorthand": "tiny",
        "os_age_shorthand": "tiny"
    },
    "modules": [
        { "type": "titles", "key": "${reset}│${c1} ╭:" },
        { "type": "distro", "key": "${reset}│${c1} ├:" },
        { "type": "kernel", "key": "${reset}│${c1} ├:" },
        { "type": "packages", "key": "${reset}│${c1} ├󰏖:" },
        { "type": "shell", "key": "${reset}│ ${c1}├:" },
        { "type": "wm", "key": "${reset}│${c1} ├:" },
        { "type": "de", "key": "${reset}│${c1} ├󰇄:" },
        { "type": "theme", "key": "${reset}│${c1} ├󰸌:" },
        { "type": "colors", "key": "${reset}│${c1} ╰:" },
        { "type": "model", "key": "${reset}│${c1} ╭:" },
        { "type": "cpu", "key": "${reset}│${c1} ├:" },
        { "type": "gpu", "key": "${reset}│${c1} ├:" },
        { "type": "memory", "key": "${reset}│${c1} ╰:" },
        { "type": "disk", "key": "${reset}│${c1} ├󰋊:" },
        { "type": "resolution", "key": "${reset}│${c1} ├󱄄:" },
        { "type": "os_age", "key": "${reset}│${c1} ╭󱦟 OS Age" },
        { "type": "uptime", "key": "${reset}│${c1} ╰󱫐 Uptime" }
    ]
}"#;

pub const HARDWARE_HEAVY_CONFIG: &str = r#"{
    "flags": {
        "ascii_distro": "auto",
        "ascii_colors": "distro",
        "custom_logo_path": "",
        "logo_scale": 1.0,
        "battery_display": "off",
        "color_blocks": "●",
        "cpu_brand": true,
        "cpu_cores": true,
        "cpu_frequency": true,
        "cpu_speed": true,
        "cpu_temp": "C",
        "de_version": false,
        "gpu_brand": true,
        "gpu_type": "all",
        "kernel_shorthand": true,
        "distro_shorthand": "name_model_version",
        "disk_display": "bar, info",
        "disk_subtitle": "dir",
        "disk_percent": true,
        "disk_show": "/",
        "memory_percent": true,
        "memory_unit": "mib",
        "package_managers": "off",
        "shell_path": false,
        "shell_version": false,
        "speed_shorthand": false,
        "uptime_shorthand": "tiny",
        "os_age_shorthand": "tiny"
    },
    "modules": [
        { "type": "titles", "key": "${reset}${c1}" },
        { "type": "model", "key": "${c1}Host:" },
        { "type": "cpu", "key": "${c1}CPU:" },
        { "type": "gpu", "key": "${c1}GPU:" },
        { "type": "memory", "key": "${c1}Memory:" },
        { "type": "disk", "key": "${c1}Disk:" },
        { "type": "resolution", "key": "${c1}Resolution:" }
    ]
}"#;

pub const SYSTEM_ADMIN_CONFIG: &str = r#"{
    "flags": {
        "ascii_distro": "auto",
        "ascii_colors": "distro",
        "custom_logo_path": "",
        "logo_scale": 1.0,
        "battery_display": "off",
        "color_blocks": "●",
        "cpu_brand": true,
        "cpu_cores": true,
        "cpu_frequency": true,
        "cpu_speed": true,
        "cpu_temp": "C",
        "de_version": true,
        "gpu_brand": true,
        "gpu_type": "all",
        "kernel_shorthand": true,
        "distro_shorthand": "name_version",
        "disk_display": "info",
        "disk_subtitle": "mount",
        "disk_percent": true,
        "disk_show": "/",
        "memory_percent": true,
        "memory_unit": "mib",
        "package_managers": "on",
        "shell_path": true,
        "shell_version": true,
        "speed_shorthand": false,
        "uptime_shorthand": "full",
        "os_age_shorthand": "full"
    },
    "modules": [
        { "type": "titles", "key": "${reset}${c1}" },
        { "type": "distro", "key": "${c1}Distro:" },
        { "type": "kernel", "key": "${c1}Kernel:" },
        { "type": "packages", "key": "${c1}Packages:" },
        { "type": "shell", "key": "${c1}Shell:" },
        { "type": "os_age", "key": "${c1}OS Age:" },
        { "type": "uptime", "key": "${c1}Uptime:" },
        { "type": "memory", "key": "${c1}Memory:" },
        { "type": "disk", "key": "${c1}Disk:" }
    ]
}"#;

pub const CLEAN_MONO_CONFIG: &str = r#"{
    "flags": {
        "ascii_distro": "auto",
        "ascii_colors": "7",
        "custom_logo_path": "",
        "logo_scale": 1.0,
        "battery_display": "off",
        "color_blocks": "██",
        "cpu_brand": true,
        "cpu_cores": false,
        "cpu_frequency": false,
        "cpu_speed": false,
        "cpu_temp": "off",
        "de_version": false,
        "gpu_brand": false,
        "gpu_type": "all",
        "kernel_shorthand": true,
        "distro_shorthand": "name",
        "disk_display": "info",
        "disk_subtitle": "none",
        "disk_percent": false,
        "disk_show": "/",
        "memory_percent": true,
        "memory_unit": "mib",
        "package_managers": "off",
        "shell_path": false,
        "shell_version": false,
        "speed_shorthand": true,
        "uptime_shorthand": "tiny",
        "os_age_shorthand": "tiny"
    },
    "modules": [
        { "type": "titles", "key": "${reset}${c1}" },
        { "type": "distro", "key": "${c1}Distro:" },
        { "type": "kernel", "key": "${c1}Kernel:" },
        { "type": "memory", "key": "${c1}Memory:" },
        { "type": "uptime", "key": "${c1}Uptime:" }
    ]
}"#;

pub const TINY_CONFIG: &str = r#"{
    "flags": {
        "ascii_distro": "off",
        "ascii_colors": "distro",
        "custom_logo_path": "",
        "logo_scale": 1.0,
        "battery_display": "off",
        "color_blocks": "●",
        "cpu_brand": false,
        "cpu_cores": false,
        "cpu_frequency": false,
        "cpu_speed": false,
        "cpu_temp": "off",
        "de_version": false,
        "gpu_brand": false,
        "gpu_type": "all",
        "kernel_shorthand": true,
        "distro_shorthand": "name",
        "disk_display": "info",
        "disk_subtitle": "none",
        "disk_percent": false,
        "disk_show": "/",
        "memory_percent": true,
        "memory_unit": "mib",
        "package_managers": "off",
        "shell_path": false,
        "shell_version": false,
        "speed_shorthand": true,
        "uptime_shorthand": "tiny",
        "os_age_shorthand": "tiny"
    },
    "modules": [
        { "type": "titles", "key": "${reset}${c1}" },
        { "type": "distro", "key": "${c1}Distro:" },
        { "type": "kernel", "key": "${c1}Kernel:" },
        { "type": "uptime", "key": "${c1}Uptime:" }
    ]
}"#;
