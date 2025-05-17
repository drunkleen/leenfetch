pub const DEFAULT_FLAGS: &str = r#"
(
    ascii_distro: "auto",
    ascii_colors: "distro",
    battery_display: "barinfo",
    color_blocks: "███",
    cpu_brand: true,
    cpu_cores: true,
    cpu_frequency: true,
    cpu_speed: true,
    cpu_temp: 'C',
    cpu_show_temp: true,
    custom_ascii_path: "",
    de_version: true,
    distro_display: "name_model_arch",
    disk_display: "barinfo",
    disk_subtitle: "dir",
    memory_percent: true,
    memory_unit: "mib",
    package_managers: "tiny",
    refresh_rate: true,
    shell_path: true,
    shell_version: true,
    uptime_shorthand: "on",
)
"#;

pub const DEFAULT_TOGGLES: &str = r#"
(
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
)
"#;

pub const DEFAULT_PRINT_LAYOUT: &str = r#"
[
    (label: "Titles", field: "titles"),
    (label: "Distro", field: "distro"),
    (label: "Model", field: "model"),
    (label: "Kernel", field: "kernel"),
    (label: "Uptime", field: "uptime"),
    (label: "Packages", field: "packages"),
    (label: "Shell", field: "shell"),
    (label: "WM", field: "wm"),
    (label: "DE", field: "de"),
    (label: "WM theme", field: "wm_theme"),
    (label: "CPU", field: "cpu"),
    (label: "GPU", field: "gpu"),
    (label: "Memory", field: "memory"),
    (label: "Disk", field: "disk"),
    (label: "Resolution", field: "resolution"),
    (label: "Theme", field: "theme"),
    (label: "Battery", field: "battery"),
    (label: "", field: "song"),
    (label: "", field: "colors"),
]
    
"#;
