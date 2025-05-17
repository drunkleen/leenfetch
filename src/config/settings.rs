use serde::Deserialize;

/// Describes a single line/block in the output layout.
/// `label`: The label to display (e.g., "CPU").
/// `field`: The field name to match in the data (e.g., "cpu").
#[derive(Debug, Deserialize)]
pub struct Layout {
    pub label: String,
    pub field: String,
}

/// Toggles for enabling/disabling each info block in the output.
/// All fields are booleans. If true, the corresponding block is shown.
#[derive(Debug, Deserialize)]
pub struct Toggles {
    #[serde(default)]
    pub show_titles: bool,            // Show user@host title block
    #[serde(default)]
    pub show_os: bool,                // Show base OS name
    #[serde(default)]
    pub show_distro: bool,            // Show distribution string
    #[serde(default)]
    pub show_model: bool,             // Show hardware model
    #[serde(default)]
    pub show_uptime: bool,            // Show system uptime
    #[serde(default)]
    pub show_packages: bool,          // Show installed package count
    #[serde(default)]
    pub show_shell: bool,             // Show current shell name/version
    #[serde(default)]
    pub show_wm: bool,                // Show window manager
    #[serde(default)]
    pub show_de: bool,                // Show desktop environment
    #[serde(default)]
    pub show_wm_theme: bool,          // Show WM theme name
    #[serde(default)]
    pub show_kernel: bool,            // Show kernel version
    #[serde(default)]
    pub show_cpu: bool,               // Show CPU info
    #[serde(default)]
    pub show_gpu: bool,               // Show GPU(s)
    #[serde(default)]
    pub show_memory: bool,            // Show memory usage
    #[serde(default)]
    pub show_song: bool,              // Show current playing track
    #[serde(default)]
    pub show_resolution: bool,        // Show display resolution
    #[serde(default)]
    pub show_theme: bool,             // Show GTK/Qt/DE theme name
    #[serde(default)]
    pub show_disks: bool,             // Show disk usage
    #[serde(default)]
    pub show_battery: bool,           // Show battery info
    #[serde(default)]
    pub show_terminal_colors: bool,   // Show terminal color palette
}

/// Fine-grained configuration for how each block is displayed.
/// Most fields correspond to a feature or formatting option.
#[derive(Debug, Deserialize)]
pub struct Flags {
    #[serde(default)]
    pub ascii_distro: String,         // Which distro's ASCII art to use ("auto", "distro", or a specific name)
    #[serde(default)]
    pub ascii_colors: String,         // Which color palette to use ("distro", or a comma-separated list of color indices)
    #[serde(default)]
    pub battery_display: String,      // Battery display mode ("off", "bar", "infobar", "barinfo")
    #[serde(default)]
    pub color_blocks: String,         // String to use for color blocks (e.g., "███")
    #[serde(default)]
    pub cpu_brand: bool,              // Show CPU brand prefix (true/false)
    #[serde(default)]
    pub cpu_cores: bool,              // Show CPU core count (true/false)
    #[serde(default)]
    pub cpu_frequency: bool,          // Show CPU frequency (true/false)
    #[serde(default)]
    pub cpu_speed: bool,              // Show CPU speed (true/false)
    #[serde(default)]
    pub cpu_temp: char,               // Temperature unit ('C', 'F', or 'off')
    #[serde(default)]
    pub cpu_show_temp: bool,          // Show CPU temperature (true/false)
    #[serde(default)]
    pub custom_ascii_path: String,    // Path to custom ASCII art file
    #[serde(default)]
    pub de_version: bool,             // Show DE version (true/false)
    #[serde(default)]
    pub distro_display: String,       // Distro display format (see DistroDisplay enum)
    #[serde(default)]
    pub disk_display: String,         // Disk display format (see DiskDisplay enum)
    #[serde(default)]
    pub disk_subtitle: String,        // Disk subtitle format (see DiskSubtitle enum)
    #[serde(default)]
    pub memory_percent: bool,         // Show memory usage as percentage (true/false)
    #[serde(default)]
    pub memory_unit: String,          // Memory unit ("kib", "mib", "gib")
    #[serde(default)]
    pub package_managers: String,     // Package manager display (see PackageShorthand enum)
    #[serde(default)]
    pub refresh_rate: bool,           // Show display refresh rate (true/false)
    #[serde(default)]
    pub shell_path: bool,             // Show full shell path (true/false)
    #[serde(default)]
    pub shell_version: bool,          // Show shell version (true/false)
    #[serde(default)]
    pub uptime_shorthand: String,     // Uptime display format (see UptimeShorthand enum)
}
