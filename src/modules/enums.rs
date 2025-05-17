use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct SongInfo {
    pub artist: String,
    pub album: String,
    pub title: String,
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UptimeShorthand {
    Full,    // 1 day, 2 hours, 55 minutes
    Tiny,    // 1d 2h 55m
    Seconds, // 123456s
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

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
