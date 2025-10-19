mod data;

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::Arc,
};

use once_cell::sync::OnceCell;
use rayon::prelude::*;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ModuleKind {
    Titles,
    Os,
    Distro,
    Model,
    Kernel,
    OsAge,
    Uptime,
    Packages,
    Shell,
    Wm,
    De,
    Cpu,
    Gpu,
    Memory,
    Disk,
    Resolution,
    Theme,
    Battery,
    Song,
    Colors,
}

impl ModuleKind {
    fn from_field_name(name: &str) -> Option<Self> {
        let normalized = name.trim().to_ascii_lowercase().replace('-', "_");
        match normalized.as_str() {
            "titles" => Some(Self::Titles),
            "os" => Some(Self::Os),
            "distro" => Some(Self::Distro),
            "model" => Some(Self::Model),
            "kernel" => Some(Self::Kernel),
            "os_age" => Some(Self::OsAge),
            "uptime" => Some(Self::Uptime),
            "packages" => Some(Self::Packages),
            "shell" => Some(Self::Shell),
            "wm" => Some(Self::Wm),
            "de" => Some(Self::De),
            "cpu" => Some(Self::Cpu),
            "gpu" => Some(Self::Gpu),
            "memory" => Some(Self::Memory),
            "disk" => Some(Self::Disk),
            "resolution" => Some(Self::Resolution),
            "theme" => Some(Self::Theme),
            "battery" => Some(Self::Battery),
            "song" => Some(Self::Song),
            "colors" => Some(Self::Colors),
            _ => None,
        }
    }
}

struct CollectContext {
    flags: settings::Flags,
    wm: OnceCell<Option<String>>,
    de: OnceCell<Option<String>>,
}

impl CollectContext {
    fn new(flags: settings::Flags) -> Self {
        Self {
            flags,
            wm: OnceCell::new(),
            de: OnceCell::new(),
        }
    }

    fn get_wm(&self) -> Option<String> {
        self.wm.get_or_init(get_wm).clone()
    }

    fn get_de(&self) -> Option<String> {
        self.de
            .get_or_init(|| {
                let wm = self.get_wm();
                get_de(self.flags.de_version, wm.as_deref())
            })
            .clone()
    }
}

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
    /// Each entry in the layout is resolved against the configured flags. Module data is collected
    /// in parallel, cached in `self.data`, and rendered in the configured order so that expensive
    /// lookups overlap without changing how the output is composed.
    ///
    /// # Returns
    ///
    /// A single colorized `String` containing the assembled module output.
    pub fn get_info_layout(&mut self) -> String {
        self.data = self.collect_data_parallel();

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
                    let Some(raw_field_name) = module.field_name() else {
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

                    let field_name = raw_field_name.to_ascii_lowercase();
                    let label_storage = module
                        .label()
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| field_name.clone());
                    let label = label_storage.as_str();

                    match ModuleKind::from_field_name(&field_name) {
                        Some(ModuleKind::Titles) => {
                            let username = self.data.username.as_deref().unwrap_or("Unknown");
                            let hostname = self.data.hostname.as_deref().unwrap_or("Unknown");
                            let titles_line = format!(
                                "${{c1}}{}${{reset}} {}${{c1}}@${{reset}}{}${{reset}}\n",
                                label, username, hostname,
                            );
                            final_output.push_str(&titles_line);
                        }
                        Some(ModuleKind::Os) => {
                            Self::is_some_add_to_output(label, &self.data.os, &mut final_output);
                        }
                        Some(ModuleKind::Distro) => {
                            Self::is_some_add_to_output(
                                label,
                                &self.data.distro,
                                &mut final_output,
                            );
                        }
                        Some(ModuleKind::Model) => {
                            Self::is_some_add_to_output(label, &self.data.model, &mut final_output);
                        }
                        Some(ModuleKind::Kernel) => {
                            Self::is_some_add_to_output(
                                label,
                                &self.data.kernel,
                                &mut final_output,
                            );
                        }
                        Some(ModuleKind::OsAge) => {
                            Self::is_some_add_to_output(
                                label,
                                &self.data.os_age,
                                &mut final_output,
                            );
                        }
                        Some(ModuleKind::Uptime) => {
                            Self::is_some_add_to_output(
                                label,
                                &self.data.uptime,
                                &mut final_output,
                            );
                        }
                        Some(ModuleKind::Packages) => {
                            Self::is_some_add_to_output(
                                label,
                                &self.data.packages,
                                &mut final_output,
                            );
                        }
                        Some(ModuleKind::Shell) => {
                            Self::is_some_add_to_output(label, &self.data.shell, &mut final_output);
                        }
                        Some(ModuleKind::Wm) => {
                            Self::is_some_add_to_output(label, &self.data.wm, &mut final_output);
                        }
                        Some(ModuleKind::De) => {
                            Self::is_some_add_to_output(label, &self.data.de, &mut final_output);
                        }
                        Some(ModuleKind::Cpu) => {
                            Self::is_some_add_to_output(label, &self.data.cpu, &mut final_output);
                        }
                        Some(ModuleKind::Gpu) => match self.data.gpu.as_ref() {
                            Some(gpus) if gpus.is_empty() => {
                                let line =
                                    format!("${{c1}}{} ${{reset}}{}\n", label, "No GPU found");
                                final_output.push_str(&line);
                            }
                            Some(gpus) if gpus.len() == 1 => {
                                let line = format!("${{c1}}{} ${{reset}}{}\n", label, gpus[0]);
                                final_output.push_str(&line);
                            }
                            Some(gpus) => {
                                for gpu in gpus {
                                    let line = format!("${{c1}}{} ${{reset}}{}\n", label, gpu);
                                    final_output.push_str(&line);
                                }
                            }
                            None => Self::push_unknown(label, &mut final_output),
                        },
                        Some(ModuleKind::Memory) => {
                            Self::is_some_add_to_output(
                                label,
                                &self.data.memory,
                                &mut final_output,
                            );
                        }
                        Some(ModuleKind::Disk) => match self.data.disk.as_ref() {
                            Some(disks) => {
                                if disks.is_empty() {
                                    let line = format!(
                                        "${{c1}}{} ${{reset}}{}\n",
                                        label, "No disks found"
                                    );
                                    final_output.push_str(&line);
                                } else {
                                    for (name, summary) in disks {
                                        let line = format!(
                                            "${{c1}}{} {} ${{reset}}{}\n",
                                            label, name, summary
                                        );
                                        final_output.push_str(&line);
                                    }
                                }
                            }
                            None => {
                                let line =
                                    format!("${{c1}}{} ${{reset}}{}\n", label, "No disks found");
                                final_output.push_str(&line);
                            }
                        },
                        Some(ModuleKind::Resolution) => {
                            Self::is_some_add_to_output(
                                label,
                                &self.data.resolution,
                                &mut final_output,
                            );
                        }
                        Some(ModuleKind::Theme) => {
                            Self::is_some_add_to_output(label, &self.data.theme, &mut final_output);
                        }
                        Some(ModuleKind::Battery) => match self.data.battery.as_ref() {
                            Some(batteries) if batteries.is_empty() => {
                                let line =
                                    format!("${{c1}}{} ${{reset}}{}\n", label, "No Battery found");
                                final_output.push_str(&line);
                            }
                            Some(batteries) if batteries.len() == 1 => {
                                let line = format!("${{c1}}{} ${{reset}}{}\n", label, batteries[0]);
                                final_output.push_str(&line);
                            }
                            Some(batteries) => {
                                for (index, battery) in batteries.iter().enumerate() {
                                    let line = format!(
                                        "${{c1}}{} {}: ${{reset}}{}\n",
                                        label, index, battery
                                    );
                                    final_output.push_str(&line);
                                }
                            }
                            None => {
                                let line =
                                    format!("${{c1}}{} ${{reset}}{}\n", label, "No Battery found");
                                final_output.push_str(&line);
                            }
                        },
                        Some(ModuleKind::Song) => {
                            if let Some(music) = self.data.song.as_ref() {
                                let line = format!(
                                    "${{c1}}Playing${{reset}}\n    {}\n    {}\n",
                                    music.title, music.artist
                                );
                                final_output.push_str(&line);
                            }
                        }
                        Some(ModuleKind::Colors) => {
                            Self::is_some_add_to_output(
                                label,
                                &self.data.colors,
                                &mut final_output,
                            );
                        }
                        None => {
                            let fallback_line =
                                format!("${{c1}}{} ${{reset}}{}\n", label, field_name);
                            final_output.push_str(&fallback_line);
                        }
                    }
                }
            }
        }

        self.output = final_output.clone();
        self.output.clone()
    }

    fn collect_data_parallel(&self) -> Data {
        let mut required = HashSet::new();

        for item in &self.layout {
            if let settings::LayoutItem::Module(module) = item {
                if module.is_custom() {
                    continue;
                }

                if let Some(field_name) = module.field_name() {
                    if let Some(kind) = ModuleKind::from_field_name(field_name) {
                        required.insert(kind);
                    }
                }
            }
        }

        if required.is_empty() {
            return Data::default();
        }

        let context = Arc::new(CollectContext::new(self.flags.clone()));
        let modules: Vec<_> = required.into_iter().collect();

        let results: Vec<Data> = modules
            .into_par_iter()
            .map(|kind| Self::collect_module_data(kind, context.clone()))
            .collect();

        let mut data = Data::default();
        for update in results {
            Self::merge_data(&mut data, update);
        }

        data
    }

    fn collect_module_data(kind: ModuleKind, context: Arc<CollectContext>) -> Data {
        let mut data = Data::default();
        let flags = &context.flags;

        match kind {
            ModuleKind::Titles => {
                let (user, host) = get_titles(true);
                data.username = Some(user);
                data.hostname = Some(host);
            }
            ModuleKind::Os => {
                data.os = Some(get_os());
            }
            ModuleKind::Distro => {
                let display = DistroDisplay::from_str(&flags.distro_display)
                    .unwrap_or(DistroDisplay::NameModelVersionArch);
                data.distro = Some(get_distro(display));
            }
            ModuleKind::Model => {
                data.model = get_model();
            }
            ModuleKind::Kernel => {
                data.kernel = get_kernel();
            }
            ModuleKind::OsAge => {
                let os_age = get_os_age(
                    OsAgeShorthand::from_str(&flags.os_age_shorthand)
                        .unwrap_or(OsAgeShorthand::Tiny),
                );
                data.os_age = os_age;
            }
            ModuleKind::Uptime => {
                let uptime = get_uptime(
                    UptimeShorthand::from_str(&flags.uptime_shorthand)
                        .unwrap_or(UptimeShorthand::Full),
                );
                data.uptime = uptime;
            }
            ModuleKind::Packages => {
                let packages = get_packages(
                    PackageShorthand::from_str(&flags.package_managers)
                        .unwrap_or(PackageShorthand::On),
                );
                data.packages = packages;
            }
            ModuleKind::Shell => {
                data.shell = get_shell(flags.shell_path, flags.shell_version);
            }
            ModuleKind::Wm => {
                data.wm = context.get_wm();
            }
            ModuleKind::De => {
                data.wm = context.get_wm();
                data.de = context.get_de();
            }
            ModuleKind::Cpu => {
                data.cpu = get_cpu(
                    flags.cpu_brand,
                    flags.cpu_frequency,
                    flags.cpu_cores,
                    flags.cpu_show_temp,
                    flags.cpu_speed,
                    match flags.cpu_temp {
                        'f' | 'F' => Some('F'),
                        'c' | 'C' => Some('C'),
                        _ => None,
                    },
                );
            }
            ModuleKind::Gpu => {
                data.gpu = Some(get_gpus());
            }
            ModuleKind::Memory => {
                data.memory = get_memory(
                    flags.memory_percent,
                    MemoryUnit::from_str(flags.memory_unit.as_str()).unwrap_or(MemoryUnit::MiB),
                );
            }
            ModuleKind::Disk => {
                data.disk = get_disks(
                    DiskSubtitle::from_str(flags.disk_subtitle.as_str())
                        .unwrap_or(DiskSubtitle::Dir),
                    DiskDisplay::from_str(flags.disk_display.as_str())
                        .unwrap_or(DiskDisplay::InfoBar),
                    None,
                );
            }
            ModuleKind::Resolution => {
                data.resolution = get_resolution(flags.refresh_rate);
            }
            ModuleKind::Theme => {
                let de = context.get_de();
                data.de = de.clone();
                data.theme = get_theme(de.as_deref());
            }
            ModuleKind::Battery => {
                let mode = BatteryDisplayMode::from_str(flags.battery_display.as_str())
                    .unwrap_or(BatteryDisplayMode::BarInfo);
                let batteries = get_battery(mode);
                data.battery = Some(batteries);
            }
            ModuleKind::Song => {
                data.song = get_song();
            }
            ModuleKind::Colors => {
                let color_blocks = if flags.color_blocks.is_empty() {
                    "●"
                } else {
                    flags.color_blocks.as_str()
                };
                let colors = get_terminal_color(color_blocks);
                data.colors = Some(colors);
            }
        }

        data
    }

    fn merge_data(target: &mut Data, update: Data) {
        if let Some(username) = update.username {
            target.username = Some(username);
        }
        if let Some(hostname) = update.hostname {
            target.hostname = Some(hostname);
        }
        if let Some(os) = update.os {
            target.os = Some(os);
        }
        if let Some(distro) = update.distro {
            target.distro = Some(distro);
        }
        if let Some(model) = update.model {
            target.model = Some(model);
        }
        if let Some(kernel) = update.kernel {
            target.kernel = Some(kernel);
        }
        if let Some(os_age) = update.os_age {
            target.os_age = Some(os_age);
        }
        if let Some(uptime) = update.uptime {
            target.uptime = Some(uptime);
        }
        if let Some(packages) = update.packages {
            target.packages = Some(packages);
        }
        if let Some(shell) = update.shell {
            target.shell = Some(shell);
        }
        if let Some(wm) = update.wm {
            target.wm = Some(wm);
        }
        if let Some(de) = update.de {
            target.de = Some(de);
        }
        if let Some(cpu) = update.cpu {
            target.cpu = Some(cpu);
        }
        if let Some(gpu) = update.gpu {
            target.gpu = Some(gpu);
        }
        if let Some(memory) = update.memory {
            target.memory = Some(memory);
        }
        if let Some(disk) = update.disk {
            target.disk = Some(disk);
        }
        if let Some(resolution) = update.resolution {
            target.resolution = Some(resolution);
        }
        if let Some(theme) = update.theme {
            target.theme = Some(theme);
        }
        if let Some(battery) = update.battery {
            target.battery = Some(battery);
        }
        if let Some(song) = update.song {
            target.song = Some(song);
        }
        if let Some(colors) = update.colors {
            target.colors = Some(colors);
        }
    }

    fn push_unknown(label: &str, output: &mut String) {
        output.push_str(format!("${{c1}}{} ${{reset}}{}\n", label, "Unknown").as_str());
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
