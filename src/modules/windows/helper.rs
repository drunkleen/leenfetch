pub fn list_options() {
    println!(
        r#"
📄 leenfetch Windows Config Reference
──────────────────────────────────────────────────────────────────────────────

📂 Config File Path: %USERPROFILE%\.config\leenfetch\config.conf
📎 Layout Reference: Uses [[block]] tags like [[uptime]], [[cpu]], etc.
🎨 Colors: Use tokens like ${{bold.c5}}, ${{fg.c8}}, and ${{reset}} for styling.
──────────────────────────────────────────────────────────────────────────────

🖼️  Display Settings
──────────────────────────────────────────────────────────────────────────────
  title_fqdn         = on | off           Show full hostname (e.g. host.local)
  underline          = on | off           Underline below title block
  separator          = string             Custom separator (default: ":")

🧩 Output Toggles
──────────────────────────────────────────────────────────────────────────────
  show_ascii         = on | off           Display ASCII logo or art
  show_titles        = on | off           Show user@host title block
  show_os            = on | off           Display the base OS (e.g. Windows)
  show_distro        = on | off           Show the distribution string
  show_model         = on | off           Show PC model info
  show_kernel        = on | off           Display OS build/kernel version
  show_uptime        = on | off           Display system uptime
  show_packages      = on | off           Try to count packages (Chocolatey/Winget)
  show_shell         = on | off           Show current shell name/version
  show_wm            = on | off           Window Manager (usually DWM)
  show_de            = on | off           Desktop Environment (Explorer, etc)
  show_wm_theme      = on | off           Light/Dark mode (and other tweaks)
  show_cpu           = on | off           Show CPU info
  show_gpu           = on | off           Show detected GPU(s)
  show_memory        = on | off           Show memory usage
  show_resolution    = on | off           Show screen resolution (e.g. 1920x1080 @ 60Hz)
  show_theme         = on | off           Theme (GTK, Qt, Windows, etc)
  show_disks         = on | off           Show mounted disk usage
  show_battery       = on | off           Battery percentage/bar if supported
  show_song          = on | off           Show media song playing (Spotify, etc)
  show_terminal_colors = on | off         Show terminal color palette

⚙️  CPU Settings
──────────────────────────────────────────────────────────────────────────────
  cpu_brand          = on | off           Keep brand prefix (e.g. Intel, AMD)
  cpu_speed          = on | off           Show max frequency
  cpu_cores          = logical | physical | off
                                           Logical = threads; Physical = cores
  cpu_temp           = C | F | off        Display temperature (if supported)
  speed_shorthand    = on | off           Use e.g. "3.6GHz" instead of "3600MHz"

💾 Memory Settings
──────────────────────────────────────────────────────────────────────────────
  memory_unit        = kib | mib | gib    Choose display unit
  memory_percent     = on | off           Show memory usage as percentage

📦 Package Count
──────────────────────────────────────────────────────────────────────────────
  package_managers   = on | off | tiny    Show managers and/or package count
                                          "tiny" = count only

🖥️  Shell Settings
──────────────────────────────────────────────────────────────────────────────
  shell_path         = on | off           Show full path to shell executable
  shell_version      = on | off           Show shell version

🧮 Distro & Uptime
──────────────────────────────────────────────────────────────────────────────
  distro_display     = name | name_model | name_version | name_model_version_arch
                                          Full string output customization
  uptime_shorthand   = on | tiny | off    Format as "1d 2h" or "1 day, 2 hours"

💽 Disk Info
──────────────────────────────────────────────────────────────────────────────
  disk_display       = bar | barinfo | infobar | percentage | info
                                          "info" = 40G / 80G
                                          "barinfo" = [████] 40G / 80G
  disk_subtitle      = name | dir | mount | none
                                          Subtitle type: e.g. "C:", "Disk (C:)"
  disk_show          = string             Only display specified mount (e.g. "/")

🔋 Battery Info
──────────────────────────────────────────────────────────────────────────────
  battery_display    = bar | barinfo | infobar | off
                                          Show as percentage, bar, or both

🎵 Media Playback
──────────────────────────────────────────────────────────────────────────────
  Supported Players: Spotify, VLC, Foobar, etc.
  Uses window title or playerctl (for future MPRIS integration)

🖼️ Theme Detection
──────────────────────────────────────────────────────────────────────────────
  GTK/Qt themes: reads from gsettings or qt5ct.conf
  Windows: detects Light/Dark from registry

📁 Paths
──────────────────────────────────────────────────────────────────────────────
  ascii_path         = path               Use custom path for ASCII logo

"#
    );
}
