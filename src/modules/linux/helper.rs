pub fn list_options() {
    println!(
        r#"
📄 leenfetch Linux Config Reference
──────────────────────────────────────────────────────────────────────────────

📂 Config File Path: ~/.config/leenfetch/config.conf
📎 Layout Reference: Uses [block] tags like [cpu], [uptime], etc.
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
  show_os            = on | off           Show kernel's base OS name
  show_distro        = on | off           Show distribution string
  show_model         = on | off           Show hardware model
  show_kernel        = on | off           Show kernel version
  show_uptime        = on | off           Show system uptime
  show_packages      = on | off           Show installed package count
  show_shell         = on | off           Show current shell name/version
  show_wm            = on | off           Window manager (e.g. i3, Mutter)
  show_de            = on | off           Desktop environment (e.g. GNOME, KDE)
  show_wm_theme      = on | off           WM theme name (Openbox, KWin, etc.)
  show_cpu           = on | off           Show CPU info
  show_gpu           = on | off           Show detected GPU(s)
  show_memory        = on | off           Show memory usage
  show_resolution    = on | off           Show display resolution
  show_theme         = on | off           GTK/Qt/DE theme name
  show_disks         = on | off           Show disk usage
  show_battery       = on | off           Show battery info
  show_song          = on | off           Show current playing track
  show_terminal_colors = on | off         Show terminal color palette

⚙️  CPU Settings
──────────────────────────────────────────────────────────────────────────────
  cpu_brand          = on | off           Keep brand prefix (e.g. Intel, AMD)
  cpu_speed          = on | off           Show max frequency
  cpu_cores          = logical | physical | off
                                           Logical = threads; Physical = cores
  cpu_temp           = C | F | off        Display temperature if supported
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
  shell_path         = on | off           Show full shell path
  shell_version      = on | off           Show shell version

🧮 Distro & Uptime
──────────────────────────────────────────────────────────────────────────────
  distro_shorthand   = on | tiny | off    Format name (e.g. Arch → Arch Linux)
  uptime_shorthand   = on | tiny | off    Format as "1d 2h" or "1 day, 2 hours"

💽 Disk Info
──────────────────────────────────────────────────────────────────────────────
  disk_display       = bar | barinfo | infobar | percentage | info
                                          "info" = 40G / 80G
                                          "barinfo" = 40G / 80G [██████]
  disk_subtitle      = name | dir | mount | none
                                          e.g. "Disk (/dev/sda1)", "Disk (home)"
  disk_show          = string             Show specific mount point (e.g. /home)

🔋 Battery Info
──────────────────────────────────────────────────────────────────────────────
  battery_display    = bar | barinfo | infobar | off
                                          Show as percentage, bar, or both

🎵 Media Playback
──────────────────────────────────────────────────────────────────────────────
  MUSIC_PLAYER       = (env var)          Preferred player: mocp, spotify, vlc, etc.
  Auto-detects: playerctl, mpd, clementine, mocp, etc.

🖼️ Theme Detection
──────────────────────────────────────────────────────────────────────────────
  GTK3: via gsettings
  GTK2: ~/.gtkrc-2.0
  GTK4: ~/.config/gtk-4.0/settings.ini
  Qt: ~/.config/qt5ct/qt5ct.conf or qt6ct.conf
  KDE: ~/.config/kdeglobals

📁 Paths
──────────────────────────────────────────────────────────────────────────────
  ascii_path         = path               Custom ASCII art path

"#
    );
}
