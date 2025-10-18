mod data;

use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};

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
            get_ascii_and_colors, get_custom_ascii, get_custom_colors_order, get_distro_colors,
            get_terminal_color,
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
    pub fn get_info_layout(&mut self) -> String {
        let output = Arc::new(Mutex::new(HashMap::new()));
        let data = Arc::new(Mutex::new(self.data.clone()));
        let mut handles = vec![];

        for (index, info) in self.layout.iter().enumerate() {
            if !Self::should_collect(info.field.as_str(), &self.toggles) {
                continue;
            }
            let output = Arc::clone(&output);
            let data = Arc::clone(&data);
            let info = info.clone();
            let toggles = self.toggles.clone();
            let flags = self.flags.clone();

            let handle = thread::spawn(move || {
                let mut result = String::new();
                match info.field.as_str() {
                    "titles" => {
                        if !toggles.show_titles {
                            return;
                        }
                        let (user, host, dash_len) = get_titles(true);
                        result.push_str(
                            format!(
                                "${{c1}}{}${{reset}}@${{c1}}{}${{reset}}\n{}\n",
                                user,
                                host,
                                "-".repeat(dash_len)
                            )
                            .as_str(),
                        );
                        let mut data = data.lock().unwrap();
                        data.username = Some(user);
                        data.hostname = Some(host);
                    }
                    "os" => {
                        if !toggles.show_os {
                            return;
                        }
                        let os = get_os();
                        result.push_str(
                            format!("${{c1}}{}: ${{reset}}{}\n", info.label, os).as_str(),
                        );
                        let mut data = data.lock().unwrap();
                        data.os = Some(os);
                    }
                    "distro" => {
                        if !toggles.show_distro {
                            return;
                        }
                        let distro = get_distro(
                            DistroDisplay::from_str(&flags.distro_display)
                                .unwrap_or(DistroDisplay::NameModelVersionArch),
                        );
                        result.push_str(
                            format!("${{c1}}{}: ${{reset}}{}\n", info.label, distro).as_str(),
                        );
                        let mut data = data.lock().unwrap();
                        data.distro = Some(distro);
                    }
                    "model" => {
                        if !toggles.show_model {
                            return;
                        }
                        let model = get_model();
                        Self::is_some_add_to_output(&info.label, &model, &mut result);
                        let mut data = data.lock().unwrap();
                        data.model = model;
                    }
                    "kernel" => {
                        if !toggles.show_kernel {
                            return;
                        }
                        let kernel = get_kernel();
                        Self::is_some_add_to_output(&info.label, &kernel, &mut result);
                        let mut data = data.lock().unwrap();
                        data.kernel = kernel;
                    }
                    "uptime" => {
                        if !toggles.show_uptime {
                            return;
                        }
                        let uptime = get_uptime(
                            UptimeShorthand::from_str(&flags.uptime_shorthand)
                                .unwrap_or(UptimeShorthand::Full),
                        );
                        Self::is_some_add_to_output(&info.label, &uptime, &mut result);
                        let mut data = data.lock().unwrap();
                        data.uptime = uptime;
                    }
                    "packages" => {
                        if !toggles.show_packages {
                            return;
                        }
                        let packages = get_packages(
                            PackageShorthand::from_str(&flags.package_managers)
                                .unwrap_or(PackageShorthand::On),
                        );
                        Self::is_some_add_to_output(&info.label, &packages, &mut result);
                        let mut data = data.lock().unwrap();
                        data.packages = packages;
                    }
                    "shell" => {
                        if !toggles.show_shell {
                            return;
                        }
                        let shell = get_shell(flags.shell_path, flags.shell_version);
                        Self::is_some_add_to_output(&info.label, &shell, &mut result);
                        let mut data = data.lock().unwrap();
                        data.shell = shell;
                    }
                    "wm" => {
                        if !toggles.show_wm {
                            return;
                        }
                        let wm = {
                            let mut data = data.lock().unwrap();
                            if data.wm.is_none() {
                                data.wm = get_wm();
                            }
                            data.wm.clone()
                        };
                        Self::is_some_add_to_output(&info.label, &wm, &mut result);
                    }
                    "de" => {
                        if !toggles.show_de {
                            return;
                        }
                        let de = {
                            let mut data = data.lock().unwrap();
                            if data.wm.is_none() {
                                data.wm = get_wm();
                            }
                            get_de(flags.de_version, data.wm.as_deref())
                        };
                        Self::is_some_add_to_output(&info.label, &de, &mut result);
                        let mut data = data.lock().unwrap();
                        data.de = de;
                    }
                    "wm_theme" => {
                        if !toggles.show_wm_theme {
                            return;
                        }
                        let (wm, de) = {
                            let mut data = data.lock().unwrap();
                            if data.wm.is_none() {
                                data.wm = get_wm();
                            }
                            if data.de.is_none() {
                                data.de = get_de(toggles.show_wm, data.wm.as_deref());
                            }
                            (
                                data.wm.as_deref().unwrap_or("").to_string(),
                                data.de.clone(),
                            )
                        };
                        let wm_theme = get_wm_theme(&wm, de.as_deref());
                        Self::is_some_add_to_output(&info.label, &wm_theme, &mut result);
                        let mut data = data.lock().unwrap();
                        data.wm_theme = wm_theme;
                    }
                    "cpu" => {
                        if !toggles.show_cpu {
                            return;
                        }
                        let cpu = get_cpu(
                            flags.cpu_brand,
                            flags.cpu_frequency,
                            flags.cpu_cores,
                            flags.cpu_show_temp,
                            flags.cpu_speed,
                            match flags.cpu_temp {
                                'f' | 'F' => Some('F'),
                                _ => Some('C'),
                            },
                        );
                        Self::is_some_add_to_output(&info.label, &cpu, &mut result);
                        let mut data = data.lock().unwrap();
                        data.cpu = cpu;
                    }
                    "gpu" => {
                        if !toggles.show_gpu {
                            return;
                        }
                        let gpus = get_gpus();
                        if gpus.is_empty() {
                            result.push_str(
                                format!("${{c1}}{}: ${{reset}}{}\n", &info.label, "No GPU found")
                                    .as_str(),
                            );
                        } else if gpus.len() == 1 {
                            result.push_str(
                                format!("${{c1}}{}: ${{reset}}{}\n", &info.label, gpus[0]).as_str(),
                            );
                        } else {
                            for (index, gpu) in gpus.iter().enumerate() {
                                result.push_str(
                                    format!(
                                        "${{c1}}{} #{}: ${{reset}}{}\n",
                                        &info.label, index, gpu
                                    )
                                    .as_str(),
                                );
                            }
                        }
                        let mut data = data.lock().unwrap();
                        data.gpu = Some(gpus);
                    }
                    "memory" => {
                        if !toggles.show_memory {
                            return;
                        }
                        let memory = get_memory(
                            flags.memory_percent,
                            MemoryUnit::from_str(flags.memory_unit.as_str())
                                .unwrap_or(MemoryUnit::MiB),
                        );
                        Self::is_some_add_to_output(&info.label, &memory, &mut result);
                        let mut data = data.lock().unwrap();
                        data.memory = memory;
                    }
                    "disk" => {
                        if toggles.show_disks {
                            if let Some(disks) = get_disks(
                                DiskSubtitle::from_str(flags.disk_subtitle.as_str())
                                    .unwrap_or(DiskSubtitle::Dir),
                                DiskDisplay::from_str(flags.disk_display.as_str())
                                    .unwrap_or(DiskDisplay::InfoBar),
                                None,
                            ) {
                                for disk in &disks {
                                    result.push_str(
                                        format!(
                                            "${{c1}}{} {}: ${{reset}}{}\n",
                                            &info.label, disk.0, disk.1
                                        )
                                        .as_str(),
                                    );
                                }
                                let mut data = data.lock().unwrap();
                                data.disk = Some(disks);
                            } else {
                                result.push_str(
                                    format!(
                                        "${{c1}}{}: ${{reset}}{}\n",
                                        &info.label, "No disks found"
                                    )
                                    .as_str(),
                                );
                                let mut data = data.lock().unwrap();
                                data.disk = None;
                            }
                        }
                    }
                    "resolution" => {
                        if toggles.show_resolution {
                            let resolution = get_resolution(flags.refresh_rate);
                            Self::is_some_add_to_output(&info.label, &resolution, &mut result);
                            let mut data = data.lock().unwrap();
                            data.resolution = resolution;
                        }
                    }
                    "theme" => {
                        if !toggles.show_theme {
                            return;
                        }
                        let theme = {
                            let mut data = data.lock().unwrap();
                            if data.de.is_none() {
                                data.de = get_de(toggles.show_wm, data.wm.as_deref());
                            }
                            get_theme(data.de.as_deref())
                        };
                        Self::is_some_add_to_output(&info.label, &theme, &mut result);
                        let mut data = data.lock().unwrap();
                        data.theme = theme;
                    }
                    "battery" => {
                        if !toggles.show_battery {
                            return;
                        }

                        let battery_display_mode =
                            BatteryDisplayMode::from_str(&flags.battery_display);
                        let batteries = get_battery(
                            battery_display_mode.unwrap_or(BatteryDisplayMode::BarInfo),
                        );

                        if batteries.is_empty() {
                            result.push_str(
                                format!(
                                    "${{c1}}{}: ${{reset}}{}\n",
                                    &info.label, "No Battery found"
                                )
                                .as_str(),
                            );
                        } else if batteries.len() == 1 {
                            result.push_str(
                                format!("${{c1}}{}: ${{reset}}{}\n", &info.label, batteries[0])
                                    .as_str(),
                            );
                        } else {
                            for (index, battery) in batteries.iter().enumerate() {
                                result.push_str(
                                    format!(
                                        "${{c1}}{} {}: ${{reset}}{}\n",
                                        &info.label, index, battery
                                    )
                                    .as_str(),
                                );
                            }
                        }
                        let mut data = data.lock().unwrap();
                        data.battery = Some(batteries);
                    }
                    "song" => {
                        if !toggles.show_song {
                            return;
                        }
                        let song = get_song();
                        if song.is_none() {
                            return;
                        }
                        let mut data = data.lock().unwrap();
                        data.song = song.clone();

                        if let Some(music) = song {
                            data.song = Some(music.clone());
                            result.push_str(
                                format!(
                                    "${{c1}}Playing${{reset}}\n    {}\n    {}\n",
                                    music.title, music.artist
                                )
                                .as_str(),
                            );
                        }
                    }
                    "colors" => {
                        if !toggles.show_terminal_colors {
                            return;
                        }
                        let color_blocks = if flags.color_blocks != "" {
                            flags.color_blocks.as_str()
                        } else {
                            "███"
                        };

                        let colors = get_terminal_color(color_blocks).join("\n");
                        result.push_str(format!("\n{}", colors).as_str());
                        let mut data = data.lock().unwrap();
                        data.colors = Some(colors);
                    }
                    other => {
                        result.push_str(
                            format!("${{c1}}{}: ${{reset}}{}\n", &info.label, other).as_str(),
                        );
                    }
                }
                if !result.is_empty() {
                    let mut output = output.lock().unwrap();
                    output.insert(index, result);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Combine results in order
        let output_map = Arc::try_unwrap(output).unwrap().into_inner().unwrap();
        let mut final_output = String::new();
        for i in 0..self.layout.len() {
            if let Some(s) = output_map.get(&i) {
                final_output.push_str(s);
            }
        }

        self.data = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
        self.output = final_output;
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
                output.push_str(format!("${{c1}}{}: ${{reset}}{}\n", label, d).as_str());
            }
            None => {
                output.push_str(format!("${{c1}}{}: ${{reset}}{}\n", label, "Unknown").as_str());
            }
        }
    }

    fn should_collect(field: &str, toggles: &settings::Toggles) -> bool {
        match field {
            "titles" => toggles.show_titles,
            "os" => toggles.show_os,
            "distro" => toggles.show_distro,
            "model" => toggles.show_model,
            "kernel" => toggles.show_kernel,
            "uptime" => toggles.show_uptime,
            "packages" => toggles.show_packages,
            "shell" => toggles.show_shell,
            "wm" => toggles.show_wm,
            "de" => toggles.show_de,
            "wm_theme" => toggles.show_wm_theme,
            "cpu" => toggles.show_cpu,
            "gpu" => toggles.show_gpu,
            "memory" => toggles.show_memory,
            "disk" => toggles.show_disks,
            "resolution" => toggles.show_resolution,
            "theme" => toggles.show_theme,
            "battery" => toggles.show_battery,
            "song" => toggles.show_song,
            "colors" => toggles.show_terminal_colors,
            _ => true,
        }
    }
}
