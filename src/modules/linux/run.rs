use super::desktop::de::get_de;
use super::desktop::resolution::get_resolution;
use super::desktop::theme::get_theme;
use super::desktop::wm::get_wm;
use super::desktop::wm_theme::get_wm_theme;
use super::info::battery::{get_battery, BatteryDisplayMode};
use super::info::cpu::get_cpu;
use super::info::disk::{get_disks, DiskDisplay, DiskSubtitle};
use super::info::gpu::get_gpus;
use super::info::memory::{get_memory, MemoryUnit};
use super::info::uptime::{get_uptime, UptimeShorthand};
use super::packages::{get_packages, PackageShorthand};
use super::shell::get_shell;
use super::song::get_song;
use super::system::distro::{get_distro, DistroShorthand};
use super::system::kernel::get_kernel;
use super::system::model::get_model;
use super::system::os::get_os;
use super::title::get_titles;
use std::fs;
use std::path::PathBuf;

use crate::config::default::{Configs, ConfigsExt, DEFAULT_CONFIG};
use crate::modules::utils::{
    colorize_text, get_terminal_color, process_loop_block, process_single_block,
};

impl ConfigsExt for Configs {
    fn load() -> Self {
        let mut config = Configs::default();
        let path = dirs::home_dir()
            .map(|p| p.join(".config/leenfetch/config.conf"))
            .unwrap_or(PathBuf::from("/dev/null"));

        if let Ok(contents) = fs::read_to_string(path) {
            if let Some(layout_block) = contents.split("layout=").nth(1).and_then(|s| {
                if s.starts_with("\"\"\"") {
                    s[3..].split("\"\"\"").next()
                } else if s.starts_with('"') {
                    s[1..].split('"').next()
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

                    // Store booleans and strings separately
                    match val {
                        "on" => {
                            config.enabled.insert(key, true);
                        }
                        "off" => {
                            config.enabled.insert(key, false);
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

    fn ensure_config_exists() {
        let path = dirs::home_dir()
            .map(|p| p.join(".config/leenfetch/config.conf"))
            .expect("Could not determine home directory");

        if !path.exists() {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).expect("Failed to create config directory");
                }
            }

            std::fs::write(&path, crate::modules::run::DEFAULT_CONFIG)
                .expect("Failed to write default config file");

            println!("✅ Created default config at {}", path.display());
        }
    }

    fn fill_layout(layout: &str, cfg: &Configs) -> String {
        let mut output = layout.to_string();

        // ----------------------------
        // Titles (always used for {underline})
        // ----------------------------

        if Self::should_render_tag(layout, cfg, "titles", "show_titles") {
            let (username, hostname, title_len) = get_titles(cfg.is_enabled("show_titles"));

            let process_titles_block =
                |output: &mut String, username: &str, hostname: &str, enabled: bool| {
                    let start_tag = "[[titles]]";
                    let end_tag = "[[/titles]]";

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
                cfg.is_enabled("show_titles"),
            );
        }

        // ----------------------------
        // Block replacements
        // ----------------------------
        let os = if Self::should_render_tag(layout, cfg, "os", "show_os") {
            Some(get_os().unwrap_or("Unknown".into()))
        } else {
            None
        };

        process_single_block(&mut output, "os", cfg.is_enabled("show_os"), os);

        let distro = if Self::should_render_tag(layout, cfg, "distro", "show_distro") {
            let shorthand = cfg.get_enum("distro_shorthand", DistroShorthand::Tiny);
            Some(get_distro(shorthand).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "distro", cfg.is_enabled("show_distro"), distro);

        let model = if Self::should_render_tag(layout, cfg, "model", "show_model") {
            Some(get_model().unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "model", cfg.is_enabled("show_model"), model);

        let kernel = if Self::should_render_tag(layout, cfg, "kernel", "show_kernel") {
            Some(get_kernel().unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "kernel", cfg.is_enabled("show_kernel"), kernel);

        let uptime = if Self::should_render_tag(layout, cfg, "uptime", "show_uptime") {
            let short = cfg.get_enum("uptime_shorthand", UptimeShorthand::Full);
            Some(get_uptime(short).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "uptime", cfg.is_enabled("show_uptime"), uptime);

        let packages = if Self::should_render_tag(layout, cfg, "packages", "show_packages") {
            Some(get_packages(PackageShorthand::Tiny).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(
            &mut output,
            "packages",
            cfg.is_enabled("show_packages"),
            packages,
        );

        let shell = if Self::should_render_tag(layout, cfg, "shell", "show_shell") {
            Some(
                get_shell(
                    cfg.is_enabled("shell_path"),
                    cfg.is_enabled("shell_version"),
                )
                .unwrap_or("Unknown".into()),
            )
        } else {
            None
        };
        process_single_block(&mut output, "shell", cfg.is_enabled("show_shell"), shell);

        let wm = if Self::should_render_tag(layout, cfg, "wm", "show_wm") {
            Some(get_wm("Linux", "Linux").unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "wm", cfg.is_enabled("show_wm"), wm.clone());

        let de = if Self::should_render_tag(layout, cfg, "de", "show_de") {
            Some(get_de("Linux", "Linux", wm.as_deref(), true).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "de", cfg.is_enabled("show_de"), de.clone());

        let wm_theme = if Self::should_render_tag(layout, cfg, "wm_theme", "show_wm_theme") {
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
            cfg.is_enabled("show_wm_theme"),
            wm_theme,
        );

        let theme = if Self::should_render_tag(layout, cfg, "theme", "show_theme") {
            Some(get_theme(de.as_deref()).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "theme", cfg.is_enabled("show_theme"), theme);

        let cpu = if Self::should_render_tag(layout, cfg, "cpu", "show_cpu") {
            Some(
                get_cpu(
                    cfg.is_enabled("cpu_brand"),
                    cfg.is_enabled("cpu_speed"),
                    cfg.get("cpu_cores") != Some("off"),
                    cfg.get("cpu_temp").unwrap_or("off") != "off",
                    cfg.is_enabled("speed_shorthand"),
                    cfg.get("cpu_temp").map(|s| s.chars().next().unwrap_or('C')),
                )
                .unwrap_or("Unknown".into()),
            )
        } else {
            None
        };
        process_single_block(&mut output, "cpu", cfg.is_enabled("show_cpu"), cpu);

        let memory = if Self::should_render_tag(layout, cfg, "memory", "show_memory") {
            Some(
                get_memory(cfg.is_enabled("memory_percent"), MemoryUnit::MiB)
                    .unwrap_or("Unknown".into()),
            )
        } else {
            None
        };
        process_single_block(&mut output, "memory", cfg.is_enabled("show_memory"), memory);

        let resolution = if Self::should_render_tag(layout, cfg, "resolution", "show_resolution") {
            Some(get_resolution(cfg.is_enabled("refresh_rate")).unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(
            &mut output,
            "resolution",
            cfg.is_enabled("show_resolution"),
            resolution,
        );

        let battery = if Self::should_render_tag(layout, cfg, "battery", "show_battery") {
            Some(
                get_battery(BatteryDisplayMode::BarInfo)
                    .map(|lines| lines.join("\n"))
                    .unwrap_or_default(),
            )
        } else {
            None
        };
        process_single_block(
            &mut output,
            "battery",
            cfg.is_enabled("show_battery"),
            battery,
        );

        let song = if Self::should_render_tag(layout, cfg, "song", "show_song") {
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
        process_single_block(&mut output, "song", cfg.is_enabled("show_song"), song);

        let colors = if Self::should_render_tag(layout, cfg, "colors", "show_terminal_colors") {
            Some(get_terminal_color().join("\n"))
        } else {
            None
        };
        process_single_block(
            &mut output,
            "colors",
            cfg.is_enabled("show_terminal_colors"),
            colors,
        );

        // ----------------------------
        // GPU loop
        // ----------------------------
        let gpu_enabled = cfg.is_enabled("show_gpu");

        let gpus = if gpu_enabled { get_gpus() } else { vec![] };

        if layout.contains("[[gpu]]") {
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
        let disk_enabled = cfg.is_enabled("show_disks");

        if layout.contains("[[disk]]") {
            let mode = match cfg.get("disk_display") {
                Some("bar") => DiskDisplay::Bar,
                Some("infobar") => DiskDisplay::InfoBar,
                Some("barinfo") => DiskDisplay::BarInfo,
                Some("percentage") => DiskDisplay::Percentage,
                _ => DiskDisplay::Info,
            };

            let subtitle = match cfg.get("disk_subtitle") {
                Some("name") => DiskSubtitle::Name,
                Some("mount") => DiskSubtitle::Mount,
                Some("none") => DiskSubtitle::None,
                _ => DiskSubtitle::Dir,
            };

            let paths = cfg.get("disk_show").map(|s| vec![s]);

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

        output = output.replace("{empty_line}", "\n");
        colorize_text(&mut output)
    }
}
