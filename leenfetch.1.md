---
title: leenfetch
section: 1
header: Manual
footer: Leenfetch 1.0.1
---

# leenfetch — User Commands

## NAME
leenfetch — a fast, customizable system information tool (Neofetch clone)

## SYNOPSIS
```
leenfetch [OPTIONS]
```

## DESCRIPTION
**leenfetch** is a modern, fast, and highly configurable system information tool written in Rust. It displays system information alongside a distribution logo or custom ASCII art, similar to Neofetch, and is designed for both terminal enthusiasts and new users.

Configuration lives in a single JSON-with-comments file (`config.jsonc`), with inline explanations for every option.

leenfetch can also accept **piped input** to render as ASCII art. This allows users to display custom text logos via standard input.

## OPTIONS
- `-v`, `--version`  
  Print version information and exit.
- `-h`, `--help`  
  Show a help message and exit.
- `-l`, `--list-options`  
  List all available configuration options and their possible values.
- `-i`, `--init`  
  Create the default configuration file in the user config directory if it does not exist.
- `-r`, `--reinit`  
  Delete the configuration file and regenerate it with default values.
- `--ascii_distro <DISTRO>`  
  Override the ASCII art distribution for this run (e.g., "arch", "ubuntu", "debian").
- `--ascii_colors <COLORS>`  
  Override the ASCII art color palette for this run (comma-separated list, e.g., "1,2,3,4,5,6,7").
- `--custom_ascii_path <PATH>`  
  Use a custom ASCII art file for this run.
- `terminal stdin`  
  Read ASCII logo content from standard input instead of using built-in or custom files.
  

## CONFIGURATION
leenfetch uses a single JSONC file for configuration, typically located at:
```
$XDG_CONFIG_HOME/leenfetch/
~/.config/leenfetch/
```

The configuration file is:
- **config.jsonc**: Includes `flags`, an ordered `modules` (`layout`) array, and an optional `logo` override. Use strings like `"break"` for spacers or objects with `type`/`key` to control the output. JSONC keeps comments intact so every option is documented in place.

Edit the file to adjust appearance, enabled modules, spacing, or output order.

## EXAMPLES
- `leenfetch`  
  Display system information using the current configuration.
- `leenfetch --ascii_distro debian`  
  Show system info with Debian ASCII art, regardless of detected distro.
- `leenfetch --init`  
  Create the default config file if missing.
- `leenfetch --reinit`  
  Delete and regenerate the config file.
- `echo "Rustacean" | leenfetch`  
  Display system info with a piped ASCII logo generated from the input text.
- `fortune | cowsay | leenfetch`  
  fortune generates a random quote.
  cowsay wraps it in a cow speech bubble (ASCII art).
  leenfetch takes piped input and displays it as an ASCII logo.


## ENVIRONMENT
- **XDG_CONFIG_HOME**: Specifies the base directory for user-specific configuration files. Defaults to `~/.config` if not set.

## FILE
- `~/.config/leenfetch/config.jsonc`

## WIKI
Please report bugs or feature requests at:  
https://github.com/drunkleen/leenfetch/wiki

## BUGS
Please report bugs or feature requests at:  
https://github.com/drunkleen/leenfetch/issues

## AUTHOR
Written by DrunkLeen <snape@drunkleen.com>
