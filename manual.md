# LeenFetch Documentation

## Table of Contents
- [LeenFetch Documentation](#leenfetch-documentation)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Quick Start](#quick-start)
  - [Features](#features)
  - [Architecture](#architecture)
  - [Installation](#installation)
    - [From crates.io (Recommended)](#from-cratesio-recommended)
    - [Build from Source](#build-from-source)
    - [Binary Releases](#binary-releases)
    - [Shell Completions](#shell-completions)
  - [Configuration](#configuration)
    - [Configuration Overview](#configuration-overview)
    - [flags section](#flags-section)
    - [modules array](#modules-array)
    - [Reloading Configs](#reloading-configs)
  - [Information Blocks](#information-blocks)
    - [ASCII Art Block](#ascii-art-block)
    - [Titles](#titles)
    - [Operating System \& Distribution](#operating-system--distribution)
    - [Hardware Model](#hardware-model)
    - [OS Age](#os-age)
    - [Uptime](#uptime)
    - [Package Managers](#package-managers)
    - [Shell](#shell)
    - [Window Manager](#window-manager)
    - [Desktop Environment](#desktop-environment)
    - [Kernel](#kernel)
    - [CPU](#cpu)
    - [GPU](#gpu)
    - [Memory](#memory)
    - [Disk Usage](#disk-usage)
    - [Resolution](#resolution)
    - [Theme](#theme)
    - [Battery](#battery)
    - [Now Playing](#now-playing)
    - [Terminal Color Swatch](#terminal-color-swatch)
  - [ASCII Art \& Colors](#ascii-art--colors)
  - [Usage](#usage)
    - [Interactive Examples](#interactive-examples)
    - [Command-Line Options](#command-line-options)
    - [Exit Codes](#exit-codes)
  - [Customization](#customization)
    - [Profiles \& Version Control](#profiles--version-control)
    - [Theming Recipes](#theming-recipes)
  - [Scripting \& Automation](#scripting--automation)
  - [Performance \& Diagnostics](#performance--diagnostics)
  - [Troubleshooting](#troubleshooting)
  - [Frequently Asked Questions](#frequently-asked-questions)
  - [Contributing](#contributing)
  - [Support](#support)
  - [License](#license)
  - [Inspiration](#inspiration)
  - [Appendix](#appendix)
    - [Default Modules Example](#default-modules-example)
    - [Color Index Reference](#color-index-reference)
    - [Glossary](#glossary)

---

## Overview

**LeenFetch** is a fast, minimal, and highly customizable system information tool written in Rust. It is designed as a modern alternative to Neofetch, providing beautiful, colorized terminal output and deep configuration for power users. Every element that LeenFetch prints can be reordered and restyled without touching the source code. Users who love to tinker get full control, while sane defaults make the first run delightful.

---

## Quick Start

1. Install LeenFetch via `cargo install leenfetch`.
2. Run `leenfetch` to generate the default configuration file.
3. Open `config.jsonc` in `~/.config/leenfetch/` (Linux) or `C:\Users\<you>\AppData\Roaming\leenfetch\` (Windows).
4. Edit the `flags` or `modules` sections, then re-run `leenfetch` to see your changes instantly.

---

## Features
- Blazing fast startup thanks to Rust and minimal I/O.
- Modular design: enable or disable any information block independently.
- Fully customizable output layout, labels, and color palette.
- Rich ASCII art support with distro detection and custom artwork.
- Tracks packages, shell, GPU, DE/WM, theme, and more per platform.
- Commented configuration file shipped in human-readable JSONC format.
- Works on Linux and Windows with feature-aware fallbacks (macOS in progress).
- Plays nicely in scripts, status bars, and login shells.

---

## Architecture

LeenFetch separates data gathering from rendering:
- **Collectors** probe the system for raw information (CPU, GPU, packages, etc.).
- **Flags** control how the collected data is transformed or formatted.
- **Layout** orders and labels the final output, stitching in ASCII art and colors.

Collectors rely on platform-specific backends where necessary. When a value cannot be fetched on the current platform, LeenFetch falls back to safe defaults or hides the block entirely if configured to do so. This design keeps runtime fast and makes new modules easy to add.

---

## Installation

### From crates.io (Recommended)

Make sure you have [Rust & Cargo](https://rustup.rs/) installed:

```bash
cargo install leenfetch
```

Then run:

```bash
leenfetch
```

If you have issues with `PATH`, add Cargo's bin directory:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Build from Source

```bash
git clone https://github.com/drunkleen/leenfetch.git
cd leenfetch
cargo build --release
```

Add to PATH:

```bash
cp target/release/leenfetch ~/.local/bin/
```

### Binary Releases

If prebuilt archives are provided on the releases page:

1. Download the archive for your architecture.
2. Extract the binary into a directory that is in your `PATH`.
3. Optionally rename the binary to `leenfetch` for consistency.

### Shell Completions

LeenFetch does not ship completions by default. You can still leverage your shell's history or aliasing to streamline usage:
- Add `alias lf='leenfetch'` to your shell rc file.
- Combine with other tools, for example `alias lff='leenfetch --list-options | fzf'`.

---

## Configuration

LeenFetch stores its settings in a single JSON-with-comments file so you get structured data and helpful inline guidance.

- **Linux:** `~/.config/leenfetch/config.jsonc`
- **Windows:** `C:\Users\<username>\AppData\Roaming\leenfetch\config.jsonc`

### Configuration Overview

| Section | Purpose | Typical Changes |
| ------- | ------- | ---------------- |
| `flags` | Formatting, measurement units, ASCII art preferences | Switch palettes, change CPU display detail, adjust disk layout |
| `modules` (`layout`) | Ordering, separators, and custom rows | Reorder modules, add headings, insert breaks |

The file is authored in JSONC, which is JSON plus `//` comments. Every default entry includes a brief explanation right in the file.

### flags section

The `flags` object tunes how each block of information is displayed. Each key has a descriptive comment explaining valid values. Highlights include:

- **ascii_distro**: `"auto"` detects the current distro or accepts a specific name (e.g., `"arch"`, `"ubuntu"`).
- **ascii_colors**: Use `"distro"` for defaults or provide a comma-separated list such as `"1,2,4,5"`.
- **custom_ascii_path**: Absolute or relative path to custom ASCII art. Leave empty for built-ins.
- **battery_display**: Choose `off`, `bar`, `infobar`, or `barinfo` to control battery appearance.
- **color_blocks**: Characters used for the color bar (e.g., `███` or `###`).
- **cpu_brand / cpu_cores / cpu_frequency / cpu_speed**: Boolean flags to display respective CPU details.
- **cpu_temp**: Choose `"C"` or `"F"` for temperature units.
- **cpu_show_temp**: Toggle temperature visibility without losing other CPU info.
- **de_version**: Show or hide desktop environment version numbers.
- **distro_display**: Granularity of OS info. Options include `name`, `name_version`, `name_arch`, `name_model`, `name_model_version`, `name_model_arch`, `name_model_version_arch`.
- **disk_display**: Render disks as `info`, `percentage`, `infobar`, `barinfo`, or `bar`.
- **disk_subtitle**: Subtitle strategy per disk (`name`, `dir`, `none`, `mount`).
- **memory_percent**: Show memory as a percentage alongside absolute values.
- **memory_unit**: Select `kib`, `mib`, or `gib` to standardize units.
- **package_managers**: Display `off`, `on`, or `tiny` counts depending on how much detail you want.
- **refresh_rate**: Enable detection of display refresh rates (supported on select setups).
- **shell_path** / **shell_version**: Control whether LeenFetch prints the full shell path or version.
- **uptime_shorthand**: Choose `full`, `tiny`, or `seconds` based on your preferred format.
- **os_age_shorthand**: Match `uptime_shorthand` but for the dedicated OS install age module.

Most options are purely visual. If a certain value is unsupported on your system, LeenFetch gracefully falls back to the closest available presentation.

```jsonc
{
  "flags": {
    "ascii_distro": "auto",
    "battery_display": "barinfo",
    "shell_version": true
  }
}
```


```

### logo section (optional)

The `logo` object controls ASCII art. Use it to point to a file (`source`), adjust spacing (`padding`), or override palette indices (`color`).

```jsonc
{
  "logo": {
    "type": "file",
    "source": "~/.config/leenfetch/branding/about.txt",
    "padding": { "top": 2, "right": 6 }
  }
}
```

### modules array

The `modules` array governs ordering, headings, and custom rows. Entries can be literal strings (use `"break"` for a blank spacer line) or objects describing a module.

```jsonc
{
  "modules": [
    "break",
    { "type": "custom", "format": "== System ==" },
    { "type": "titles", "key": "User" },
    { "type": "distro", "key": "Distro" },
    { "type": "cpu", "key": "CPU" },
    { "type": "colors", "key": "" }
  ]
}
```

- **type**: Matches the collectors listed below under [Information Blocks](#information-blocks).
- **key**: Optional label printed before the value. Leave empty (`""`) for no label.
- **format**: When `type` is `custom`, print the supplied string as-is.
- Duplicate entries are allowed if you want to emphasize a field in multiple places.
- Remove entries to hide a block entirely, or insert `"break"` for spacing.

### Reloading Configs

LeenFetch reads configuration at startup. When you update `config.jsonc`:
1. Save your changes.
2. Re-run `leenfetch` to apply them.
3. If the output looks wrong, check the terminal for parse errors. LeenFetch prints helpful messages indicating which line failed to parse.

---

## Information Blocks

Each entry in the `modules` array triggers a collector. The summaries below highlight the module name, which flags influence it, and any notable behavior so you can tailor layouts quickly.

### ASCII Art Block
- **Module entry**: Implicit (renders alongside the `titles` module)
- **Flags**: `ascii_distro`, `ascii_colors`, `custom_ascii_path`
- **Data**: Distro-specific or custom ASCII artwork.
- **Notes**: Pair with `titles` for the classic banner look, or pipe input to override art on the fly.

### Titles
- **Module entry**: `titles`
- **Flags**: Inherits ASCII palette; no dedicated flags.
- **Data**: Current user and hostname.
- **Notes**: Commonly the first info row, aligning the banner with text output.

### Operating System & Distribution
- **Module entry**: `distro`
- **Flags**: `distro_display`
- **Data**: Distro name, version, architecture, and optionally hardware model depending on `distro_display`.
- **Notes**: Windows reports edition and build when available.

### Hardware Model
- **Module entry**: `model`
- **Flags**: None directly; `distro_display` can embed the same info in the distro row.
- **Data**: Motherboard, laptop chassis, or device identifier.
- **Notes**: Useful for multi-device screenshots or inventory output.

### OS Age
- **Module entry**: `os_age`
- **Flags**: `os_age_shorthand`
- **Data**: Time since the current OS install or root filesystem creation.
- **Notes**: Combine with `uptime` to show both install age and current session length.

### Uptime
- **Module entry**: `uptime`
- **Flags**: `uptime_shorthand`
- **Data**: Time since the last boot using the best available system clock.
- **Notes**: `tiny` formatting condenses the value for compact layouts.

### Package Managers
- **Module entry**: `packages`
- **Flags**: `package_managers`
- **Data**: Package counts aggregated from supported managers on your platform. Unsupported managers are skipped silently.
- **Notes**: `tiny` mode displays a short summary such as `pacman(1200)`.

### Shell
- **Module entry**: `shell`
- **Flags**: `shell_path`, `shell_version`
- **Data**: Active shell process. On Windows, respects PowerShell, cmd, and third-party shells.
- **Notes**: Disable both flags for the cleanest single-word shell output.

### Window Manager
- **Module entry**: `wm`
- **Flags**: None in the `flags` section.
- **Data**: Currently running window manager, including Wayland compositors where detected.
- **Notes**: Requires a graphical session; falls back gracefully otherwise.

### Desktop Environment
- **Module entry**: `de`
- **Flags**: `de_version`
- **Data**: Desktop environment name and optional version.
- **Notes**: Set `de_version=false` to avoid noise when the version cannot be detected.

### Kernel
- **Module entry**: `kernel`
- **Flags**: None in the `flags` section.
- **Data**: Kernel or OS build string.
- **Notes**: On Windows, shows the NT kernel version.

### CPU
- **Module entry**: `cpu`
- **Flags**: `cpu_brand`, `cpu_cores`, `cpu_frequency`, `cpu_speed`, `cpu_temp`, `cpu_show_temp`
- **Data**: CPU vendor, model, clock speeds, core counts, and optional temperature.
- **Notes**: Temperature collection depends on platform sensors; disable `cpu_show_temp` if sensors are unreliable.

### GPU
- **Module entry**: `gpu`
- **Flags**: None in the `flags` section.
- **Data**: Active graphics adapter(s).
- **Notes**: Multi-GPU systems are collapsed or listed individually depending on platform.

### Memory
- **Module entry**: `memory`
- **Flags**: `memory_percent`, `memory_unit`
- **Data**: RAM usage, optionally with percentage.
- **Notes**: `memory_unit` enforces consistent units across outputs.

### Disk Usage
- **Module entry**: `disk`
- **Flags**: `disk_display`, `disk_subtitle`
- **Data**: Mounted filesystem usage.
- **Notes**: `infobar` and `barinfo` draw textual bars for a quick overview.

### Resolution
- **Module entry**: `resolution`
- **Flags**: `refresh_rate`
- **Data**: Connected monitor resolutions, optionally including refresh rate when supported.
- **Notes**: Ideal for showcasing multi-monitor setups.

### Theme
- **Module entry**: `theme`
- **Flags**: None in the `flags` section.
- **Data**: GTK/Qt theme names when available.
- **Notes**: Falls back silently if the theme cannot be detected.

### Battery
- **Module entry**: `battery`
- **Flags**: `battery_display`
- **Data**: Battery charge, status, and optionally bars.
- **Notes**: Desktop users can remove the module entirely to avoid empty placeholders.

### Now Playing
- **Module entry**: `song`
- **Flags**: None in the `flags` section.
- **Data**: Integrates with common media players via MPRIS or platform APIs.
- **Notes**: Appears only when playback is active and a song title is available.

### Terminal Color Swatch
- **Module entry**: `colors`
- **Flags**: `color_blocks`, `ascii_colors`
- **Data**: Color palette preview showing up to 16 colors.
- **Notes**: Customize `color_blocks` to change the glyph used in the swatch.

---

## ASCII Art & Colors

ASCII art is central to LeenFetch's personality. You can:
- Set `ascii_distro` to `auto` for automatic distro detection.
- Pick any built-in art by name (e.g., `"arch"`, `"ubuntu"`, `"fedora"`, `"windows"`).
- Supply a custom art file via `custom_ascii_path`.
- Override palette with `ascii_colors`. Use terminal color indexes (`0`-`7` for standard, `8`-`15` for bright).

Tips:
- Keep custom art within 30-40 characters wide for the best fit.
- Mix and match ASCII art with a minimalist data layout for wallpapers or screenshots.
- Pair ASCII palettes with the `color_blocks` glyph to maintain a unified look.

---

## Usage

Run LeenFetch directly from the terminal. It reads configs, gathers data, and prints the decorated output in one pass.

### Interactive Examples

```bash
# Show default output
leenfetch

# Dump all available configuration keys (with inline docs)
leenfetch --list-options

# Pipe into a file for sharing
leenfetch > leenfetch.txt
```

Combine with other tools:

- `watch -n 5 leenfetch` monitors uptime or battery over time.
- `tmux display-message "$(leenfetch | head -n 12)"` shows a compact summary inside tmux.
- `leenfetch` can be run inside login scripts to greet you with fresh system info.

### Command-Line Options

- `leenfetch --list-options` — Prints every configurable key along with short documentation.
- `leenfetch --help` — Standard usage information.

LeenFetch intentionally keeps the CLI small; the rich customization story lives inside `config.jsonc`.

### Exit Codes

- `0` — Execution succeeded, output was printed.
- `1` — General failure (configuration parsing, IO error, unsupported platform call).

---

## Customization

LeenFetch thrives on personalization. Since configs are regular text files, you can use your favorite editor, track changes in version control, and share profiles with friends.

### Profiles & Version Control

- Store multiple config sets under version control, e.g., `~/.config/leenfetch/profiles/`.
- Symlink or copy the desired `config.jsonc` before running LeenFetch.
- For remote machines, keep configs in a dotfiles repository and bootstrap them with shell scripts.

### Theming Recipes

Ideas to get you started:
- **Minimal**: Disable ASCII art by piping input or using an empty custom ASCII, and keep only titles and distro within the `modules` array.
- **Screenshots**: Enable ASCII art, color swatches, CPU, GPU, resolution, and theme. Stick to `memory_percent=false` for clean numbers.
- **Monitoring**: Enable uptime, disks with `barinfo`, memory percent, and battery infobar.
- **Music Showcase**: Add the `song` module with a custom `key` like `Now Playing`, and keep color blocks bright.

---

## Scripting & Automation

LeenFetch plays nicely with automation:
- Use it in login shells to verify machine identity.
- Embed the output in status bars by capturing `leenfetch` output and slicing the lines you need.
- Combine with cron or systemd timers to snapshot system info periodically.
- Pair with screenshot tools to publish desktop showcase posts with consistent styling.

When scripting, trim the `modules` array to only the collectors you need to reduce noise and speed up execution even further.

---

## Performance & Diagnostics

LeenFetch is optimized for speed, but you can fine-tune it:
- Disable heavy collectors (package counts, now playing) if you prefer bare minimum output.
- Keep ASCII art short to reduce terminal scrollback.
- Monitor configuration parse errors: LeenFetch prints descriptive messages for invalid JSONC syntax, including line numbers.
- When debugging, temporarily set `package_managers="off"` or remove the `colors` module to isolate problematic sections.

---

## Troubleshooting

- **Tool fails to start**: Check terminal output for parse errors. The message includes file paths and line numbers.
- **Missing blocks**: Make sure the module appears in `modules` and any related flag (like `package_managers`) isn't disabling the data.
- **Wrong ASCII art**: Verify `ascii_distro` is spelled correctly or set it to `"auto"`.
- **Incorrect colors**: Terminal themes can override ANSI colors; test in a default theme to confirm.
- **Package count is zero**: The manager might not be supported on your platform yet. Contributions are welcome.
- **Media info missing**: Make sure your player exposes MPRIS (Linux) or the relevant Windows API. Not every application provides hooks.
- **Battery not detected**: Desktop systems without batteries should disable the block. For laptops, ensure the OS power subsystem is accessible.
- **Windows path issues**: Run the terminal as the same user that installed LeenFetch so the config directory resolves correctly.

Deleting any config file forces LeenFetch to regenerate it on the next run, restoring defaults without manual editing.

---

## Frequently Asked Questions

- **Does LeenFetch support macOS?** Not yet. macOS support is on the roadmap; community feedback speeds it up.
- **Can I show multiple disks?** Yes. `disk_display` applies to every detected mount. Customize subtitles via `disk_subtitle`.
- **How do I highlight GPU vendors?** Adjust the `key` in the `modules` array or use the ASCII art colors to accent hardware lines.
- **Is there a way to localize labels?** Absolutely. Set the `key` field in the `modules` array to any text you like (ASCII recommended).
- **Why is temperature missing?** Sensors may require elevated permissions or additional packages. Toggle `cpu_show_temp` off if temperatures are unavailable.

---

## Contributing

1. Fork the repo.
2. Create a feature branch: `git checkout -b feature/my-feature`.
3. Make your changes. Keep modules documented and configurations commented.
4. Run tests or `cargo fmt`/`cargo clippy` if applicable.
5. Commit with a descriptive message: `git commit -m 'feat: add my feature'`.
6. Push to your fork and open a Pull Request.

We welcome clean PRs, detailed documentation, and new collectors. Please open an issue before adding large features to coordinate design.

---

## Support

If you find LeenFetch valuable, consider supporting development:
- [PayPal](https://www.paypal.com/paypalme/RDarvishifar)
- **BTC:** `bc1qsmvxpn79g6wkel3w67k37r9nvzm5jnggeltxl6`
- **ETH/BNB/MATIC:** `0x8613aD01910d17Bc922D95cf16Dc233B92cd32d6`
- **USDT/TRON:** `TGNru3vuDfPh5zBJ31DKzcVVvFgfMK9J48`
- **DOGE:** `D8U25FjxdxdQ7pEH37cMSw8HXBdY1qZ7n3`

---

## License

[MIT](./LICENSE) License © [DrunkLeen](https://github.com/drunkleen)

---

## Inspiration

- [Neofetch](https://github.com/dylanaraps/neofetch)
- [Fastfetch](https://github.com/fastfetch-cli/fastfetch)
- [Rust](https://www.rust-lang.org/) — the foundation of LeenFetch

---

## Appendix

### Default Modules Example

Below is a trimmed example of the default `modules` section. Your generated file includes inline comments for every entry.

```jsonc
{
  "modules": [
    "break",
    { "type": "custom", "format": "== System ==" },
    { "type": "titles", "key": "User" },
    { "type": "distro", "key": "Distro" },
    { "type": "kernel", "key": "Kernel" },
    { "type": "uptime", "key": "Uptime" },
    { "type": "packages", "key": "Packages" },
    { "type": "shell", "key": "Shell" },
    { "type": "cpu", "key": "CPU" },
    { "type": "gpu", "key": "GPU" },
    { "type": "memory", "key": "Memory" },
    { "type": "disk", "key": "Disk" },
    { "type": "battery", "key": "Battery" },
    { "type": "colors", "key": "" }
  ]
}
```

### Color Index Reference

ANSI color indexes used in `ascii_colors`:
- `0` Black
- `1` Red
- `2` Green
- `3` Yellow
- `4` Blue
- `5` Magenta
- `6` Cyan
- `7` White
- `8` Bright Black (Gray)
- `9` Bright Red
- `10` Bright Green
- `11` Bright Yellow
- `12` Bright Blue
- `13` Bright Magenta
- `14` Bright Cyan
- `15` Bright White

### Glossary

- **Collector**: Module that queries the system for data (CPU info, disk stats, etc.).
- **JSONC**: JSON with `//` comments, the format used for LeenFetch configuration.
- **Block**: A single line or section in the LeenFetch output (CPU, Memory, etc.).
- **Palette**: The set of colors used to render ASCII art and color swatches.
- **Module entry**: A string or object in the `modules` array that controls which collector runs and how it is labeled.
