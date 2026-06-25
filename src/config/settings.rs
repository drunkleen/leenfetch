use serde::Deserialize;

/// Describes a single entry in the `modules` array.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum LayoutItem {
    Break(String),
    Module(ModuleEntry),
}

/// Configuration flags corresponding to display and formatting options.
#[derive(Debug, Deserialize, Clone)]
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
    #[serde(
        default,
        alias = "cpu_show_temp",
        deserialize_with = "deserialize_cpu_temp"
    )]
    pub cpu_temp: String,
    #[serde(default)]
    pub custom_ascii_path: String,
    #[serde(default)]
    pub de_version: bool,
    #[serde(default)]
    pub disk_display: String,
    #[serde(default)]
    pub disk_percent: bool,
    #[serde(default)]
    pub disk_show: String,
    #[serde(default)]
    pub disk_subtitle: String,
    #[serde(default, alias = "distro_display")]
    pub distro_shorthand: String,
    #[serde(default)]
    pub gpu_brand: bool,
    #[serde(default)]
    pub gpu_type: String,
    #[serde(default)]
    pub kernel_shorthand: bool,
    #[serde(default)]
    pub memory_percent: bool,
    #[serde(default)]
    pub memory_unit: String,
    #[serde(default)]
    pub os_age_shorthand: String,
    #[serde(default)]
    pub package_managers: String,
    #[serde(default)]
    pub shell_path: bool,
    #[serde(default)]
    pub shell_version: bool,
    #[serde(default)]
    pub speed_shorthand: bool,
    #[serde(default)]
    pub uptime_shorthand: String,
}

/// Root configuration structure.
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
    #[serde(default, alias = "layout", alias = "modules")]
    pub layout: Vec<LayoutItem>,
}

/// Minimal representation of the logo block.
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
    pub fn field_name(&self) -> Option<&str> {
        self.module_type
            .as_deref()
            .or(self.field.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    pub fn label(&self) -> Option<&str> {
        self.key.as_deref().or(self.label.as_deref())
    }

    pub fn is_custom(&self) -> bool {
        self.field_name()
            .map(|field| field.eq_ignore_ascii_case("custom"))
            .unwrap_or(false)
    }
}

fn deserialize_cpu_temp<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct CpuTempVisitor;

    impl<'de> serde::de::Visitor<'de> for CpuTempVisitor {
        type Value = String;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("\"C\", \"F\", \"off\", true, or false")
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<String, E> {
            match v.to_lowercase().as_str() {
                "c" | "f" => Ok(v.to_uppercase()),
                "off" => Ok("off".to_string()),
                _ => Err(serde::de::Error::custom(format!("invalid cpu_temp: {v}"))),
            }
        }

        fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<String, E> {
            Ok(if v {
                "C".to_string()
            } else {
                "off".to_string()
            })
        }
    }

    deserializer.deserialize_any(CpuTempVisitor)
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
            cpu_temp: "C".into(),
            de_version: true,
            disk_display: "barinfo".into(),
            disk_percent: true,
            disk_show: "/".into(),
            disk_subtitle: "dir".into(),
            distro_shorthand: "name".into(),
            gpu_brand: true,
            gpu_type: "all".into(),
            kernel_shorthand: true,
            memory_percent: true,
            memory_unit: "mib".into(),
            os_age_shorthand: "full".into(),
            package_managers: "tiny".into(),
            shell_path: true,
            shell_version: true,
            speed_shorthand: false,
            uptime_shorthand: "full".into(),
        }
    }
}
