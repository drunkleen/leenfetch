---
title: leenfetch
section: 1
header: Manual
footer: Leenfetch 1.0.2
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
  Show usage information and exit.
- `-l`, `--list-options`  
  List all configuration keys and accepted values.
- `-i`, `--init`  
  Create the default configuration file if it does not exist.
- `-r`, `--reinit`  
  Delete the configuration file and regenerate it with bundled defaults.
- `--config <PATH>`  
  Load configuration from a custom JSONC file.
- `--no-config`  
  Ignore files and run with built-in defaults.
- `--ascii_distro <DISTRO>`  
  Override the ASCII art distribution for this run (e.g., `arch`, `ubuntu`, `debian`).
- `--ascii_colors <COLORS>`  
  Override the ASCII palette (comma-separated indices such as `1,2,3,4,5,6,7` or `distro`).
- `--custom_ascii_path <PATH>`  
  Use a custom ASCII art file for this run.
- `--color-blocks <GLYPH>`  
  Change the glyph used for the color swatch (default: `●`).
- `--battery-display <MODE>`  
  Switch battery output between `off`, `bar`, `infobar`, and `barinfo`.
- `--disk-display <MODE>`  
  Select disk output (`info`, `percentage`, `infobar`, `barinfo`, `bar`).
- `--disk-subtitle <MODE>`  
  Choose disk labels (`name`, `dir`, `none`, `mount`).
- `--memory-unit <UNIT>`  
  Force memory units (`kib`, `mib`, `gib`).
- `--packages <MODE>`  
  Control package summaries (`off`, `on`, `tiny`).
- `--uptime <MODE>` / `--os-age <MODE>`  
  Pick shorthand (`full`, `tiny`, `seconds`).
- `--distro-display <MODE>`  
  Set OS detail level (e.g., `name`, `name_version`, `name_model_arch`).
- `--cpu-temp-unit <UNIT>`  
  Select CPU temperature units (`C`, `F`, or `off` to hide).
- `--only <LIST>`  
  Render only the listed modules (comma-separated).
- `--hide <LIST>`  
  Hide specific modules (comma-separated).
- Boolean toggles follow a paired pattern—use `--cpu-speed` / `--no-cpu-speed`, `--shell-path` / `--no-shell-path`, `--memory-percent` / `--no-memory-percent`, `--refresh-rate` / `--no-refresh-rate`, etc.


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
