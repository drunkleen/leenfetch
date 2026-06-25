use clap::{ArgAction, Parser, ValueEnum};
use std::collections::{HashMap, HashSet};

/// Captures configuration overrides controlled by CLI switches.
#[derive(Default, Debug)]
pub struct CliOverrides {
    pub flags: HashMap<String, String>,
    pub only_modules: Option<Vec<String>>,
    pub hide_modules: HashSet<String>,
    pub config_path: Option<String>,
    pub use_defaults: bool,
    pub output_format: OutputFormat,
    pub ssh_hosts: Vec<String>,
}

impl CliOverrides {
    fn set_bool(&mut self, key: &str, value: bool) {
        self.flags.insert(
            key.to_string(),
            if value { "true" } else { "false" }.to_string(),
        );
    }

    fn set_string(&mut self, key: &str, value: String) {
        self.flags.insert(key.to_string(), value);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Pretty
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "leenfetch",
    about = "Minimal, stylish system info for your terminal",
    version,
    disable_help_flag = true
)]
pub struct Args {
    /// Show this help message and exit
    #[arg(short = 'h', long = "help", action = ArgAction::SetTrue)]
    pub help: bool,

    /// Output format: pretty (default) or json
    #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
    pub format: OutputFormat,

    /// Create the default config file in ~/.config/leenfetch/
    #[arg(short = 'i', long, action = ArgAction::SetTrue)]
    pub init: bool,

    /// Reinitialize the config file to defaults
    #[arg(short = 'r', long, action = ArgAction::SetTrue)]
    pub reinit: bool,

    /// Show all available config options and values
    #[arg(short = 'l', long = "list-options", action = ArgAction::SetTrue)]
    pub list_options: bool,

    /// Load configuration from a custom file
    #[arg(long = "config")]
    pub config_path: Option<String>,

    /// Ignore config files and use built-in defaults
    #[arg(long = "no-config", action = ArgAction::SetTrue)]
    pub no_config: bool,

    #[arg(long = "ascii_distro")]
    pub ascii_distro: Option<String>,
    #[arg(long = "ascii_colors")]
    pub ascii_colors: Option<String>,
    #[arg(long = "custom_ascii_path")]
    pub custom_ascii_path: Option<String>,
    #[arg(long = "battery_display")]
    pub battery_display: Option<String>,
    #[arg(long = "disk_display")]
    pub disk_display: Option<String>,
    #[arg(long = "disk_subtitle")]
    pub disk_subtitle: Option<String>,
    #[arg(long = "memory_unit")]
    pub memory_unit: Option<String>,
    #[arg(long = "package_managers", alias = "packages")]
    pub package_managers: Option<String>,
    #[arg(long = "uptime_shorthand", alias = "uptime")]
    pub uptime_shorthand: Option<String>,
    #[arg(long = "os_age_shorthand")]
    pub os_age_shorthand: Option<String>,
    #[arg(long = "distro_shorthand", alias = "distro-display")]
    pub distro_shorthand: Option<String>,
    #[arg(long = "color_blocks")]
    pub color_blocks: Option<String>,
    #[arg(long = "cpu_temp", alias = "cpu-temp-unit")]
    pub cpu_temp: Option<String>,
    #[arg(long = "only")]
    pub only_modules: Option<String>,
    #[arg(long = "hide")]
    pub hide_modules: Option<String>,
    #[arg(long = "disable")]
    pub disable_modules: Option<String>,

    #[arg(long = "memory_percent")]
    pub memory_percent: Option<bool>,
    #[arg(long = "cpu_speed")]
    pub cpu_speed: Option<bool>,
    #[arg(long = "cpu_frequency")]
    pub cpu_frequency: Option<bool>,
    #[arg(long = "cpu_cores")]
    pub cpu_cores: Option<bool>,
    #[arg(long = "cpu_brand")]
    pub cpu_brand: Option<bool>,
    #[arg(long = "shell_path")]
    pub shell_path: Option<bool>,
    #[arg(long = "shell_version")]
    pub shell_version: Option<bool>,
    #[arg(long = "de_version")]
    pub de_version: Option<bool>,
    #[arg(long = "gpu_brand")]
    pub gpu_brand: Option<bool>,
    #[arg(long = "kernel_shorthand")]
    pub kernel_shorthand: Option<bool>,
    #[arg(long = "speed_shorthand")]
    pub speed_shorthand: Option<bool>,
    #[arg(long = "disk_percent")]
    pub disk_percent: Option<bool>,
    #[arg(long = "gpu_type")]
    pub gpu_type: Option<String>,
    #[arg(long = "disk_show")]
    pub disk_show: Option<String>,

    /// Fetch info from remote hosts via SSH (e.g., user@host or host:port)
    #[arg(long = "ssh", value_name = "HOST")]
    pub ssh_hosts: Vec<String>,

    /// Print the default config to stdout and exit
    #[arg(long = "print_config", action = ArgAction::SetTrue)]
    pub print_config: bool,
}

impl Args {
    pub fn into_overrides(self) -> CliOverrides {
        let mut overrides = CliOverrides::default();
        overrides.use_defaults = self.no_config;
        overrides.config_path = self.config_path.clone();
        overrides.output_format = self.format;

        if let Some(val) = self.ascii_distro {
            overrides.set_string("ascii_distro", val);
        }
        if let Some(val) = self.ascii_colors {
            overrides.set_string("ascii_colors", val);
        }
        if let Some(val) = self.custom_ascii_path {
            overrides.set_string("custom_ascii_path", val);
        }
        if let Some(val) = self.battery_display {
            overrides.set_string("battery_display", val);
        }
        if let Some(val) = self.disk_display {
            overrides.set_string("disk_display", val);
        }
        if let Some(val) = self.disk_subtitle {
            overrides.set_string("disk_subtitle", val);
        }
        if let Some(val) = self.memory_unit {
            overrides.set_string("memory_unit", val);
        }
        if let Some(val) = self.package_managers {
            overrides.set_string("package_managers", val);
        }
        if let Some(val) = self.uptime_shorthand {
            overrides.set_string("uptime_shorthand", val);
        }
        if let Some(val) = self.os_age_shorthand {
            overrides.set_string("os_age_shorthand", val);
        }
        if let Some(val) = self.distro_shorthand {
            overrides.set_string("distro_shorthand", val);
        }
        if let Some(val) = self.color_blocks {
            overrides.set_string("color_blocks", val);
        }
        if let Some(val) = self.cpu_temp {
            overrides.set_string("cpu_temp", val);
        }
        if let Some(val) = self.gpu_type {
            overrides.set_string("gpu_type", val);
        }
        if let Some(val) = self.disk_show {
            overrides.set_string("disk_show", val);
        }

        if let Some(only) = self.only_modules {
            let modules = only
                .split(',')
                .map(|item| item.trim().to_string())
                .filter(|item| !item.is_empty())
                .collect::<Vec<_>>();
            overrides.only_modules = if modules.is_empty() {
                None
            } else {
                Some(modules)
            };
        }

        if let Some(hide) = self.hide_modules {
            for entry in hide.split(',') {
                let trimmed = entry.trim();
                if !trimmed.is_empty() {
                    overrides.hide_modules.insert(trimmed.to_string());
                }
            }
        }

        if let Some(disable) = self.disable_modules {
            for entry in disable.split(',') {
                let trimmed = entry.trim();
                if !trimmed.is_empty() {
                    overrides.hide_modules.insert(trimmed.to_string());
                }
            }
        }

        apply_bool_override(&mut overrides, "memory_percent", self.memory_percent);
        apply_bool_override(&mut overrides, "cpu_speed", self.cpu_speed);
        apply_bool_override(&mut overrides, "cpu_frequency", self.cpu_frequency);
        apply_bool_override(&mut overrides, "cpu_cores", self.cpu_cores);
        apply_bool_override(&mut overrides, "cpu_brand", self.cpu_brand);
        apply_bool_override(&mut overrides, "shell_path", self.shell_path);
        apply_bool_override(&mut overrides, "shell_version", self.shell_version);
        apply_bool_override(&mut overrides, "de_version", self.de_version);
        apply_bool_override(&mut overrides, "gpu_brand", self.gpu_brand);
        apply_bool_override(&mut overrides, "kernel_shorthand", self.kernel_shorthand);
        apply_bool_override(&mut overrides, "speed_shorthand", self.speed_shorthand);
        apply_bool_override(&mut overrides, "disk_percent", self.disk_percent);

        overrides.ssh_hosts = self.ssh_hosts.clone();

        overrides
    }
}

fn apply_bool_override(overrides: &mut CliOverrides, key: &str, value: Option<bool>) {
    if let Some(value) = value {
        overrides.set_bool(key, value);
    }
}

pub fn print_custom_help() {
    println!(
        "{}",
        r#"🧠 leenfetch — Minimal, Stylish System Info for Your Terminal

USAGE:
  leenfetch [OPTIONS]

OPTIONS:
  -V, --version            Print version information and exit
  -h, --help               Show this help message and exit
  -i, --init               Create the default config file in ~/.config/leenfetch/
  -r, --reinit             Reinitialize the config file to defaults
  -l, --list-options       Show all available config options and values
      --config <path>      Load configuration from a custom file
      --no-config          Ignore config files and use built-in defaults
      --ssh <host>         Fetch info from remote hosts via SSH (repeatable)
      --format <kind>      Output format: pretty (default) or json

  --ascii_distro <s>       Override detected distro (e.g., ubuntu, arch, arch_small)
  --ascii_colors <s>       Override color palette (e.g., 2,7,3 or "distro")
  --custom_ascii_path <p>  Use ASCII art from the given file path

  --battery_display <mode> Battery output style (off, bar, infobar, barinfo)
  --disk_display <mode>    Disk output style (info, percentage, infobar, barinfo, bar)
  --disk_subtitle <mode>   Disk subtitle (name, dir, none, mount)
  --disk_percent <bool>    Show disk percentage
  --disk_show <path>       Which disks to display (comma-separated mount points)
  --memory_unit <unit>     Force memory unit (kib, mib, gib)
  --package_managers <mode> Package summary verbosity (off, on, tiny)
  --uptime_shorthand <mode> Uptime shorthand (full, tiny, seconds)
  --os_age_shorthand <mode> OS age shorthand (full, tiny, seconds)
  --distro_shorthand <mode> Distro detail level (name, name_version, ...)
  --color_blocks <glyph>   Glyph used for color swatches
  --cpu_temp <unit>        CPU temperature unit (C, F, off)
  --gpu_type <type>        Which GPU to display (all, dedicated, integrated)
  --only <list>            Render only listed modules (comma-separated)
  --hide <list>            Hide listed modules (comma-separated)
  --disable <list>         Alias for --hide

  --memory_percent  <true|false>
  --cpu_speed       <true|false>
  --cpu_frequency   <true|false>
  --cpu_cores       <true|false>
  --cpu_brand       <true|false>
  --shell_path      <true|false>
  --shell_version   <true|false>
  --de_version      <true|false>
  --gpu_brand       <true|false>
  --kernel_shorthand <true|false>
  --speed_shorthand <true|false>
  --disk_percent    <true|false>

DESCRIPTION:
  leenfetch is a modern, minimal, and the fastest system info tool,
  written in Rust, designed for terminal enthusiasts.

  It fetches and prints system information like:
    • OS, Kernel, Uptime
    • CPU, GPU, Memory, Disks
    • Shell, WM, DE, Theme
    • Resolution, Battery, Current Song

  🛠️  Configuration:
    • Linux:   ~/.config/leenfetch/config.jsonc
    • Windows: %APPDATA%/leenfetch/config.jsonc
    One JSONC file with inline comments covering flags and a Fastfetch-style modules array.
    Edit it to control appearance, spacing (via "break" entries), and output order.

EXAMPLES:
  leenfetch                         🚀 Run normally with your config
  leenfetch --init                  🔧 Create the default config file
  leenfetch --ssh user@server       🌐 Fetch from a remote host over SSH
  leenfetch --ssh host1 --ssh host2 🛰️ Fetch multiple hosts sequentially
  leenfetch --ascii_distro arch     🎨 Use Arch logo manually
  leenfetch --ascii_colors 2,7,3    🌈 Use custom colors
  leenfetch --package_managers tiny 📦 Compact package summary for screenshots
  leenfetch --only cpu,memory       🧩 Focus on specific modules temporarily
  leenfetch --list-options          📜 View all available configuration keys

TIPS:
  • Adjust styles in the `flags` section (e.g., ascii_distro, disk_display, battery_display)
  • Reorder entries in the `modules` array (use "break" for spacing)
  • Tweak `logo.padding` to add margins around the ASCII art

For more, see the README or run `leenfetch --list-options`.
        "#
    );
}

pub fn list_options() {
    println!(
        "{}",
        r#"

📄 leenfetch Configuration Options Reference
──────────────────────────────────────────────

📁 leenfetch stores everything in a single JSONC file:
  • Linux:   ~/.config/leenfetch/config.jsonc
  • Windows: %APPDATA%/leenfetch/config.jsonc

🗂️  Sections inside config.jsonc:
  • 🖼️ flags — Display and formatting options
  • 🧱 modules — Output order and custom rows
──────────────────────────────────────────────
🖼️ flags — Display and Formatting Options
──────────────────────────────────────────────
  ascii_distro        = "auto" | <name>
      Which ASCII art to use. "auto" detects your distro or specify a distro name (e.g., "arch").
  
  ascii_colors        = "distro" | <list>
      Color palette for ASCII art. "distro" uses default, or provide a comma-separated list (e.g., "1,2,3,4").
  
  custom_ascii_path   = "" | <path>
      Path to a custom ASCII art file. Empty for default.
  
  battery_display     = "off" | "bar" | "infobar" | "barinfo"
      How to show battery info: none, bar only, info+bar, or bar+info.
  
  color_blocks        = <string>
      String used for color blocks (e.g., "███", "\#\#\#").
  
  cpu_brand           = true | false
      Show CPU brand name.
  
  cpu_cores           = true | false
      Show CPU core count.
  
  cpu_frequency       = true | false
      Show CPU frequency.
  
  cpu_speed           = true | false
      Show CPU speed.
  
  cpu_temp            = "C" | "F" | "off"
      Temperature unit for CPU: Celsius, Fahrenheit, or off.
  
  de_version          = true | false
      Show desktop environment version.
  
  distro_shorthand    = "name" | "name_version" | "name_arch" | "name_model" | "name_model_version" | "name_model_arch" | "name_model_version_arch"
      How much detail to show for OS info.
  
  disk_display        = "info" | "percentage" | "infobar" | "barinfo" | "bar"
      Disk usage display style.
  
  disk_percent        = true | false
      Show disk usage percentage.
  
  disk_show           = <path>
      Which disks to display (default "/").
  
  disk_subtitle       = "name" | "dir" | "none" | "mount"
      Disk label: device, last dir, none, or full mount point.
  
  gpu_brand           = true | false
      Show GPU vendor name.
  
  gpu_type            = "all" | "dedicated" | "integrated"
      Which GPU to display.
  
  kernel_shorthand    = true | false
      Shorten kernel output.
  
  memory_percent      = true | false
      Show memory as percent.
  
  memory_unit         = "mib" | "gib" | "kib"
      Memory unit.
  
  package_managers    = "off" | "on" | "tiny"
      Package info: none, full, or compact.
  
  shell_path          = true | false
      Show full shell path.
  
  shell_version       = true | false
      Show shell version.
  
  speed_shorthand     = true | false
      Show CPU speed without decimals.
  
  uptime_shorthand    = "full" | "tiny" | "seconds"
      Uptime format: verbose, compact, or seconds only.
  
  os_age_shorthand    = "full" | "tiny" | "seconds"
      Format for the OS install age module.

──────────────────────────────────────────────
🖼 logo — ASCII Art Overrides
──────────────────────────────────────────────
  type              = "auto" | "file"
      Select built-in art or load from disk.

  source            = <path>
      Path to a custom ASCII art file (used when type is "file").

  padding.top       = <number>
      Add blank lines above the ASCII logo.

  padding.right     = <number>
      Add spacing between the ASCII logo and information column.

  padding.left      = <number>
      Indent the ASCII logo horizontally.

──────────────────────────────────────────────
🧱 modules — Output Order and Custom Rows
──────────────────────────────────────────────
  Each entry is either:
    • "break" — insert a blank spacer line
    • { "type": <field>, "key": <label>, ... } — render a module
    • { "type": "custom", "text": "hello" } — literal text

  Common module fields:
    - "titles", "os", "distro", "model", "kernel", "os_age"
    - "uptime", "packages", "shell", "wm", "de", "cpu", "gpu"
    - "memory", "disk", "resolution", "theme", "battery", "song", "colors"
"#
    );
}
