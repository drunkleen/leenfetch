pub fn list_options() {
    println!(
        r#"
🔧 leenfetch Config Options

──────────────────────────── Display Settings ────────────────────────────
  title_fqdn         = on | off        Use full hostname
  underline          = on | off        Show underline under title
  separator          = string          Separator for key-value (e.g. :, ->)

──────────────────────────── Output Toggles ─────────────────────────────
  show_ascii         = on | off        Display ASCII art
  show_titles        = on | off        Show user@host title
  show_os            = on | off        Show OS
  show_distro        = on | off        Show Distro
  show_model         = on | off        Show Host/Model
  show_uptime        = on | off        Show system uptime
  show_packages      = on | off        Show installed packages
  show_shell         = on | off        Show current shell
  show_wm            = on | off        Show Window Manager
  show_de            = on | off        Show Desktop Environment
  show_wm_theme      = on | off        Show WM Theme
  show_kernel        = on | off        Show Kernel
  show_cpu           = on | off        Show CPU info
  show_gpu           = on | off        Show GPU(s)
  show_memory        = on | off        Show memory usage
  show_resolution    = on | off        Show display resolution
  show_theme         = on | off        Show GTK/Theme
  show_disks         = on | off        Show disk usage
  show_battery       = on | off        Show battery status
  show_song          = on | off        Show playing song (media)
  show_terminal_colors = on | off      Show terminal color palette

──────────────────────────── CPU Settings ───────────────────────────────
  cpu_brand          = on | off        Show CPU brand
  cpu_speed          = on | off        Show CPU frequency
  cpu_cores          = logical | physical | off
  cpu_temp           = C | F | off     Temperature unit
  speed_shorthand    = on | off        Use shorthand for MHz/GHz

─────────────────────────── Memory Settings ─────────────────────────────
  memory_unit        = kib | mib | gib Display units
  memory_percent     = on | off        Show memory as %

────────────────────────── Package Managers ─────────────────────────────
  package_managers   = on | off | tiny Show list of managers

──────────────────────────── Shell Info ─────────────────────────────────
  shell_path         = on | off        Show full shell path
  shell_version      = on | off        Show shell version

────────────────────────── Distro / Uptime ──────────────────────────────
  distro_shorthand   = on | tiny | off Distro name format
  uptime_shorthand   = on | tiny | off Uptime format

────────────────────────────── Disk Info ────────────────────────────────
  disk_display       = bar | barinfo | infobar | percentage | info
  disk_subtitle      = name | dir | mount | none
  disk_show          = string (path)   Show specific mount

──────────────────────────── Battery Info ───────────────────────────────
  battery_display    = bar | barinfo | infobar | off

───────────────────────────── Custom Paths ──────────────────────────────
  ascii_path         = path            Custom path to ASCII file

To update config:
  Edit: ~/.config/leenfetch/config.conf
"#
    );
}
