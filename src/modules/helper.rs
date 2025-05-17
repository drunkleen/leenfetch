pub fn print_help() {
    println!(
        r#"🧠 leenfetch — Minimal, Stylish System Info for Your Terminal

USAGE:
  leenfetch [OPTIONS]

OPTIONS:
  -h, --help               Show this help message and exit
  -i, --init               Create default config files in ~/.config/leenfetch/
  -r, --reinit             Reinitialize all config files to defaults
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
    • Linux:   ~/.config/leenfetch/flags.ron, toggles.ron, print_layout.ron
    • Windows: %APPDATA%/leenfetch/
    Edit these files to customize output, layout, and which blocks are shown.
    All options are explained with comments in the files themselves.

EXAMPLES:
  leenfetch                         🚀 Run normally with your config
  leenfetch --init                  🔧 Create default config files
  leenfetch --ascii_distro arch     🎨 Use Arch logo manually
  leenfetch --ascii_colors 2,7,3    🌈 Use custom colors
  leenfetch --list-options          📜 View all available configuration keys

TIPS:
  • Toggle info blocks in toggles.ron (e.g., show_cpu, show_gpu)
  • Change display style in flags.ron (e.g., disk_display, battery_display)
  • Rearrange output in print_layout.ron

For more, see the README or run `leenfetch --list-options`.
        "#
    );
}

pub fn list_options() {
    println!(
        r#"

📄 LeenFetch Configuration Options Reference
──────────────────────────────────────────────

📁 LeenFetch uses three config files in RON format:
  • 🖼️  flags.ron      — Display and formatting options
  • 🧩 toggles.ron    — Show/hide information blocks
  • 📝 print_layout.ron — Output order and labels

📂 All files are in ~/.config/leenfetch/ (Linux) or %APPDATA%/leenfetch/ (Windows).

──────────────────────────────────────────────
🖼️  flags.ron — Display and Formatting Options
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
  
  cpu_temp            = 'C' | 'F'
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
🧩 toggles.ron — Show/Hide Information Blocks
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
📝 print_layout.ron — Output Order and Labels
──────────────────────────────────────────────
  Each entry is:
    (label: <string>, field: <field_name>)
  - label: Text shown before the value (e.g., "CPU:"). Can be empty for no label.
  - field: Which data block to show. Valid fields:
      titles, distro, model, kernel, uptime, packages, shell, wm, de, wm_theme, cpu, gpu, memory, disk, resolution, theme, battery, song, colors

You can rearrange, remove, or relabel any section to customize your output.

──────────────────────────────────────────────
✏️  Edit these files in your favorite text editor. For a full explanation of each option, see the comments in the config files themselves.
        "#
    );
}



