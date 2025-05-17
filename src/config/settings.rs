use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Layout {
    pub label: String,
    pub field: String,
}

#[derive(Debug, Deserialize)]
pub struct Toggles {
    #[serde(default)]
    pub show_titles: bool,
    #[serde(default)]
    pub show_os: bool,
    #[serde(default)]
    pub show_distro: bool,
    #[serde(default)]
    pub show_model: bool,
    #[serde(default)]
    pub show_uptime: bool,
    #[serde(default)]
    pub show_packages: bool,
    #[serde(default)]
    pub show_shell: bool,
    #[serde(default)]
    pub show_wm: bool,
    #[serde(default)]
    pub show_de: bool,
    #[serde(default)]
    pub show_wm_theme: bool,
    #[serde(default)]
    pub show_kernel: bool,
    #[serde(default)]
    pub show_cpu: bool,
    #[serde(default)]
    pub show_gpu: bool,
    #[serde(default)]
    pub show_memory: bool,
    #[serde(default)]
    pub show_song: bool,
    #[serde(default)]
    pub show_resolution: bool,
    #[serde(default)]
    pub show_theme: bool,
    #[serde(default)]
    pub show_disks: bool,
    #[serde(default)]
    pub show_battery: bool,
    #[serde(default)]
    pub show_terminal_colors: bool,
}

#[derive(Debug, Deserialize)]
pub struct Flags {
    #[serde(default)]
    pub ascii_distro: String,
    #[serde(default)]
    pub ascii_colors: String,
    #[serde(default)]
    pub battery_display: String,
    #[serde(default)]
    pub color_blocks: String,
    #[serde(default)]
    pub cpu_brand: bool,
    #[serde(default)]
    pub cpu_cores: bool,
    #[serde(default)]
    pub cpu_frequency: bool,
    #[serde(default)]
    pub cpu_speed: bool,
    #[serde(default)]
    pub cpu_temp: char,
    #[serde(default)]
    pub cpu_show_temp: bool,
    #[serde(default)]
    pub custom_ascii_path: String,
    #[serde(default)]
    pub de_version: bool,
    #[serde(default)]
    pub distro_display: String,
    #[serde(default)]
    pub disk_display: String,
    #[serde(default)]
    pub disk_subtitle: String,
    #[serde(default)]
    pub memory_percent: bool,
    #[serde(default)]
    pub memory_unit: String,
    #[serde(default)]
    pub package_managers: String,
    #[serde(default)]
    pub refresh_rate: bool,
    #[serde(default)]
    pub shell_path: bool,
    #[serde(default)]
    pub shell_version: bool,
    #[serde(default)]
    pub uptime_shorthand: String,
}
