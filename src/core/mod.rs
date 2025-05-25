mod data;

use std::{collections::HashMap, str::FromStr};

use data::Data;

use crate::{
    config::{self, settings},
    modules::{
        desktop::{
            de::get_de, resolution::get_resolution, theme::get_theme, wm::get_wm,
            wm_theme::get_wm_theme,
        },
        enums::{
            BatteryDisplayMode, DiskDisplay, DiskSubtitle, DistroDisplay, MemoryUnit,
            PackageShorthand, UptimeShorthand,
        },
        info::{
            battery::get_battery, cpu::get_cpu, disk::get_disks, gpu::get_gpus, memory::get_memory,
            uptime::get_uptime,
        },
        packages::get_packages,
        shell::get_shell,
        song::get_song,
        system::{distro::get_distro, kernel::get_kernel, model::get_model, os::get_os},
        title::get_titles,
        utils::{
            colorize_text, get_ascii_and_colors, get_custom_ascii, get_custom_colors_order,
            get_distro_colors, get_terminal_color,
        },
    },
};

pub struct Core {
    output: String,
    toggles: settings::Toggles,
    flags: settings::Flags,
    layout: Vec<settings::Layout>,
    data: Data,
}

impl Core {
    /// Creates a new instance of the `Core` struct.
    ///
    /// This function reads the configuration files and populates the `Core` struct with
    /// the configuration and default system information.
    pub fn new() -> Self {
        let toggles = config::load_toggles();
        let flags = config::load_flags();
        let layout = config::load_print_layout();
        Self {
            output: String::new(),
            toggles,
            flags,
            layout,
            data: Data::default(),
        }
    }

    /// Fills the layout with system information based on the configuration and overrides.
    ///
    /// This function iterates over the configured layout and gathers system information
    /// according to the toggles and flags set. It builds the output string by appending
    /// the formatted information for each section, including user@host titles, OS, distro,
    /// model, kernel, uptime, packages, shell, window manager, desktop environment, CPU, GPU,
    /// memory, disk, resolution, theme, battery, currently playing song, and terminal colors.
    /// The gathered data is stored in the `data` field, and the formatted output is stored in
    /// the `output` string. Additionally, the function handles custom ASCII art and colors,
    /// applying any overrides provided in the `override_map`.
    ///
    /// # Arguments
    ///
    /// * `override_map` - A `HashMap` that allows overriding certain configuration values
    ///   such as custom ASCII path, ASCII colors, and ASCII distro.
    ///
    /// # Returns
    ///
    /// A tuple containing two `String` values:
    /// * The first `String` is the colorized system information output.
    /// * The second `String` is the colorized ASCII art.

    pub fn fill_layout(&mut self, override_map: HashMap<&'static str, String>) -> (String, String) {
        for info in &self.layout {
            match info.field.as_str() {
                "titles" => {
                    if !self.toggles.show_titles {
                        continue;
                    }
                    let (user, host, dash_len) = get_titles(true);
                    self.output.push_str(
                        format!(
                            "${{c1}}{}${{reset}}@${{c1}}{}${{reset}}\n{}\n",
                            user,
                            host,
                            "-".repeat(dash_len)
                        )
                        .as_str(),
                    );
                    self.data.username = Some(user);
                    self.data.hostname = Some(host);
                }
                "os" => {
                    if !self.toggles.show_os {
                        continue;
                    }
                    let os = get_os();
                    self.output
                        .push_str(format!("${{c1}}{}: ${{reset}}{}\n", info.label, os).as_str());
                    self.data.os = Some(os);
                }
                "distro" => {
                    if !self.toggles.show_distro {
                        continue;
                    }
                    let distro = get_distro(
                        DistroDisplay::from_str(&self.flags.distro_display)
                            .unwrap_or(DistroDisplay::NameModelVersionArch),
                    );
                    self.output.push_str(
                        format!("${{c1}}{}: ${{reset}}{}\n", info.label, distro).as_str(),
                    );
                    self.data.distro = Some(distro);
                }
                "model" => {
                    if !self.toggles.show_model {
                        continue;
                    }
                    let model = get_model();
                    Self::is_some_add_to_output(&info.label, &model, &mut self.output);
                    self.data.model = model;
                }
                "kernel" => {
                    if !self.toggles.show_kernel {
                        continue;
                    }
                    let kernel = get_kernel();
                    Self::is_some_add_to_output(&info.label, &kernel, &mut self.output);
                    self.data.kernel = kernel;
                }
                "uptime" => {
                    if !self.toggles.show_uptime {
                        continue;
                    }
                    let uptime = get_uptime(
                        UptimeShorthand::from_str(&self.flags.uptime_shorthand)
                            .unwrap_or(UptimeShorthand::Full),
                    );
                    Self::is_some_add_to_output(&info.label, &uptime, &mut self.output);
                    self.data.uptime = uptime;
                }
                "packages" => {
                    if !self.toggles.show_packages {
                        continue;
                    }
                    let packages = get_packages(
                        PackageShorthand::from_str(&self.flags.package_managers)
                            .unwrap_or(PackageShorthand::On),
                    );
                    Self::is_some_add_to_output(&info.label, &packages, &mut self.output);
                    self.data.packages = packages;
                }
                "shell" => {
                    if !self.toggles.show_shell {
                        continue;
                    }
                    let shell = get_shell(self.flags.shell_path, self.flags.shell_version);
                    Self::is_some_add_to_output(&info.label, &shell, &mut self.output);
                    self.data.shell = shell;
                }
                "wm" => {
                    if !self.toggles.show_wm {
                        continue;
                    }
                    if self.data.wm.is_none() {
                        self.data.wm = get_wm();
                    }

                    Self::is_some_add_to_output(&info.label, &self.data.wm, &mut self.output);
                }
                "de" => {
                    if !self.toggles.show_de {
                        continue;
                    }
                    if self.data.wm.is_none() {
                        self.data.wm = get_wm();
                    }
                    let de = get_de(self.flags.de_version, self.data.wm.as_deref());
                    Self::is_some_add_to_output(&info.label, &de, &mut self.output);
                    self.data.de = de;
                }
                "wm_theme" => {
                    if !self.toggles.show_wm_theme {
                        continue;
                    }
                    if self.data.wm.is_none() {
                        self.data.wm = get_wm();
                    }
                    if self.data.de.is_none() {
                        self.data.de = get_de(self.toggles.show_wm, self.data.wm.as_deref());
                    }

                    let wm = self.data.wm.as_deref().unwrap_or("");
                    let de = self.data.de.as_deref();

                    let wm_theme = get_wm_theme(wm, de);
                    Self::is_some_add_to_output(&info.label, &wm_theme, &mut self.output);
                    self.data.wm_theme = wm_theme;
                }
                "cpu" => {
                    if !self.toggles.show_cpu {
                        continue;
                    }
                    let cpu = get_cpu(
                        self.flags.cpu_brand,
                        self.flags.cpu_frequency,
                        self.flags.cpu_cores,
                        self.flags.cpu_show_temp,
                        self.flags.cpu_speed,
                        match self.flags.cpu_temp {
                            'f' | 'F' => Some('F'),
                            _ => Some('C'),
                        },
                    );
                    Self::is_some_add_to_output(&info.label, &cpu, &mut self.output);
                    self.data.cpu = cpu;
                }
                "gpu" => {
                    if !self.toggles.show_gpu {
                        continue;
                    }
                    let gpus = get_gpus();
                    if gpus.is_empty() {
                        self.output.push_str(
                            format!("${{c1}}{}: ${{reset}}{}\n", &info.label, "No GPU found")
                                .as_str(),
                        );
                        self.data.gpu = None;
                    }
                    if gpus.len() == 1 {
                        self.output.push_str(
                            format!("${{c1}}{}: ${{reset}}{}\n", &info.label, gpus[0]).as_str(),
                        );
                    } else {
                        for (index, gpu) in gpus.iter().enumerate() {
                            self.output.push_str(
                                format!("${{c1}}{} #{}: ${{reset}}{}\n", &info.label, index, gpu)
                                    .as_str(),
                            );
                        }
                    }
                    self.data.gpu = Some(gpus);
                }
                "memory" => {
                    if !self.toggles.show_memory {
                        continue;
                    }
                    let memory = get_memory(
                        self.flags.memory_percent,
                        MemoryUnit::from_str(self.flags.memory_unit.as_str())
                            .unwrap_or(MemoryUnit::MiB),
                    );
                    Self::is_some_add_to_output(&info.label, &memory, &mut self.output);
                    self.data.memory = memory;
                }
                "disk" => {
                    if self.toggles.show_disks {
                        if let Some(disks) = get_disks(
                            DiskSubtitle::from_str(self.flags.disk_subtitle.as_str())
                                .unwrap_or(DiskSubtitle::Dir),
                            DiskDisplay::from_str(self.flags.disk_display.as_str())
                                .unwrap_or(DiskDisplay::InfoBar),
                            None,
                        ) {
                            for disk in &disks {
                                self.output.push_str(
                                    format!(
                                        "${{c1}}{} {}: ${{reset}}{}\n",
                                        &info.label, disk.0, disk.1
                                    )
                                    .as_str(),
                                );
                            }
                            self.data.disk = Some(disks);
                        } else {
                            self.output.push_str(
                                format!("${{c1}}{}: ${{reset}}{}\n", &info.label, "No disks found")
                                    .as_str(),
                            );
                            self.data.disk = None;
                        }
                    }
                }
                "resolution" => {
                    if self.toggles.show_resolution {
                        let resolution = get_resolution(self.flags.refresh_rate);
                        Self::is_some_add_to_output(&info.label, &resolution, &mut self.output);
                        self.data.resolution = resolution;
                    }
                }
                "theme" => {
                    if !self.toggles.show_theme {
                        continue;
                    }
                    if self.data.de.is_none() {
                        self.data.de = get_de(self.toggles.show_wm, self.data.wm.as_deref());
                    }

                    let theme = get_theme(self.data.de.as_deref());
                    Self::is_some_add_to_output(&info.label, &theme, &mut self.output);
                    self.data.theme = theme;
                }
                "battery" => {
                    if !self.toggles.show_battery {
                        continue;
                    }

                    let battery_display_mode =
                        BatteryDisplayMode::from_str(&self.flags.battery_display);
                    let batteries =
                        get_battery(battery_display_mode.unwrap_or(BatteryDisplayMode::BarInfo));

                    if batteries.is_empty() {
                        self.output.push_str(
                            format!("${{c1}}{}: ${{reset}}{}\n", &info.label, "No Battery found")
                                .as_str(),
                        );
                    }

                    if batteries.len() == 1 {
                        self.output.push_str(
                            format!("${{c1}}{}: ${{reset}}{}\n", &info.label, batteries[0])
                                .as_str(),
                        );
                    } else {
                        for (index, battery) in batteries.iter().enumerate() {
                            self.output.push_str(
                                format!(
                                    "${{c1}}{} {}: ${{reset}}{}\n",
                                    &info.label, index, battery
                                )
                                .as_str(),
                            );
                        }
                    }
                    self.data.battery = Some(batteries);
                }
                "song" => {
                    if !self.toggles.show_song {
                        continue;
                    }
                    let song = get_song();
                    if !song.is_none() {
                        continue;
                    }
                    self.data.song = song.clone();

                    if let Some(music) = song {
                        self.data.song = Some(music.clone());

                        self.output.push_str(
                            format!(
                                "${{c1}}Playing${{reset}}\n    {}\n    {}\n",
                                music.title, music.artist
                            )
                            .as_str(),
                        );
                    }
                }
                "colors" => {
                    if !self.toggles.show_terminal_colors {
                        continue;
                    }
                    let color_blocks = if self.flags.color_blocks != "" {
                        self.flags.color_blocks.as_str()
                    } else {
                        "███"
                    };

                    let colors = get_terminal_color(color_blocks).join("\n");
                    self.output.push_str(format!("\n{}", colors).as_str());
                    self.data.colors = Some(colors);
                }
                other => {
                    self.output.push_str(
                        format!("${{c1}}{}: ${{reset}}{}\n", &info.label, other).as_str(),
                    );
                }
            }
        }

        let mut custom_ascii_path = override_map.get("custom_ascii_path").map(String::as_str);
        if custom_ascii_path == Some("") {
            custom_ascii_path = Some(self.flags.custom_ascii_path.as_str());
        }

        let custom_ascii_colors = override_map
            .get("ascii_colors")
            .map(String::as_str)
            .or(Some(self.flags.ascii_colors.as_str()));

        let ascii_distro = override_map
            .get("ascii_distro")
            .map(String::as_str)
            .filter(|s| !s.is_empty())
            .or_else(|| {
                let val = self.flags.ascii_distro.as_str();
                (!val.is_empty()).then_some(val)
            })
            .or(Some("distro"));

        let resolved_distro = match ascii_distro.unwrap_or("auto") {
            "auto" => get_distro(DistroDisplay::Name),
            "auto_small" => format!("{}_small", get_distro(DistroDisplay::Name)),
            other => other.to_string(),
        };

        // Load ASCII Art
        let raw_ascii_art = custom_ascii_path
            .map(get_custom_ascii)
            .unwrap_or_else(|| get_ascii_and_colors(&resolved_distro));

        // Load Colors
        let distro_colors = if &resolved_distro == "off" {
            get_distro_colors(&get_distro(DistroDisplay::Name))
        } else {
            match custom_ascii_colors {
                Some("distro") => get_distro_colors(&resolved_distro),
                Some(other) => get_custom_colors_order(other),
                None => get_distro_colors(&resolved_distro),
            }
        };

        let info_colored = colorize_text(self.output.clone(), &distro_colors);
        let ascii_colored = colorize_text(raw_ascii_art, &distro_colors);
        (info_colored, ascii_colored)
    }

    /// If the given `data` is `Some`, it will be added to `output` with the given `label`.
    /// If `data` is `None`, it will add a line to `output` with the label and the value "Unknown".
    fn is_some_add_to_output(label: &str, data: &Option<String>, output: &mut String) {
        match data {
            Some(d) => {
                output.push_str(format!("${{c1}}{}: ${{reset}}{}\n", label, d).as_str());
            }
            None => {
                output.push_str(format!("${{c1}}{}: ${{reset}}{}\n", label, "Unknown").as_str());
            }
        }
    }
}
