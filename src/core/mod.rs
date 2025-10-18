mod data;

use std::{collections::HashMap, str::FromStr};

use data::Data;

use crate::{
    config::{self, settings},
    modules::{
        desktop::{de::get_de, resolution::get_resolution, theme::get_theme, wm::get_wm},
        enums::{
            BatteryDisplayMode, DiskDisplay, DiskSubtitle, DistroDisplay, MemoryUnit,
            OsAgeShorthand, PackageShorthand, UptimeShorthand,
        },
        info::{
            battery::get_battery, cpu::get_cpu, disk::get_disks, gpu::get_gpus, memory::get_memory,
            os_age::get_os_age, uptime::get_uptime,
        },
        packages::get_packages,
        shell::get_shell,
        song::get_song,
        system::{distro::get_distro, kernel::get_kernel, model::get_model, os::get_os},
        title::get_titles,
        utils::{
            get_ascii_and_colors, get_custom_ascii, get_custom_colors_order, get_distro_colors,
            get_terminal_color,
        },
    },
};

pub struct Core {
    output: String,
    flags: settings::Flags,
    layout: Vec<settings::LayoutItem>,
    data: Data,
}

impl Core {
    /// Creates a new instance of the `Core` struct.
    ///
    /// This function reads the configuration file and populates the `Core` struct with
    /// the configuration and default system information.
    pub fn new() -> Self {
        let flags = config::load_flags();
        let layout = config::load_print_layout();
        Self {
            output: String::new(),
            flags,
            layout,
            data: Data::default(),
        }
    }

    /// Builds the final colorized layout output using the loaded configuration.
    ///
    /// Each entry in the layout is resolved against the configured flags. System information
    /// is collected on demand, cached in `self.data`, and appended to the final string with
    /// the requested labels. Custom rows are written verbatim and `"break"` entries insert
    /// blank lines to keep section separators intact. ASCII art is handled separately when
    /// rendering the banner.
    ///
    /// # Returns
    ///
    /// A single colorized `String` containing the assembled module output.
    pub fn get_info_layout(&mut self) -> String {
        let mut final_output = String::new();

        for item in &self.layout {
            match item {
                settings::LayoutItem::Break(value) => {
                    if value.eq_ignore_ascii_case("break") {
                        final_output.push('\n');
                    } else {
                        final_output.push_str(value);
                        if !value.ends_with('\n') {
                            final_output.push('\n');
                        }
                    }
                }
                settings::LayoutItem::Module(module) => {
                    let Some(field_name) =
                        module.field_name().map(|name| name.to_ascii_lowercase())
                    else {
                        continue;
                    };

                    if module.is_custom() {
                        if let Some(text) = module.format.as_ref().or(module.text.as_ref()) {
                            final_output.push_str(text);
                            if !text.ends_with('\n') {
                                final_output.push('\n');
                            }
                        }
                        continue;
                    }

                    let label_storage = module
                        .label()
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| field_name.clone());
                    let label = label_storage.as_str();

                    match field_name.as_str() {
                        "titles" => {
                            let (user, host) = get_titles(true);
                            final_output.push_str(
                                format!(
                                    "${{c1}}{}${{reset}} {}${{c1}}@${{reset}}{}${{reset}}\n",
                                    label, user, host,
                                )
                                .as_str(),
                            );
                            self.data.username = Some(user);
                            self.data.hostname = Some(host);
                        }
                        "os" => {
                            let os = get_os();
                            final_output
                                .push_str(format!("${{c1}}{} ${{reset}}{}\n", label, os).as_str());
                            self.data.os = Some(os);
                        }
                        "distro" => {
                            let distro = get_distro(
                                DistroDisplay::from_str(&self.flags.distro_display)
                                    .unwrap_or(DistroDisplay::NameModelVersionArch),
                            );
                            final_output.push_str(
                                format!("${{c1}}{} ${{reset}}{}\n", label, distro).as_str(),
                            );
                            self.data.distro = Some(distro);
                        }
                        "model" => {
                            let model = get_model();
                            Self::is_some_add_to_output(label, &model, &mut final_output);
                            self.data.model = model;
                        }
                        "kernel" => {
                            let kernel = get_kernel();
                            Self::is_some_add_to_output(label, &kernel, &mut final_output);
                            self.data.kernel = kernel;
                        }
                        "os_age" => {
                            let uptime = get_os_age(
                                OsAgeShorthand::from_str(&self.flags.os_age_shorthand)
                                    .unwrap_or(OsAgeShorthand::Tiny),
                            );
                            Self::is_some_add_to_output(label, &uptime, &mut final_output);
                            self.data.uptime = uptime;
                        }
                        "uptime" => {
                            let uptime = get_uptime(
                                UptimeShorthand::from_str(&self.flags.uptime_shorthand)
                                    .unwrap_or(UptimeShorthand::Full),
                            );
                            Self::is_some_add_to_output(label, &uptime, &mut final_output);
                            self.data.uptime = uptime;
                        }
                        "packages" => {
                            let packages = get_packages(
                                PackageShorthand::from_str(&self.flags.package_managers)
                                    .unwrap_or(PackageShorthand::On),
                            );
                            Self::is_some_add_to_output(label, &packages, &mut final_output);
                            self.data.packages = packages;
                        }
                        "shell" => {
                            let shell = get_shell(self.flags.shell_path, self.flags.shell_version);
                            Self::is_some_add_to_output(label, &shell, &mut final_output);
                            self.data.shell = shell;
                        }
                        "wm" => {
                            if self.data.wm.is_none() {
                                self.data.wm = get_wm();
                            }
                            let wm = self.data.wm.clone();
                            Self::is_some_add_to_output(label, &wm, &mut final_output);
                        }
                        "de" => {
                            if self.data.wm.is_none() {
                                self.data.wm = get_wm();
                            }
                            let de = get_de(self.flags.de_version, self.data.wm.as_deref());
                            Self::is_some_add_to_output(label, &de, &mut final_output);
                            self.data.de = de;
                        }
                        "cpu" => {
                            let cpu = get_cpu(
                                self.flags.cpu_brand,
                                self.flags.cpu_frequency,
                                self.flags.cpu_cores,
                                self.flags.cpu_show_temp,
                                self.flags.cpu_speed,
                                match self.flags.cpu_temp {
                                    'f' | 'F' => Some('F'),
                                    'c' | 'C' => Some('C'),
                                    _ => None,
                                },
                            );
                            Self::is_some_add_to_output(label, &cpu, &mut final_output);
                            self.data.cpu = cpu;
                        }
                        "gpu" => {
                            let gpus = get_gpus();
                            if gpus.is_empty() {
                                final_output.push_str(
                                    format!("${{c1}}{} ${{reset}}{}\n", label, "No GPU found")
                                        .as_str(),
                                );
                            } else if gpus.len() == 1 {
                                final_output.push_str(
                                    format!("${{c1}}{} ${{reset}}{}\n", label, gpus[0]).as_str(),
                                );
                            } else {
                                for (index, gpu) in gpus.iter().enumerate() {
                                    final_output.push_str(
                                        format!("${{c1}}{} {}: ${{reset}}{}\n", label, index, gpu)
                                            .as_str(),
                                    );
                                }
                            }
                            self.data.gpu = Some(gpus);
                        }
                        "memory" => {
                            let memory = get_memory(
                                self.flags.memory_percent,
                                MemoryUnit::from_str(self.flags.memory_unit.as_str())
                                    .unwrap_or(MemoryUnit::MiB),
                            );
                            Self::is_some_add_to_output(label, &memory, &mut final_output);
                            self.data.memory = memory;
                        }
                        "disk" => {
                            if let Some(disks) = get_disks(
                                DiskSubtitle::from_str(self.flags.disk_subtitle.as_str())
                                    .unwrap_or(DiskSubtitle::Dir),
                                DiskDisplay::from_str(self.flags.disk_display.as_str())
                                    .unwrap_or(DiskDisplay::InfoBar),
                                None,
                            ) {
                                for disk in &disks {
                                    final_output.push_str(
                                        format!(
                                            "${{c1}}{} {}: ${{reset}}{}\n",
                                            label, disk.0, disk.1
                                        )
                                        .as_str(),
                                    );
                                }
                                self.data.disk = Some(disks);
                            } else {
                                final_output.push_str(
                                    format!("${{c1}}{} ${{reset}}{}\n", label, "No disks found")
                                        .as_str(),
                                );
                                self.data.disk = None;
                            }
                        }
                        "resolution" => {
                            let resolution = get_resolution(self.flags.refresh_rate);
                            Self::is_some_add_to_output(label, &resolution, &mut final_output);
                            self.data.resolution = resolution;
                        }
                        "theme" => {
                            if self.data.de.is_none() {
                                self.data.de =
                                    get_de(self.flags.de_version, self.data.wm.as_deref());
                            }
                            let theme = get_theme(self.data.de.as_deref());
                            Self::is_some_add_to_output(label, &theme, &mut final_output);
                            self.data.theme = theme;
                        }
                        "battery" => {
                            let display_mode =
                                BatteryDisplayMode::from_str(self.flags.battery_display.as_str())
                                    .unwrap_or(BatteryDisplayMode::BarInfo);
                            let batteries = get_battery(display_mode);

                            if batteries.is_empty() {
                                final_output.push_str(
                                    format!("${{c1}}{} ${{reset}}{}\n", label, "No Battery found")
                                        .as_str(),
                                );
                            } else if batteries.len() == 1 {
                                final_output.push_str(
                                    format!("${{c1}}{} ${{reset}}{}\n", label, batteries[0])
                                        .as_str(),
                                );
                            } else {
                                for (index, battery) in batteries.iter().enumerate() {
                                    final_output.push_str(
                                        format!(
                                            "${{c1}}{} {}: ${{reset}}{}\n",
                                            label, index, battery
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                            self.data.battery = Some(batteries);
                        }
                        "song" => {
                            let song = get_song();
                            if let Some(ref music) = song {
                                final_output.push_str(
                                    format!(
                                        "${{c1}}Playing${{reset}}\n    {}\n    {}\n",
                                        music.title, music.artist
                                    )
                                    .as_str(),
                                );
                            }
                            self.data.song = song;
                        }
                        "colors" => {
                            let color_blocks = if self.flags.color_blocks.is_empty() {
                                "●"
                            } else {
                                self.flags.color_blocks.as_str()
                            };

                            let colors = get_terminal_color(color_blocks);

                            final_output.push_str(
                                format!("${{c1}}{} ${{reset}}{}\n", label, &colors).as_str(),
                            );

                            self.data.colors = Some(colors);
                        }
                        other => {
                            final_output.push_str(
                                format!("${{c1}}{} ${{reset}}{}\n", label, other).as_str(),
                            );
                        }
                    }
                }
            }
        }

        self.output = final_output.clone();
        self.output.clone()
    }

    pub fn get_ascii_and_colors(
        &mut self,
        override_map: HashMap<&'static str, String>,
    ) -> (String, HashMap<&str, &str>) {
        let flags_path = self.flags.custom_ascii_path.as_str();
        let custom_ascii_path: Option<&str> = override_map
            .get("custom_ascii_path")
            .map(String::as_str)
            // If override exists: empty => use flags_path; non-empty => use override
            .map(|ov| if ov.is_empty() { flags_path } else { ov })
            // If override missing: keep flags_path only if non-empty; else None
            .or_else(|| (!flags_path.is_empty()).then_some(flags_path));

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

        (raw_ascii_art, distro_colors)
    }

    /// If the given `data` is `Some`, it will be added to `output` with the given `label`.
    /// If `data` is `None`, it will add a line to `output` with the label and the value "Unknown".
    fn is_some_add_to_output(label: &str, data: &Option<String>, output: &mut String) {
        match data {
            Some(d) => {
                output.push_str(format!("${{c1}}{} ${{reset}}{}\n", label, d).as_str());
            }
            None => {
                output.push_str(format!("${{c1}}{} ${{reset}}{}\n", label, "Unknown").as_str());
            }
        }
    }
}
