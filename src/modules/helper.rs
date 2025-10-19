use crate::config;
use std::collections::{HashMap, HashSet};

/// Captures configuration overrides controlled by CLI switches.
#[derive(Default, Debug)]
pub struct CliOverrides {
    pub flags: HashMap<String, String>,
    pub only_modules: Option<Vec<String>>,
    pub hide_modules: HashSet<String>,
    pub config_path: Option<String>,
    pub use_defaults: bool,
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

/// Parses command-line arguments and returns CLI overrides.
///
/// Exits early (returns `Err(())`) when a flag performs an action such as `--help`.
pub fn handle_args(args: &mut std::env::Args) -> Result<CliOverrides, ()> {
    let mut overrides = CliOverrides::default();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--version" | "-v" => {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                return Err(());
            }
            "--help" | "-h" => {
                print_help();
                return Err(());
            }
            "--list-options" | "-l" => {
                list_options();
                return Err(());
            }
            "--init" | "-i" => {
                let results = config::ensure_config_files_exist();
                for (file, created) in results {
                    if created {
                        println!("✅ Created missing config: {file}");
                    } else {
                        println!("✔️ Config already exists: {file}");
                    }
                }
                return Err(());
            }
            "--reinit" | "-r" => {
                let result = config::delete_config_files();
                for (file, ok) in result {
                    println!(
                        "{} {}\n use --help for more info",
                        if ok {
                            "🗑️ Deleted"
                        } else {
                            "⚠️ Failed to delete"
                        },
                        file
                    );
                }

                let result = config::generate_config_files();
                for (file, ok) in result {
                    println!(
                        "{} {}\n use --help for more info",
                        if ok {
                            "✅ Generated"
                        } else {
                            "⚠️ Failed to generate"
                        },
                        file
                    );
                }
                return Err(());
            }
            "--config" => {
                let val = expect_value(args, "--config")?;
                overrides.use_defaults = false;
                overrides.config_path = Some(val);
            }
            "--no-config" => {
                overrides.use_defaults = true;
                overrides.config_path = None;
            }
            "--ascii_distro" => {
                let val = expect_value(args, "--ascii_distro")?;
                overrides.set_string("ascii_distro", val);
            }
            "--ascii_colors" => {
                let val = expect_value(args, "--ascii_colors")?;
                overrides.set_string("ascii_colors", val);
            }
            "--custom_ascii_path" => {
                let val = expect_value(args, "--custom_ascii_path")?;
                overrides.set_string("custom_ascii_path", val);
            }
            "--battery-display" => {
                let val = expect_value(args, "--battery-display")?;
                overrides.set_string("battery_display", val);
            }
            "--disk-display" => {
                let val = expect_value(args, "--disk-display")?;
                overrides.set_string("disk_display", val);
            }
            "--disk-subtitle" => {
                let val = expect_value(args, "--disk-subtitle")?;
                overrides.set_string("disk_subtitle", val);
            }
            "--memory-unit" => {
                let val = expect_value(args, "--memory-unit")?;
                overrides.set_string("memory_unit", val);
            }
            "--packages" | "--package-managers" => {
                let val = expect_value(args, arg.as_str())?;
                overrides.set_string("package_managers", val);
            }
            "--uptime" => {
                let val = expect_value(args, "--uptime")?;
                overrides.set_string("uptime_shorthand", val);
            }
            "--os-age" => {
                let val = expect_value(args, "--os-age")?;
                overrides.set_string("os_age_shorthand", val);
            }
            "--distro-display" => {
                let val = expect_value(args, "--distro-display")?;
                overrides.set_string("distro_display", val);
            }
            "--color-blocks" => {
                let val = expect_value(args, "--color-blocks")?;
                overrides.set_string("color_blocks", val);
            }
            "--cpu-temp-unit" => {
                let val = expect_value(args, "--cpu-temp-unit")?;
                overrides.set_string("cpu_temp", val);
            }
            "--only" => {
                let val = expect_value(args, "--only")?;
                let modules = val
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
            "--hide" => {
                let val = expect_value(args, "--hide")?;
                for entry in val.split(',') {
                    let trimmed = entry.trim();
                    if !trimmed.is_empty() {
                        overrides.hide_modules.insert(trimmed.to_string());
                    }
                }
            }
            "--memory-percent" => overrides.set_bool("memory_percent", true),
            "--no-memory-percent" => overrides.set_bool("memory_percent", false),
            "--cpu-show-temp" => overrides.set_bool("cpu_show_temp", true),
            "--no-cpu-show-temp" => overrides.set_bool("cpu_show_temp", false),
            "--cpu-speed" => overrides.set_bool("cpu_speed", true),
            "--no-cpu-speed" => overrides.set_bool("cpu_speed", false),
            "--cpu-frequency" => overrides.set_bool("cpu_frequency", true),
            "--no-cpu-frequency" => overrides.set_bool("cpu_frequency", false),
            "--cpu-cores" => overrides.set_bool("cpu_cores", true),
            "--no-cpu-cores" => overrides.set_bool("cpu_cores", false),
            "--cpu-brand" => overrides.set_bool("cpu_brand", true),
            "--no-cpu-brand" => overrides.set_bool("cpu_brand", false),
            "--shell-path" => overrides.set_bool("shell_path", true),
            "--no-shell-path" => overrides.set_bool("shell_path", false),
            "--shell-version" => overrides.set_bool("shell_version", true),
            "--no-shell-version" => overrides.set_bool("shell_version", false),
            "--refresh-rate" => overrides.set_bool("refresh_rate", true),
            "--no-refresh-rate" => overrides.set_bool("refresh_rate", false),
            "--de-version" => overrides.set_bool("de_version", true),
            "--no-de-version" => overrides.set_bool("de_version", false),
            _ => {
                println!("❌ Unknown argument: {}", arg);
                print_help();
                return Err(());
            }
        }
    }

    Ok(overrides)
}

fn expect_value(args: &mut std::env::Args, flag: &str) -> Result<String, ()> {
    if let Some(val) = args.next() {
        Ok(val)
    } else {
        println!("❌ {flag} requires a value");
        println!("⚠️ use --help for more info");
        Err(())
    }
}

pub fn print_help() {
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

  --ascii_distro <s>       Override detected distro (e.g., ubuntu, arch, arch_small)
  --ascii_colors <s>       Override color palette (e.g., 2,7,3 or "distro")
  --custom_ascii_path <p>  Use ASCII art from the given file path

  --battery-display <mode> Battery output style (off, bar, infobar, barinfo)
  --disk-display <mode>    Disk output style (info, percentage, infobar, barinfo, bar)
  --disk-subtitle <mode>   Disk subtitle (name, dir, none, mount)
  --memory-unit <unit>     Force memory unit (kib, mib, gib)
  --packages <mode>        Package summary verbosity (off, on, tiny)
  --uptime <mode>          Uptime shorthand (full, tiny, seconds)
  --os-age <mode>          OS age shorthand (full, tiny, seconds)
  --distro-display <mode>  Distro detail level (name, name_version, ...)
  --color-blocks <glyph>   Glyph used for color swatches
  --cpu-temp-unit <unit>   CPU temperature unit (C, F, off)
  --only <list>            Render only listed modules (comma-separated)
  --hide <list>            Hide listed modules (comma-separated)

  --memory-percent / --no-memory-percent
  --cpu-show-temp   / --no-cpu-show-temp
  --cpu-speed       / --no-cpu-speed
  --cpu-frequency   / --no-cpu-frequency
  --cpu-cores       / --no-cpu-cores
  --cpu-brand       / --no-cpu-brand
  --shell-path      / --no-shell-path
  --shell-version   / --no-shell-version
  --refresh-rate    / --no-refresh-rate
  --de-version      / --no-de-version

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
  leenfetch --ascii_distro arch     🎨 Use Arch logo manually
  leenfetch --ascii_colors 2,7,3    🌈 Use custom colors
  leenfetch --packages tiny         📦 Compact package summary for screenshots
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

📄 LeenFetch Configuration Options Reference
──────────────────────────────────────────────

📁 LeenFetch stores everything in a single JSONC file:
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
  
  cpu_temp            = "C" | "F"
      Temperature unit for CPU: Celsius or Fahrenheit.
  
  cpu_show_temp       = true | false
      Show CPU temperature.
  
  de_version          = true | false
      Show desktop environment version.
  
  distro_display      = "name" | "name_version" | "name_arch" | "name_model" | "name_model_version" | "name_model_arch" | "name_model_version_arch"
      How much detail to show for OS info.
  
  disk_display        = "info" | "percentage" | "infobar" | "barinfo" | "bar"
      Disk usage display style.
  
  disk_subtitle       = "name" | "dir" | "none" | "mount"
      Disk label: device, last dir, none, or full mount point.
  
  memory_percent      = true | false
      Show memory as percent.
  
  memory_unit         = "mib" | "gib" | "kib"
      Memory unit.
  
  package_managers    = "off" | "on" | "tiny"
      Package info: none, full, or compact.
  
  refresh_rate        = true | false
      Show display refresh rate.
  
  shell_path          = true | false
      Show full shell path.
  
  shell_version       = true | false
      Show shell version.
  
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
  - type: Matches the collectors listed below under Information Blocks.
  - key: Optional label (set to "" for no label).
  - keyColor: Optional color name (e.g., blue, bright_magenta).
  - format: When type is "custom", print the string verbatim.

Add, remove, or duplicate modules to customize your output. Mix in "break" strings wherever you want spacing.

──────────────────────────────────────────────
✏️  Edit config.jsonc in your favorite text editor. Inline comments explain each option in detail.
        "#
    );
}
