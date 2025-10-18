use crate::config;
use std::collections::HashMap;

/// Parses command-line arguments and returns a map of configuration overrides.
///
/// This function processes a set of command-line arguments and performs actions
/// based on the provided flags. It supports the following flags:
///
/// - `--version` or `-v`: Prints the package name and version, then exits.
/// - `--help` or `-h`: Displays help information and exits.
/// - `--list-options` or `-l`: Lists available options and exits.
/// - `--init` or `-i`: Ensures configuration files exist, creating them if necessary, and exits.
/// - `--reinit` or `-r`: Deletes and regenerates configuration files, then exits.
/// - `--ascii_distro`: Sets the ASCII distribution to the specified value.
/// - `--ascii_colors`: Sets the ASCII colors to the specified value.
/// - `--custom_ascii_path`: Sets the custom ASCII path to the specified value.
///
/// Arguments:
/// - `args`: Mutable reference to the command-line arguments iterator.
///
/// Returns:
/// - `Ok(HashMap<&'static str, String>)`: A map of configuration overrides if successful.
/// - `Err(())`: If any flag requires an immediate exit after processing.
pub fn handle_args(args: &mut std::env::Args) -> Result<HashMap<&'static str, String>, ()> {
    let mut override_map: HashMap<&'static str, String> = HashMap::new();

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
            "--ascii_distro" => {
                if let Some(val) = args.next() {
                    override_map.insert("ascii_distro", val);
                } else {
                    println!("❌ ascii_distro cannot be empty");
                    println!("⚠️ use --help for more info");
                    return Err(());
                }
            }
            "--ascii_colors" => {
                if let Some(val) = args.next() {
                    override_map.insert("ascii_colors", val);
                } else {
                    println!("❌ ascii_colors cannot be empty");
                    println!("⚠️ use --help for more info");
                    return Err(());
                }
            }
            "--custom_ascii_path" => {
                if let Some(val) = args.next() {
                    override_map.insert("custom_ascii_path", val);
                } else {
                    println!("❌ custom_ascii_path cannot be empty");
                    println!("⚠️ use --help for more info");
                    return Err(());
                }
            }
            _ => {
                println!("❌ Unknown argument: {}", arg);
                print_help();
                return Err(());
            }
        }
    }
    Ok(override_map)
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

  --ascii_distro <s>       Override detected distro (e.g., ubuntu, arch, arch_small)
  --ascii_colors <s>       Override color palette (e.g., 2,7,3 or "distro")

DESCRIPTION:
  leenfetch is a fast, modern, and minimal system info tool,
  written in Rust, designed for terminal enthusiasts.

  It fetches and prints system information like:
    • OS, Kernel, Uptime
    • CPU, GPU, Memory, Disks
    • Shell, WM, DE, Theme
    • Resolution, Battery, Current Song

  🛠️  Configuration:
    • Linux:   ~/.config/leenfetch/config.jsonc
    • Windows: %APPDATA%/leenfetch/config.jsonc
    One JSONC file with inline comments covering flags, toggles, and a Fastfetch-style modules array.
    Edit it to control appearance, spacing (via "break" entries), and output order.

EXAMPLES:
  leenfetch                         🚀 Run normally with your config
  leenfetch --init                  🔧 Create default config files
  leenfetch --ascii_distro arch     🎨 Use Arch logo manually
  leenfetch --ascii_colors 2,7,3    🌈 Use custom colors
  leenfetch --list-options          📜 View all available configuration keys

TIPS:
  • Adjust styles in the `flags` section (e.g., ascii_distro, disk_display, battery_display)
  • Toggle info blocks in the `toggles` section (e.g., show_cpu, show_gpu)
  • Reorder entries in the `modules` array (use "break" for spacing)

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
  • 🧩 toggles — Show/hide information blocks
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

──────────────────────────────────────────────
🧩 toggles — Show/Hide Information Blocks
──────────────────────────────────────────────
  show_titles         = true | false   👤 Show user@host title
  show_os             = true | false   🖥️  Show base OS name
  show_distro         = true | false   🏷️  Show distro info
  show_model          = true | false   💻 Show hardware model
  show_uptime         = true | false   ⏱️  Show system uptime
  show_packages       = true | false   📦 Show package info
  show_shell          = true | false   🐚 Show shell info
  show_wm             = true | false   🪟 Show window manager
  show_de             = true | false   🖼️  Show desktop environment
  show_wm_theme       = true | false   🎨 Show WM theme
  show_kernel         = true | false   🧬 Show kernel version
  show_cpu            = true | false   🧠 Show CPU info
  show_gpu            = true | false   🎮 Show GPU info
  show_memory         = true | false   🧮 Show memory info
  show_song           = true | false   🎵 Show currently playing song/media info
  show_resolution     = true | false   🖵 Show display resolution
  show_theme          = true | false   🎭 Show GTK/Qt theme
  show_disks          = true | false   💽 Show disk usage
  show_battery        = true | false   🔋 Show battery info
  show_terminal_colors= true | false   🌈 Show terminal color palette

──────────────────────────────────────────────
🧱 modules — Output Order and Custom Rows
──────────────────────────────────────────────
  Each entry is either:
    • "break" — insert a blank spacer line
    • { "type": <field>, "key": <label>, ... } — render a module
  - type: Matches the collectors listed below under Information Blocks.
  - key: Optional label (set to "" for no label).
  - format: When type is "custom", print the string verbatim.

Add, remove, or duplicate modules to customize your output. Mix in "break" strings wherever you want spacing.

──────────────────────────────────────────────
✏️  Edit config.jsonc in your favorite text editor. Inline comments explain each option in detail.
        "#
    );
}
