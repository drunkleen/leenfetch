use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::modules::{
    config::CONFIG_PATH,
    desktop::{
        de::get_de, resolution::get_resolution, theme::get_theme, wm::get_wm,
        wm_theme::get_wm_theme,
    },
    info::{
        battery::{get_battery, BatteryDisplayMode},
        cpu::get_cpu,
        disk::{get_disks, DiskDisplay, DiskSubtitle},
        gpu::get_gpus,
        memory::{get_memory, MemoryUnit},
        uptime::{get_uptime, UptimeShorthand},
    },
    packages::{get_packages, PackageShorthand},
    shell::get_shell,
    song::get_song,
    system::{
        distro::{get_distro, DistroDisplay},
        kernel::get_kernel,
        model::get_model,
        os::get_os,
    },
    title::get_titles,
    utils::{
        colorize_text, get_ascii_and_colors, get_custom_ascii, get_custom_colors_order,
        get_distro_colors, get_terminal_color, process_loop_block, process_single_block,
    },
};

#[derive(Debug, Default)]
pub struct Run {
    pub enabled: HashMap<String, bool>,
    pub string_values: HashMap<String, String>,
}

impl Run {
    pub fn load() -> Self {
        let mut config = Run::default();
        let path = dirs::home_dir()
            .map(|p| p.join(CONFIG_PATH))
            .unwrap_or(PathBuf::from("/dev/null"));

        if let Ok(contents) = fs::read_to_string(path) {
            if let Some(layout_block) = contents.split("layout=").nth(1).and_then(|s| {
                if s.starts_with("(") {
                    s[1..].split(")").next()
                } else {
                    None
                }
            }) {
                config
                    .string_values
                    .insert("layout".into(), layout_block.trim().to_string());
            }

            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim().to_string();
                    let val = value.trim().trim_matches('"');

                    match val {
                        "on" => {
                            config.enabled.insert(key.clone(), true);
                            config.string_values.insert(key, val.to_string());
                        }
                        "off" => {
                            config.enabled.insert(key.clone(), false);
                            config.string_values.insert(key, val.to_string());
                        }
                        _ => {
                            config.string_values.insert(key, val.to_string());
                        }
                    }
                }
            }
        }

        config
    }

    pub fn ensure_config_exists() -> bool {
        let path = dirs::home_dir()
            .map(|p| p.join(CONFIG_PATH))
            .expect("Could not determine home directory");

        if !path.exists() {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).expect("Failed to create config directory");
                }
            }

            std::fs::write(&path, crate::modules::config::DEFAULT_CONFIG)
                .expect("Failed to write default config file");

            println!("✅ Created default config at {}", path.display());
            true
        } else {
            false
        }
    }

    pub fn fill_layout(
        layout: &str,
        run: &Run,
        override_map: HashMap<&'static str, String>,
    ) -> (String, String) {
        let mut output = layout.to_string();

        // ----------------------------
        // Titles (always used for {underline})
        // ----------------------------

        if Self::should_render_tag(layout, run, "titles", "show_titles") {
            let (username, hostname, title_len) = get_titles(run.is_enabled("show_titles"));

            let process_titles_block =
                |output: &mut String, username: &str, hostname: &str, enabled: bool| {
                    let start_tag = "[titles]";
                    let end_tag = "[/titles]";

                    if let Some(start_idx) = output.find(start_tag) {
                        if let Some(end_idx) = output[start_idx + start_tag.len()..].find(end_tag) {
                            let block_start = start_idx + start_tag.len();
                            let block_end = block_start + end_idx;
                            let full_end = block_end + end_tag.len();

                            let block = &output[block_start..block_end];
                            let rendered = if enabled {
                                block
                                    .replace("{username}", username)
                                    .replace("{hostname}", hostname)
                                    .trim()
                                    .to_string()
                            } else {
                                String::new()
                            };

                            output.replace_range(start_idx..full_end, &rendered);
                        }
                    }
                };

            output = output.replace("{underline}", &"-".repeat(title_len));

            process_titles_block(
                &mut output,
                &username,
                &hostname,
                run.is_enabled("show_titles"),
            );
        }

        // ----------------------------
        // Block replacements
        // ----------------------------
        let os = if Self::should_render_tag(layout, run, "os", "show_os") {
            Some(get_os())
        } else {
            None
        };
        process_single_block(&mut output, "os", run.is_enabled("show_os"), os);

        let distro = if Self::should_render_tag(layout, run, "distro", "show_distro") {
            let shorthand = run.get_enum("distro_display", DistroDisplay::NameModelVersionArch);
            Some(get_distro(shorthand))
        } else {
            None
        };
        process_single_block(
            &mut output,
            "distro",
            run.is_enabled("show_distro"),
            distro.clone(),
        );

        let model = if Self::should_render_tag(layout, run, "model", "show_model") {
            Some(get_model().unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "model", run.is_enabled("show_model"), model);

        let kernel = if Self::should_render_tag(layout, run, "kernel", "show_kernel") {
            Some(get_kernel().unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "kernel", run.is_enabled("show_kernel"), kernel);

        let uptime = if Self::should_render_tag(layout, run, "uptime", "show_uptime") {
            let short = run.get_enum("uptime_shorthand", UptimeShorthand::Full);
            Some(get_uptime(short).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "uptime", run.is_enabled("show_uptime"), uptime);

        let packages = if Self::should_render_tag(layout, run, "packages", "show_packages") {
            let shorthand: PackageShorthand =
                run.get_enum("package_managers", PackageShorthand::Tiny);
            Some(get_packages(shorthand).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(
            &mut output,
            "packages",
            run.is_enabled("show_packages"),
            packages,
        );

        let shell = if Self::should_render_tag(layout, run, "shell", "show_shell") {
            Some(
                get_shell(
                    run.is_enabled("shell_path"),
                    run.is_enabled("shell_version"),
                )
                .unwrap_or("Unknown".into()),
            )
        } else {
            None
        };
        process_single_block(&mut output, "shell", run.is_enabled("show_shell"), shell);

        let wm = if Self::should_render_tag(layout, run, "wm", "show_wm") {
            Some(get_wm().unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "wm", run.is_enabled("show_wm"), wm.clone());

        let de = if Self::should_render_tag(layout, run, "de", "show_de") {
            Some(get_de(run.is_enabled("de_version"), wm.as_deref()).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "de", run.is_enabled("show_de"), de.clone());

        let wm_theme = if Self::should_render_tag(layout, run, "wm_theme", "show_wm_theme") {
            Some(
                get_wm_theme(wm.as_deref().unwrap_or(""), de.as_deref())
                    .unwrap_or("Unknown".into()),
            )
        } else {
            None
        };
        process_single_block(
            &mut output,
            "wm_theme",
            run.is_enabled("show_wm_theme"),
            wm_theme,
        );

        let theme = if Self::should_render_tag(layout, run, "theme", "show_theme") {
            Some(get_theme(de.as_deref()).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "theme", run.is_enabled("show_theme"), theme);

        let cpu = if Self::should_render_tag(layout, run, "cpu", "show_cpu") {
            Some(
                get_cpu(
                    run.is_enabled("cpu_brand"),
                    run.is_enabled("cpu_speed"),
                    run.get_from_cfg("cpu_cores") != Some("off"),
                    run.get_from_cfg("cpu_temp").unwrap_or("off") != "off",
                    run.is_enabled("speed_shorthand"),
                    run.get_from_cfg("cpu_temp")
                        .map(|s| s.chars().next().unwrap_or('C')),
                )
                .unwrap_or("Unknown".into()),
            )
        } else {
            None
        };
        process_single_block(&mut output, "cpu", run.is_enabled("show_cpu"), cpu);

        let memory = if Self::should_render_tag(layout, run, "memory", "show_memory") {
            let mem_unit = run.get_enum("memory_unit", MemoryUnit::MiB);
            Some(get_memory(run.is_enabled("memory_percent"), mem_unit).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "memory", run.is_enabled("show_memory"), memory);

        let resolution = if Self::should_render_tag(layout, run, "resolution", "show_resolution") {
            Some(get_resolution(run.is_enabled("refresh_rate")).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(
            &mut output,
            "resolution",
            run.is_enabled("show_resolution"),
            resolution,
        );

        let song = if Self::should_render_tag(layout, run, "song", "show_song") {
            Some(
                get_song()
                    .map(|s| {
                        format!(
                            "🎵 Playing:\n    {}\n    {}\n    {}",
                            s.title, s.artist, s.album
                        )
                    })
                    .unwrap_or_default(),
            )
        } else {
            None
        };
        process_single_block(&mut output, "song", run.is_enabled("show_song"), song);

        let colors = if Self::should_render_tag(layout, run, "colors", "show_terminal_colors") {
            let color_blocks = run.get_from_cfg("color_blocks").unwrap_or("███");

            Some(get_terminal_color(color_blocks).join("\n"))
        } else {
            None
        };
        process_single_block(
            &mut output,
            "colors",
            run.is_enabled("show_terminal_colors"),
            colors,
        );

        // ----------------------------
        // Battery loop
        // ----------------------------

        let battery_enabled = run.is_enabled("show_battery");
        let battery_display = run.get_enum("battery_display", BatteryDisplayMode::InfoBar);

        let batteries = if battery_enabled {
            get_battery(battery_display)
        } else {
            vec![]
        };

        if layout.contains("[battery]") {
            process_loop_block(
                &mut output,
                "battery",
                &batteries,
                battery_enabled,
                |block, bat| {
                    let index = batteries.iter().position(|b| b == bat).unwrap_or(0) + 1;
                    block
                        .replace("{battery}", bat)
                        .replace("{battery_index}", &index.to_string())
                },
            );
        }

        // ----------------------------
        // GPU loop
        // ----------------------------
        let gpu_enabled = run.is_enabled("show_gpu");

        let gpus = if gpu_enabled { get_gpus() } else { vec![] };

        if layout.contains("[gpu]") {
            process_loop_block(&mut output, "gpu", &gpus, gpu_enabled, |block, gpu| {
                let index = gpus.iter().position(|g| g == gpu).unwrap_or(0) + 1;
                block
                    .replace("{gpu}", gpu)
                    .replace("{gpu_index}", &index.to_string())
            });
        }

        // ----------------------------
        // Disk loop
        // ----------------------------
        let disk_enabled = run.is_enabled("show_disks");

        if layout.contains("[disk]") {
            let mode = match run.get_from_cfg("disk_display") {
                Some("bar") => DiskDisplay::Bar,
                Some("infobar") => DiskDisplay::InfoBar,
                Some("barinfo") => DiskDisplay::BarInfo,
                Some("percentage") => DiskDisplay::Percentage,
                _ => DiskDisplay::Info,
            };

            let subtitle = match run.get_from_cfg("disk_subtitle") {
                Some("name") => DiskSubtitle::Name,
                Some("mount") => DiskSubtitle::Mount,
                Some("none") => DiskSubtitle::None,
                _ => DiskSubtitle::Dir,
            };

            let paths = run.get_from_cfg("disk_show").map(|s| vec![s]);

            let disks = if disk_enabled {
                get_disks(subtitle, mode, paths).unwrap_or_default()
            } else {
                vec![]
            };

            process_loop_block(
                &mut output,
                "disk",
                &disks,
                disk_enabled,
                |block, (label, value)| {
                    block
                        .replace("{disk_label}", label)
                        .replace("{disk_index}", value)
                },
            );
        }

        output = output
            .lines()
            .map(str::trim_end)
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        // ----------------------------
        // ascii art
        // ----------------------------

        // let ascii_art_size = override_map
        //     .get("ascii_size")
        //     .map(|s| s.to_string())
        //     .unwrap_or_else(|| run.get_enum("ascii_size", AsciiSize::Large).to_string());

        // ---------------------------------------------------------------------------------------------------
        // ---------------------------------------------------------------------------------------------------

        let custom_ascii_path = override_map
            .get("custom_ascii_path")
            .map(String::as_str)
            .or_else(|| {
                run.get_from_cfg("custom_ascii_path")
                    .filter(|s| !s.trim().is_empty())
            });

        let custom_ascii_colors = override_map
            .get("ascii_colors")
            .map(String::as_str)
            .or_else(|| {
                run.get_from_cfg("ascii_colors")
                    .filter(|s| !s.trim().is_empty())
            });

        // Resolves "ascii_distro", defaults to "auto"
        let ascii_distro = override_map
            .get("ascii_distro")
            .cloned()
            .or_else(|| run.get_from_cfg("ascii_distro").map(|s| s.to_string()))
            .unwrap_or_else(|| "auto".to_string());

        let resolved_distro = match ascii_distro.as_str() {
            "auto" => get_distro(DistroDisplay::Name),
            "auto_small" => format!("{}_small", get_distro(DistroDisplay::Name)),
            other => other.to_string(),
        };

        // Load ASCII Art
        let raw_ascii_art = custom_ascii_path
            .map(get_custom_ascii)
            .unwrap_or_else(|| get_ascii_and_colors(&resolved_distro));

        // Load Colors
        let distro_colors = match custom_ascii_colors {
            Some("distro") => get_distro_colors(&resolved_distro),
            Some(other) => get_custom_colors_order(other),
            None => get_distro_colors(&resolved_distro),
        };

        // ---------------------------------------------------------------------------------------------------
        // ---------------------------------------------------------------------------------------------------

        // let distro_colors = get_distro_colors_order(&distro.unwrap_or("unknown".into()));

        output = output.replace("{empty_line}", "");
        output = colorize_text(&mut output, &distro_colors);

        let info_colored = colorize_text(&mut output, &distro_colors);
        let ascii_colored = colorize_text(&raw_ascii_art, &distro_colors);
        (info_colored, ascii_colored)
    }

    pub fn should_render_tag(layout: &str, cfg: &Run, tag: &str, key: &str) -> bool {
        layout.contains(&format!("[{tag}]")) && cfg.is_enabled(key)
    }

    pub fn get_enum<T>(&self, key: &str, default: T) -> T
    where
        T: std::str::FromStr,
    {
        self.get_from_cfg(key)
            .and_then(|val| val.parse::<T>().ok())
            .unwrap_or(default)
    }

    pub fn get_from_cfg(&self, key: &str) -> Option<&str> {
        self.string_values.get(key).map(|s| s.as_str())
    }

    pub fn is_enabled(&self, key: &str) -> bool {
        self.enabled.get(key).copied().unwrap_or(false)
    }
}
