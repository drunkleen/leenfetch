use serde::Deserialize;

/// Describes a single entry in the `modules` array.
/// Entries are either literal strings (used for `break`) or objects describing a module.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum LayoutItem {
    /// Matches the literal string "break" (case insensitive) to insert a blank line.
    Break(String),
    /// A normal module configuration entry.
    Module(ModuleEntry),
}

/// Toggles for enabling/disabling each info block in the output.
/// All fields are booleans. If true, the corresponding block is shown.
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
pub struct Flags {
    #[serde(default)]
    pub ascii_distro: String,         // Which distro's ASCII art to use ("auto" or a specific name)
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

/// Root configuration structure that mirrors the JSONC config file.
/// Missing sections fall back to their type defaults so that
/// users can keep only the keys they care about.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default, rename = "$schema")]
    #[allow(dead_code)]
    pub schema: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub logo: Option<Logo>,
    #[serde(default)]
    pub flags: Flags,
    #[serde(default)]
    pub toggles: Toggles,
    #[serde(default, alias = "layout", alias = "modules")]
    pub layout: Vec<LayoutItem>,
}

/// Minimal representation of the logo block. Extra fields are ignored automatically.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Logo {
    #[serde(rename = "type", default)]
    #[allow(dead_code)]
    pub logo_type: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub source: Option<String>,
}

/// Configuration for an individual info module.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct ModuleEntry {
    #[serde(rename = "type", default)]
    pub module_type: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
}

impl ModuleEntry {
    /// Returns the resolved field identifier used for data lookups.
    pub fn field_name(&self) -> Option<&str> {
        self.module_type
            .as_deref()
            .or(self.field.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    /// Returns the label to render for this module, if any.
    pub fn label(&self) -> Option<&str> {
        self.key.as_deref().or(self.label.as_deref())
    }

    /// Returns true when the module is a custom text block.
    pub fn is_custom(&self) -> bool {
        self.field_name()
            .map(|field| field.eq_ignore_ascii_case("custom"))
            .unwrap_or(false)
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            ascii_distro: "auto".into(),
            ascii_colors: "distro".into(),
            custom_ascii_path: String::new(),
            battery_display: "barinfo".into(),
            color_blocks: "███".into(),
            cpu_brand: true,
            cpu_cores: true,
            cpu_frequency: true,
            cpu_speed: true,
            cpu_temp: 'C',
            cpu_show_temp: true,
            de_version: true,
            distro_display: "name_model_arch".into(),
            disk_display: "barinfo".into(),
            disk_subtitle: "dir".into(),
            memory_percent: true,
            memory_unit: "mib".into(),
            package_managers: "tiny".into(),
            refresh_rate: true,
            shell_path: true,
            shell_version: true,
            uptime_shorthand: "full".into(),
        }
    }
}

impl Default for Toggles {
    fn default() -> Self {
        Self {
            show_titles: true,
            show_os: true,
            show_distro: true,
            show_model: true,
            show_uptime: true,
            show_packages: true,
            show_shell: true,
            show_wm: true,
            show_de: true,
            show_wm_theme: true,
            show_kernel: true,
            show_cpu: true,
            show_gpu: true,
            show_memory: true,
            show_song: true,
            show_resolution: true,
            show_theme: true,
            show_disks: true,
            show_battery: true,
            show_terminal_colors: true,
        }
    }
}
