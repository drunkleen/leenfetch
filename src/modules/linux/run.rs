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
use super::system::distro::{get_distro, DistroDisplay};
use super::system::kernel::get_kernel;
use super::system::model::get_model;
use super::system::os::get_os;
use super::title::get_titles;
use std::fs;
use std::path::PathBuf;

use crate::config::default::{Configs, ConfigsExt};
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

    fn ensure_config_exists() -> bool {
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
            true
        } else {
            false
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
            let shorthand = cfg.get_enum("distro_display", DistroDisplay::NameModelVersionArch);
            Some(get_distro(shorthand))
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
            let shorthand: PackageShorthand =
                cfg.get_enum("package_managers", PackageShorthand::Tiny);
            Some(get_packages(shorthand).unwrap_or("Unknown".into()))
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
            Some(get_wm().unwrap_or("Unknown".into()))
        } else {
            None
        };
        process_single_block(&mut output, "wm", cfg.is_enabled("show_wm"), wm.clone());

        let de = if Self::should_render_tag(layout, cfg, "de", "show_de") {
            Some(get_de(cfg.is_enabled("de_version"), wm.as_deref()).unwrap_or("Unknown".into()))
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
            let mem_unit = cfg.get_enum("memory_unit", MemoryUnit::MiB);
            Some(get_memory(cfg.is_enabled("memory_percent"), mem_unit).unwrap_or("Unknown".into()))
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
        // Battery loop
        // ----------------------------

        let battery_enabled = cfg.is_enabled("show_battery");
        let battery_display = cfg.get_enum("battery_display", BatteryDisplayMode::InfoBar);

        let batteries = if battery_enabled {
            get_battery(battery_display)
        } else {
            vec![]
        };

        if layout.contains("[[battery]]") {
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

pub const DEFAULT_CONFIG: &str = r#"
# LeenFetch config file
# https://github.com/drunkleen/leenfetch

# To use this config, copy it to ~/.config/leenfetch/config.conf
# For more info, see https://github.com/drunkleen/leenfetch



#layout="""
[[titles]]
${bold.c5}{username}${fg.c8}@${bold.c5}{hostname}${fg.c8}
[[/titles]]

{underline}

[[os]]
${bold.c5}OS:${reset} {os_index}
[[/os]]

[[distro]]
${bold.c5}Distro:${reset} {distro_index}
[[/distro]]

[[model]]
${bold.c5}Host:${reset} {model_index}
[[/model]]

[[kernel]]
${bold.c5}Kernel:${reset} {kernel_index}
[[/kernel]]

[[uptime]]
${bold.c5}Uptime:${reset} {uptime_index}
[[/uptime]]

[[packages]]
${bold.c5}Packages:${reset} {packages_index}
[[/packages]]

[[shell]]
${bold.c5}Shell:${reset} {shell_index}
[[/shell]]

[[wm]]
${bold.c5}WM:${reset} {wm_index}
[[/wm]]

[[de]]
${bold.c5}DE:${reset} {de_index}
[[/de]]

[[wm_theme]]
${bold.c5}WM Theme:${reset} {wm_theme_index}
[[/wm_theme]]

[[cpu]]
${bold.c5}CPU:${reset} {cpu_index}
[[/cpu]]

[[gpu]]
${bold.c5}GPU #{gpu_index}:${reset} {gpu}
[[/gpu]]

[[memory]]
${bold.c5}Memory:${reset} {memory_index}
[[/memory]]

[[disk]]
${bold.c5}Disk:${reset} {disk_index}
[[/disk]]

[[resolution]]
${bold.c5}Resolution:${reset} {resolution_index}
[[/resolution]]

[[theme]]
${bold.c5}Theme:${reset} {theme_index}
[[/theme]]

[[battery]]
${bold.c5}Battery${reset} {battery}
[[/battery]]

[[song]]
{song_index}
[[/song]]

{empty_line}

[[colors]]
{colors_index}
[[/colors]]
"""


## 🎨 Colors
## ____________________
## 🖍️ Foreground Colors
## | Token      | Color   |
## | ---------- | ------- |
## | `${fg.c1}` | Black   |
## | `${fg.c2}` | Red     |
## | `${fg.c3}` | Green   |
## | `${fg.c4}` | Yellow  |
## | `${fg.c5}` | Blue    |
## | `${fg.c6}` | Magenta |
## | `${fg.c7}` | Cyan    |
## | `${fg.c8}` | White   |
## _________________________
## 🅱️ Bold Foreground Colors
## | Token        | Color        |
## | ------------ | ------------ |
## | `${bold.c1}` | Bold Black   |
## | `${bold.c2}` | Bold Red     |
## | `${bold.c3}` | Bold Green   |
## | `${bold.c4}` | Bold Yellow  |
## | `${bold.c5}` | Bold Blue    |
## | `${bold.c6}` | Bold Magenta |
## | `${bold.c7}` | Bold Cyan    |
## | `${bold.c8}` | Bold White   |
## ____________________
## 🖼️ Background Colors
## | Token      | Background |
## | ---------- | ---------- |
## | `${bg.c1}` | Black BG   |
## | `${bg.c2}` | Red BG     |
## | `${bg.c3}` | Green BG   |
## | `${bg.c4}` | Yellow BG  |
## | `${bg.c5}` | Blue BG    |
## | `${bg.c6}` | Magenta BG |
## | `${bg.c7}` | Cyan BG    |
## | `${bg.c8}` | White BG   |
## _______________
## 🔁 Reset Colors
## Use `${reset}` to end a color block and return to default.


# Display settings
title_fqdn=on
underline=on
separator=":"

# Output toggles
show_ascii=on
show_titles=on
show_os=on
show_distro=on
show_model=on
show_uptime=on
show_packages=on
show_shell=on
show_wm=on
show_de=on
show_wm_theme=on
show_kernel=on
show_cpu=on
show_gpu=on
show_memory=on
show_song=on
show_resolution=on
show_theme=on
show_disks=on
show_battery=on
show_terminal_colors=on

# Shorthands and flags
distro_display=name_model_arch
uptime_shorthand=on
memory_percent=on
memory_unit=mib
package_managers=tiny
shell_path=on
shell_version=on
cpu_brand=on
cpu_speed=on
cpu_cores=logical
cpu_temp=C
speed_shorthand=on
refresh_rate=on
gpu_type=all
de_version=on
disk_show=/
disk_subtitle=dir
disk_percent=on
disk_display=barinfo
battery_display=barinfo

# Custom paths
ascii_path="#;
