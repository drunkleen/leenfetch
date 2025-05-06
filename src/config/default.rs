use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Configs {
    pub enabled: HashMap<String, bool>,
    pub string_values: HashMap<String, String>,
}

impl Configs {
    pub fn should_render_tag(layout: &str, cfg: &Configs, tag: &str, key: &str) -> bool {
        layout.contains(&format!("[[{tag}]]")) && cfg.is_enabled(key)
    }

    pub fn get_enum<T>(&self, key: &str, default: T) -> T
    where
        T: std::str::FromStr,
    {
        self.get(key)
            .and_then(|val| val.parse::<T>().ok())
            .unwrap_or(default)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.string_values.get(key).map(|s| s.as_str())
    }

    pub fn is_enabled(&self, key: &str) -> bool {
        self.enabled.get(key).copied().unwrap_or(false)
    }
}

pub trait ConfigsExt {
    fn load() -> Self;
    fn ensure_config_exists() -> bool;
    fn fill_layout(layout: &str, cfg: &Configs) -> String;
}
