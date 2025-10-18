use crate::modules::enums::SongInfo;

/// Holds all the collected system information for display.
/// Each field corresponds to a block or value in the output.
#[derive(Default, Clone, Debug)]
pub struct Data {
    /// Username of the current user (e.g., "snape")
    pub username: Option<String>,
    /// Hostname of the machine (e.g., "archbox")
    pub hostname: Option<String>,
    /// Base OS name (e.g., "Linux", "Windows")
    pub os: Option<String>,
    /// Distro string (format depends on DistroDisplay)
    pub distro: Option<String>,
    /// Hardware model (e.g., "ThinkPad X1")
    pub model: Option<String>,
    /// Kernel version (e.g., "6.14.6-arch1-1")
    pub kernel: Option<String>,
    /// Uptime string (format depends on UptimeShorthand)
    pub uptime: Option<String>,
    /// Package count or summary (format depends on PackageShorthand)
    pub packages: Option<String>,
    /// Shell name and/or version (e.g., "zsh 5.9")
    pub shell: Option<String>,
    /// Window manager name (e.g., "i3", "Mutter")
    pub wm: Option<String>,
    /// Desktop environment name (e.g., "GNOME", "KDE")
    pub de: Option<String>,
    /// CPU info string (model, speed, cores, etc.)
    pub cpu: Option<String>,
    /// List of GPU(s) detected
    pub gpu: Option<Vec<String>>,
    /// Memory usage string (format depends on Flags)
    pub memory: Option<String>,
    /// List of disk usage entries (label, usage string)
    pub disk: Option<Vec<(String, String)>>,
    /// Display resolution (e.g., "1920x1080 @ 60Hz")
    pub resolution: Option<String>,
    /// Theme name (GTK/Qt/DE/Windows)
    pub theme: Option<String>,
    /// List of battery info strings (format depends on BatteryDisplayMode)
    pub battery: Option<Vec<String>>,
    /// Currently playing song info (artist, album, title)
    pub song: Option<SongInfo>,
    /// Terminal color palette string
    pub colors: Option<String>,
}
