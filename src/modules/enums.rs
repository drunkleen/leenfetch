use std::str::FromStr;

/// Holds information about the currently playing song, if available.
/// Used for the media/song block in the output.
///
/// - `artist`: Name of the artist (e.g., "Radiohead")
/// - `album`: Name of the album (e.g., "OK Computer")
/// - `title`: Title of the song (e.g., "Paranoid Android")
#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct SongInfo {
    pub artist: String,
    pub album: String,
    pub title: String,
}

/// Controls how the package manager summary is displayed in the output.
/// - Off: Only show the total package count.
/// - On: Show a list of package managers and their counts.
/// - Tiny: Show only the count and a compact list of manager names.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackageShorthand {
    Off,
    On,
    Tiny,
}

impl FromStr for PackageShorthand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "off" => Ok(PackageShorthand::Off),
            "on" => Ok(PackageShorthand::On),
            "tiny" => Ok(PackageShorthand::Tiny),
            _ => Ok(PackageShorthand::On),
        }
    }
}

/// Controls the format of the distribution (distro) string in the output.
/// Used to customize how much detail about the OS is shown.
/// - Name: Only the distro name (e.g., "Arch Linux").
/// - NameVersion: Name and version.
/// - NameArch: Name and architecture.
/// - NameModel: Name and model (e.g., LTS, Rolling).
/// - NameModelVersion: Name, model, and version.
/// - NameModelArch: Name, model, and architecture.
/// - NameModelVersionArch: Name, model, version, and architecture. (default)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistroDisplay {
    Name,
    NameVersion,
    NameArch,
    NameModel,
    NameModelVersion,
    NameModelArch,
    NameModelVersionArch,
}

impl std::str::FromStr for DistroDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "name" => Ok(DistroDisplay::Name),
            "name_version" => Ok(DistroDisplay::NameVersion),
            "name_arch" => Ok(DistroDisplay::NameArch),
            "name_model" => Ok(DistroDisplay::NameModel),
            "name_model_version" => Ok(DistroDisplay::NameModelVersion),
            "name_model_arch" => Ok(DistroDisplay::NameModelArch),
            "name_model_version_arch" => Ok(DistroDisplay::NameModelVersionArch),
            _ => Err(()),
        }
    }
}

/// Controls the format of the uptime string in the output.
/// - Full: Verbose (e.g., "1 day, 2 hours, 55 minutes"). (default)
/// - Tiny: Compact (e.g., "1d 2h 55m").
/// - Seconds: Only seconds (e.g., "123456s").
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UptimeShorthand {
    Full,
    Tiny,
    Seconds,
}

impl FromStr for UptimeShorthand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "full" => Ok(UptimeShorthand::Full),
            "tiny" => Ok(UptimeShorthand::Tiny),
            "seconds" => Ok(UptimeShorthand::Seconds),
            _ => Ok(UptimeShorthand::Full),
        }
    }
}

/// Controls the unit used to display memory usage.
/// - MiB: Mebibytes (default).
/// - GiB: Gibibytes.
/// - KiB: Kibibytes.
#[derive(Debug, Clone, Copy)]
pub enum MemoryUnit {
    MiB,
    GiB,
    KiB,
}

impl FromStr for MemoryUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "mib" => Ok(MemoryUnit::MiB),
            "gib" => Ok(MemoryUnit::GiB),
            "kib" => Ok(MemoryUnit::KiB),
            _ => Ok(MemoryUnit::MiB),
        }
    }
}

/// Controls the subtitle/label shown for each disk in the disk usage output.
/// - Name: Show the device name (e.g., /dev/sda1).
/// - Dir: Show the last directory in the mount path (e.g., "home" for /home). (default)
/// - None: No subtitle.
/// - Mount: Show the full mount point (e.g., /home).
#[derive(Debug, Clone, Copy)]
pub enum DiskSubtitle {
    Name,
    Dir,
    None,
    Mount,
}

impl FromStr for DiskSubtitle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "name" => Ok(DiskSubtitle::Name),
            "dir" => Ok(DiskSubtitle::Dir),
            "none" => Ok(DiskSubtitle::None),
            "mount" => Ok(DiskSubtitle::Mount),
            _ => Ok(DiskSubtitle::Dir),
        }
    }
}

/// Controls the format of the disk usage display.
/// - Info: Show used/total (e.g., "40G / 100G").
/// - Percentage: Show percent used and a bar (e.g., "40% [███]"). (default)
/// - InfoBar: Show used/total and a bar (e.g., "40G / 100G [███]").
/// - BarInfo: Show bar first, then used/total.
/// - Bar: Show only the bar.
pub enum DiskDisplay {
    Info,
    Percentage,
    InfoBar,
    BarInfo,
    Bar,
}

impl FromStr for DiskDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "info" => Ok(DiskDisplay::Info),
            "percentage" => Ok(DiskDisplay::Percentage),
            "infobar" => Ok(DiskDisplay::InfoBar),
            "barinfo" => Ok(DiskDisplay::BarInfo),
            "bar" => Ok(DiskDisplay::Bar),
            _ => Ok(DiskDisplay::InfoBar),
        }
    }
}

/// Controls how battery information is displayed.
/// - Off: Only show percent and status (e.g., "85% [Charging]").
/// - Bar: Show only a bar.
/// - InfoBar: Show percent, status, and bar (e.g., "85% [Charging] [███]").
/// - BarInfo: Show bar, percent, and status (e.g., "[███] 85% [Charging]"). (default)
#[derive(Debug, Clone, Copy)]
pub enum BatteryDisplayMode {
    Off,
    Bar,
    InfoBar,
    BarInfo,
}

impl FromStr for BatteryDisplayMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "off" => Ok(BatteryDisplayMode::Off),
            "bar" => Ok(BatteryDisplayMode::Bar),
            "infobar" => Ok(BatteryDisplayMode::InfoBar),
            "barinfo" => Ok(BatteryDisplayMode::BarInfo),
            _ => Ok(BatteryDisplayMode::BarInfo),
        }
    }
}
