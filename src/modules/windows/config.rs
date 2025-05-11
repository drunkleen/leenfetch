pub const CONFIG_PATH: &str = ".config\\leenfetch\\config.conf";

pub const DEFAULT_CONFIG: &str = r#"
# LeenFetch config file
# https://github.com/drunkleen/leenfetch

# To use this config, copy it to ~/.config/leenfetch/config.conf
# For more info, see https://github.com/drunkleen/leenfetch

# 🎨 Colors
# ____________________
# 🖍️ Foreground Colors
# | Token        | Color        |
# | ------------ | ------------ |
# | `${fg.c1}`   | Black        |
# | `${fg.c2}`   | Red          |
# | `${fg.c3}`   | Green        |
# | `${fg.c4}`   | Yellow       |
# | `${fg.c5}`   | Blue         |
# | `${fg.c6}`   | Magenta      |
# | `${fg.c7}`   | Cyan         |
# | `${fg.c8}`   | White        |
# 🅱️ Bold Foreground Colors
# | ------------ | ------------ |
# | `${bold.c1}` | Bold Black   |
# | `${bold.c2}` | Bold Red     |
# | `${bold.c3}` | Bold Green   |
# | `${bold.c4}` | Bold Yellow  |
# | `${bold.c5}` | Bold Blue    |
# | `${bold.c6}` | Bold Magenta |
# | `${bold.c7}` | Bold Cyan    |
# | `${bold.c8}` | Bold White   |
# 🖼️ Background Colors
# | ------------ | ------------ |
# | `${bg.c1}`   | Black BG     |
# | `${bg.c2}`   | Red BG       |
# | `${bg.c3}`   | Green BG     |
# | `${bg.c4}`   | Yellow BG    |
# | `${bg.c5}`   | Blue BG      |
# | `${bg.c6}`   | Magenta BG   |
# | `${bg.c7}`   | Cyan BG      |
# | `${bg.c8}`   | White BG     |
# 🔁 Reset Colors
# Use `${reset}` to end a color block and return to default.


#layout="""
[titles]
${bold.c5}{username}${fg.c8}@${bold.c5}{hostname}${fg.c8}
[/titles]

{underline}

[distro]
${bold.c5}OS:${reset} {distro_index}
[/distro]

[model]
${bold.c5}Host:${reset} {model_index}
[/model]

[kernel]
${bold.c5}Kernel:${reset} {kernel_index}
[/kernel]

[uptime]
${bold.c5}Uptime:${reset} {uptime_index}
[/uptime]

[packages]
${bold.c5}Packages:${reset} {packages_index}
[/packages]

[shell]
${bold.c5}Shell:${reset} {shell_index}
[/shell]

[wm]
${bold.c5}WM:${reset} {wm_index}
[/wm]

[de]
${bold.c5}DE:${reset} {de_index}
[/de]

[wm_theme]
${bold.c5}WM Theme:${reset} {wm_theme_index}
[/wm_theme]

[cpu]
${bold.c5}CPU:${reset} {cpu_index}
[/cpu]

[gpu]
${bold.c5}GPU #{gpu_index}:${reset} {gpu}
[/gpu]

[memory]
${bold.c5}Memory:${reset} {memory_index}
[/memory]

[disk]
${bold.c5}Disk:${reset} {disk_label} => {disk_index}
[/disk]

[resolution]
${bold.c5}Resolution:${reset} {resolution_index}
[/resolution]

[theme]
${bold.c5}Theme:${reset} {theme_index}
[/theme]

[battery]
${bold.c5}Battery${reset} {battery}
[/battery]

[song]
{song_index}
[/song]

{empty_line}

[colors]
{colors_index}
[/colors]
"""


# Display settings
ascii_size=small

# Output toggles
show_ascii=on
show_titles=on
show_os=on
show_distro=on
show_model=on
show_uptime=on
show_packages=on
show_shell=on
show_wm=on
show_de=on
show_wm_theme=on
show_kernel=on
show_cpu=on
show_gpu=on
show_memory=on
show_song=on
show_resolution=on
show_theme=on
show_disks=on
show_battery=on
show_terminal_colors=on

# Shorthands and flags
distro_display=name_model_version_arch
uptime_shorthand=on
memory_percent=on
memory_unit=mib
package_managers=tiny
shell_path=off
shell_version=on
cpu_brand=on
cpu_speed=on
cpu_cores=logical
cpu_temp=C
speed_shorthand=on
refresh_rate=on
gpu_type=all
de_version=on
disk_show=/
disk_subtitle=dir
disk_percent=on
disk_display=barinfo
battery_display=barinfo

# Colors
color_blocks=██

# Custom paths
ascii_path="#;
