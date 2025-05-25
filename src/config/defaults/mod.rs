/// Default configuration for the [flags.ron] file.
///
/// This string is used to generate the default `flags.ron` config file, which controls fine-grained display and formatting options for each block in the output.
/// Each field corresponds to a field in the `Flags` struct (see `src/config/settings.rs`).
///
/// - String values (e.g., "auto", "barinfo") select display modes or formatting (see enums in `enums.rs`).
/// - Boolean values (true/false) enable or disable features.
/// - See comments in `Flags` for details on each option.
pub const DEFAULT_FLAGS: &str = r#"
// ===============================
// flags.ron - Display and formatting options
//
// This file controls how each block of information is shown in the leenfetch output.
// You can customize the look, detail, and formatting of every section.
// Change the values below to adjust what is displayed and how it appears.
//
// - String values select display modes or formatting.
// - true/false values enable or disable features.
//
// For a full explanation of each option, see the comments above each line.
// ===============================
(
    // Select which distribution's ASCII art to display at the top.
    // Options:
    //   "auto"   - Automatically detect and use your current distribution's ASCII art.
    //   <name>    - Use a specific distro's art (e.g., "arch", "ubuntu", "debian").
    ascii_distro: "auto",

    // Choose the color palette for the ASCII art and color blocks.
    // Options:
    //   "distro" - Use your distribution's default color palette.
    //   <list>    - Comma-separated list of color numbers (e.g., "1,2,3,4,5,6,7,8").
    ascii_colors: "distro",

    // Path to a custom ASCII art file to display instead of the default.
    // Options:
    //   ""        - Use the built-in ASCII art.
    //   <path>    - Path to a custom ASCII art file (e.g., "/home/user/myart.txt").
    custom_ascii_path: "",

    // How to display battery information.
    // Options:
    //   "off"     - Do not show battery information.
    //   "bar"     - Show only a visual bar for battery level.
    //   "infobar" - Show battery percentage and a bar.
    //   "barinfo" - Show a bar, percentage, and charging status (default).
    battery_display: "barinfo",

    // The string used to draw color blocks in the palette preview.
    // You can change this to any string, such as "▓▓▓" or "\#\#\#".
    color_blocks: "███",

    // Show the brand name of your CPU.
    // Options: true (show), false (hide)
    cpu_brand: true,

    // Show the number of CPU cores.
    // Options: true (show), false (hide)
    cpu_cores: true,

    // Show the CPU frequency.
    // Options: true (show), false (hide)
    cpu_frequency: true,

    // Show the current CPU speed.
    // Options: true (show), false (hide)
    cpu_speed: true,

    // Which temperature unit to use for CPU temperature.
    // Options:
    //   'C' - Celsius
    //   'F' - Fahrenheit
    cpu_temp: 'C',

    // Show the CPU temperature.
    // Options: true (show), false (hide)
    cpu_show_temp: true,

    // Show the version of your desktop environment.
    // Options: true (show), false (hide)
    de_version: true,

    // How much detail to show about your operating system.
    // Options:
    //   "name"                - Only the OS name (e.g., "Arch Linux").
    //   "name_version"        - Name and version (e.g., "Arch Linux 2024.06").
    //   "name_arch"           - Name and architecture (e.g., "Arch Linux x86_64").
    //   "name_model"          - Name and model (e.g., "Arch Linux LTS").
    //   "name_model_version"  - Name, model, and version.
    //   "name_model_arch"     - Name, model, and architecture (default).
    //   "name_model_version_arch" - Name, model, version, and architecture.
    distro_display: "name_model_arch",

    // How to display disk usage information.
    // Options:
    //   "info"      - Show used/total (e.g., "40G / 100G").
    //   "percentage"- Show percent used and a bar (e.g., "40% [███]").
    //   "infobar"   - Show used/total and a bar (e.g., "40G / 100G [███]").
    //   "barinfo"   - Show bar first, then used/total (default).
    //   "bar"       - Show only the bar.
    disk_display: "barinfo",

    // What label to show for each disk.
    // Options:
    //   "name"  - Show the device name (e.g., /dev/sda1).
    //   "dir"   - Show the last directory in the mount path (e.g., "home" for /home) (default).
    //   "none"  - No subtitle.
    //   "mount" - Show the full mount point (e.g., /home).
    disk_subtitle: "dir",

    // Show memory usage as a percentage.
    // Options: true (show percent), false (show only amount used)
    memory_percent: true,

    // Which unit to use for memory display.
    // Options:
    //   "mib" - Mebibytes (default)
    //   "gib" - Gibibytes
    //   "kib" - Kibibytes
    memory_unit: "mib",

    // How to display package manager information.
    // Options:
    //   "off"  - Only show the total package count.
    //   "on"   - Show a list of package managers and their counts.
    //   "tiny" - Show only the count and a compact list of manager names.
    package_managers: "tiny",

    // Show the display's refresh rate.
    // Options: true (show), false (hide)
    refresh_rate: true,

    // Show the full path to your shell.
    // Options: true (show full path), false (show only shell name)
    shell_path: true,

    // Show the version of your shell.
    // Options: true (show), false (hide)
    shell_version: true,

    // How to display system uptime.
    // Options:
    //   "full"    - Verbose (e.g., "1 day, 2 hours, 55 minutes").
    //   "tiny"    - Compact (e.g., "1d 2h 55m").
    //   "seconds" - Only seconds (e.g., "123456s").
    uptime_shorthand: "full",
)
"#;

/// Default configuration for the [toggles.ron] file.
///
/// This string is used to generate the default `toggles.ron` config file, which controls which information blocks are shown in the output.
/// Each field corresponds to a boolean in the `Toggles` struct (see `src/config/settings.rs`).
/// If true, the corresponding block (e.g., CPU, GPU, Memory) will be displayed.
pub const DEFAULT_TOGGLES: &str = r#"
// ===============================
// toggles.ron - Show/hide information blocks
//
// This file controls which blocks of information are shown in the leenfetch output.
// Set each option to true to show that block, or false to hide it.
//
// For example, you can hide the battery or song info if you don't want them displayed.
//
// For a full explanation of each option, see the comments above each line.
// ===============================
(
    // Show the user@host title at the top of the output.
    // true  - Display the title block (e.g., "snape@archbox").
    // false - Hide the title block.
    show_titles: true,

    // Show the base operating system name (e.g., "Linux").
    // true  - Display the OS name.
    // false - Hide the OS name.
    show_os: true,

    // Show the distribution (distro) information.
    // true  - Display the distro string (e.g., "Arch Linux").
    // false - Hide the distro info.
    show_distro: true,

    // Show the hardware model (e.g., laptop/desktop model).
    // true  - Display the model info.
    // false - Hide the model info.
    show_model: true,

    // Show the system uptime.
    // true  - Display how long the system has been running.
    // false - Hide uptime info.
    show_uptime: true,

    // Show the package count and/or package manager info.
    // true  - Display package information.
    // false - Hide package info.
    show_packages: true,

    // Show the shell name and version.
    // true  - Display shell info.
    // false - Hide shell info.
    show_shell: true,

    // Show the window manager (WM) name.
    // true  - Display WM info.
    // false - Hide WM info.
    show_wm: true,

    // Show the desktop environment (DE) name and version.
    // true  - Display DE info.
    // false - Hide DE info.
    show_de: true,

    // Show the window manager theme.
    // true  - Display WM theme info.
    // false - Hide WM theme info.
    show_wm_theme: true,

    // Show the kernel version.
    // true  - Display kernel info.
    // false - Hide kernel info.
    show_kernel: true,

    // Show CPU information (brand, cores, speed, etc.).
    // true  - Display CPU info.
    // false - Hide CPU info.
    show_cpu: true,

    // Show GPU information.
    // true  - Display GPU info.
    // false - Hide GPU info.
    show_gpu: true,

    // Show memory usage.
    // true  - Display memory info.
    // false - Hide memory info.
    show_memory: true,

    // Show currently playing song/media info.
    // true  - Display song info if available.
    // false - Hide song info.
    show_song: true,

    // Show display resolution.
    // true  - Display resolution info.
    // false - Hide resolution info.
    show_resolution: true,

    // Show the current GTK/Qt theme.
    // true  - Display theme info.
    // false - Hide theme info.
    show_theme: true,

    // Show disk usage information.
    // true  - Display disk info.
    // false - Hide disk info.
    show_disks: true,

    // Show battery status and level.
    // true  - Display battery info.
    // false - Hide battery info.
    show_battery: true,

    // Show the terminal color palette preview.
    // true  - Display color blocks at the bottom.
    // false - Hide color blocks.
    show_terminal_colors: true,
)
"#;

/// Default configuration for the [print_layout.ron] file.
///
/// This string is used to generate the default `print_layout.ron` config file, which controls the order and labels of blocks in the output.
/// Each entry is a `Layout` struct (see `src/config/settings.rs`), with a `label` (displayed in output) and a `field` (corresponding to a field in `Data`).
pub const DEFAULT_PRINT_LAYOUT: &str = r#"
// ===============================
// print_layout.ron - Output order and labels
//
// This file controls the order and labels of each block in the leenfetch output.
// You can rearrange, remove, or relabel any section to customize your output.
//
// - The 'label' is what is shown before the value (e.g., "CPU:").
// - The 'field' determines which information is shown in that row.
//
// For a full explanation of each entry, see the comments above each line.
// ===============================
[
    // The user@host title block (e.g., "snape@archbox").
    // You can change the label or move this entry to change its position in the output.
    (label: "Titles", field: "titles"),

    // The distribution (distro) information (e.g., "Arch Linux").
    // The label is what will be shown before the value.
    (label: "Distro", field: "distro"),

    // The hardware model (e.g., laptop/desktop model).
    (label: "Model", field: "model"),

    // The kernel version (e.g., "6.14.6-arch1-1").
    (label: "Kernel", field: "kernel"),

    // The system uptime (how long the system has been running).
    (label: "Uptime", field: "uptime"),

    // The package count and/or package manager info.
    (label: "Packages", field: "packages"),

    // The shell name and version.
    (label: "Shell", field: "shell"),

    // The window manager (WM) name.
    (label: "WM", field: "wm"),

    // The desktop environment (DE) name and version.
    (label: "DE", field: "de"),

    // The window manager theme.
    (label: "WM theme", field: "wm_theme"),

    // CPU information (brand, cores, speed, etc.).
    (label: "CPU", field: "cpu"),

    // GPU information.
    (label: "GPU", field: "gpu"),

    // Memory usage.
    (label: "Memory", field: "memory"),

    // Disk usage information.
    (label: "Disk", field: "disk"),

    // Display resolution.
    (label: "Resolution", field: "resolution"),

    // The current GTK/Qt theme.
    (label: "Theme", field: "theme"),

    // Battery status and level.
    (label: "Battery", field: "battery"),

    // Currently playing song/media info (if available).
    // The label is left empty to avoid clutter, but you can set a custom label if you wish.
    (label: "", field: "song"),

    // Terminal color palette preview (color blocks at the bottom).
    // The label is left empty to avoid clutter, but you can set a custom label if you wish.
    (label: "", field: "colors"),


    
    // You can add more custom entries as needed.
    // (label: "- Custom Field", field: "custom text"),
]
"#;
