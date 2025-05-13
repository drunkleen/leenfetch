pub const CONFIG_PATH: &str = ".config/leenfetch/config.conf";

pub const DEFAULT_CONFIG: &str = r#"
# LeenFetch con"fig file
# https://github.com/drunkleen/leenfetch

# To use this config, copy it to ~/.config/leenfetch/config.conf
# For more info, see https://github.com/drunkleen/leenfetch


#layout="(
    [titles]
    ${c1}{username}${reset}@${c1}{hostname}${reset}
    [/titles]
    
    {underline}
    
    [distro]
    ${c1}Distro:${reset} {distro_index}
    [/distro]
    
    [model]
    ${c1}Host:${reset} {model_index}
    [/model]
    
    [kernel]
    ${c1}Kernel:${reset} {kernel_index}
    [/kernel]
    
    [uptime]
    ${c1}Uptime:${reset} {uptime_index}
    [/uptime]
    
    [packages]
    ${c1}Packages:${reset} {packages_index}
    [/packages]
    
    [shell]
    ${c1}Shell:${reset} {shell_index}
    [/shell]
    
    [wm]
    ${c1}WM:${reset} {wm_index}
    [/wm]
    
    [de]
    ${c1}DE:${reset} {de_index}
    [/de]
    
    [wm_theme]
    ${c1}WM Theme:${reset} {wm_theme_index}
    [/wm_theme]
    
    [cpu]
    ${c1}CPU:${reset} {cpu_index}
    [/cpu]
    
    [gpu]
    ${c1}GPU #{gpu_index}:${reset} {gpu}
    [/gpu]
    
    [memory]
    ${c1}Memory:${reset} {memory_index}
    [/memory]
    
    [disk]
    ${c1}Disk {disk_label}:${reset} {disk_index}
    [/disk]
    
    [resolution]
    ${c1}Resolution:${reset} {resolution_index}
    [/resolution]
    
    [theme]
    ${c1}Theme:${reset} {theme_index}
    [/theme]
    
    [battery]
    ${c1}Battery${reset} {battery}
    [/battery]
    
    [song]
    {song_index}
    [/song]
    
    {empty_line}
    
    [colors]
    {colors_index}
    [/colors]
)


# Output toggles
show_titles="on"
show_os="on"
show_distro="on"
show_model="on"
show_uptime="on"
show_packages="on"
show_shell="on"
show_wm="on"
show_de="on"
show_wm_theme="on"
show_kernel="on"
show_cpu="on"
show_gpu="on"
show_memory="on"
show_song="on"
show_resolution="on"
show_theme="on"
show_disks="on"
show_battery="on"
show_terminal_colors="on"

# Shorthands and flags
distro_display="name_model_arch"
uptime_shorthand="on"
memory_percent="on"
memory_unit="mib
package_managers="tiny"
shell_path="off"
shell_version="on"
cpu_brand="on"
cpu_speed="on"
cpu_cores="logical"
cpu_temp="C"
speed_shorthand="on"
refresh_rate="on"
gpu_type="all"
de_version="on"
disk_show="/"
disk_subtitle="dir"
disk_percent="on"
disk_display="barinfo"
battery_display="barinfo"

# Colors
color_blocks="███"

# Ascii Art
ascii_distro="auto"
ascii_colors="distro"
custom_ascii_path=""

"#;
