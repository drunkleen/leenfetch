# leenfetch — User Commands

## NAME
leenfetch — a fast, customizable system information tool (Neofetch clone)

## SYNOPSIS
```
leenfetch [OPTIONS]
```

## DESCRIPTION
**leenfetch** is a modern, fast, and highly configurable system information tool written in Rust. It displays system information alongside a distribution logo or custom ASCII art, similar to Neofetch, and is designed for both terminal enthusiasts and new users.

Configuration is handled via human-friendly RON files, with detailed comments to guide users through all available options.

## OPTIONS
- `-v`, `--version`  
  Print version information and exit.
- `-h`, `--help`  
  Show a help message and exit.
- `-l`, `--list-options`  
  List all available configuration options and their possible values.
- `-i`, `--init`  
  Create default configuration files in the user config directory if they do not exist.
- `-r`, `--reinit`  
  Delete existing configuration files and regenerate them with default values.
- `--ascii_distro <DISTRO>`  
  Override the ASCII art distribution for this run (e.g., "arch", "ubuntu", "debian").
- `--ascii_colors <COLORS>`  
  Override the ASCII art color palette for this run (comma-separated list, e.g., "1,2,3,4,5,6,7").
- `--custom_ascii_path <PATH>`  
  Use a custom ASCII art file for this run.

## CONFIGURATION
leenfetch uses RON files for configuration, typically located at:
```
$XDG_CONFIG_HOME/leenfetch/
~/.config/leenfetch/
```

The main configuration files are:
- **flags.ron**: Controls display options, such as which fields to show, formatting, and output style.
- **toggles.ron**: Enables or disables specific modules (e.g., show/hide GPU, battery, song info, etc.).
- **layout.ron**: Controls the order and grouping of displayed information.

Each config file is heavily commented to explain all available options and their effects.

## EXAMPLES
- `leenfetch`  
  Display system information using the current configuration.
- `leenfetch --ascii_distro debian`  
  Show system info with Debian ASCII art, regardless of detected distro.
- `leenfetch --init`  
  Create default config files if missing.
- `leenfetch --reinit`  
  Delete and regenerate all config files.

## ENVIRONMENT
- **XDG_CONFIG_HOME**: Specifies the base directory for user-specific configuration files. Defaults to `~/.config` if not set.

## FILES
- `~/.config/leenfetch/flags.ron`
- `~/.config/leenfetch/toggles.ron`
- `~/.config/leenfetch/layout.ron`

## BUGS
Please report bugs or feature requests at:  
https://github.com/drunkleen/leenfetch/issues

## AUTHOR
Written by DrunkLeen <snape@drunkleen.com>
