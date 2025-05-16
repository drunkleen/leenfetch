use crate::modules::song::SongInfo;

pub struct Data {
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub os: Option<String>,
    pub distro: Option<String>,
    pub model: Option<String>,
    pub kernel: Option<String>,
    pub uptime: Option<String>,
    pub packages: Option<String>,
    pub shell: Option<String>,
    pub wm: Option<String>,
    pub de: Option<String>,
    pub wm_theme: Option<String>,
    pub cpu: Option<String>,
    pub gpu: Option<Vec<String>>,
    pub memory: Option<String>,
    pub disk: Option<Vec<(String, String)>>,
    pub resolution: Option<String>,
    pub theme: Option<String>,
    pub battery: Option<Vec<String>>,
    pub song: Option<SongInfo>,
    pub colors: Option<String>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            username: None,
            hostname: None,
            os: None,
            distro: None,
            model: None,
            kernel: None,
            uptime: None,
            packages: None,
            shell: None,
            wm: None,
            de: None,
            wm_theme: None,
            cpu: None,
            gpu: None,
            memory: None,
            disk: None,
            resolution: None,
            theme: None,
            battery: None,
            song: None,
            colors: None,
        }
    }
}
