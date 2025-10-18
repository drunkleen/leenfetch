# leenfetch Documentation

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Configuration](#configuration)
  - [config.jsonc Overview](#configjsonc-overview)
  - [flags section](#flags-section)
  - [toggles section](#toggles-section)
  - [layout section](#layout-section)
  - [Editing Workflow](#editing-workflow)
- [Usage](#usage)
- [Customization](#customization)
  - [Dynamic Logos via Piping](#dynamic-logos-via-piping)
- [Troubleshooting](#troubleshooting)
- [Advanced Tips](#advanced-tips)
- [Contributing](#contributing)
- [Support](#support)
- [License](#license)
- [Inspiration](#inspiration)

---

## Overview

**LeenFetch** is a fast, minimal, and highly customizable system information tool written in Rust. It is designed as a modern alternative to Neofetch, providing beautiful, colorized terminal output and deep configuration for power users.

---

## Features
- Blazing fast startup and execution
- Modular design: enable/disable any info block
- Fully customizable output layout and labels
- Smart defaults, but easily extendable
- Detects packages, shell, GPU, DE/WM, and more
- Custom ASCII art and color palette support
- Cross-platform: Linux and Windows (macOS planned)
- Simple, human-readable config files with inline documentation

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

---

## Configuration

On first run, LeenFetch creates a unified configuration file:

- **Linux:** `~/.config/leenfetch/config.jsonc`
- **Windows:** `C:\Users\<username>\AppData\Roaming\leenfetch\config.jsonc`

The file uses JSON with comments (JSONC), keeping inline explanations next to every option. It combines the previous trio of files into three logical sections.

---

### config.jsonc Overview

```jsonc
{
  "flags": { /* display & formatting options */ },
  "toggles": { /* show/hide info blocks */ },
  "layout": [ /* output order and labels */ ]
}
```

- **flags** — Controls display and formatting options for each block.
- **toggles** — Controls which information blocks are shown or hidden.
- **layout** — Controls the order and labels of blocks in the output.

You can edit everything in a single place and keep the helpful comments that ship with the defaults.

---

### flags section

Fine-tune how each block of information is displayed. Choose ASCII art, pick units, and decide how detailed each line should be.
- Choose which ASCII art and color palette to use
- Select how battery, disk, memory, and package info are shown
- Pick units, detail level, and formatting for each section
- Enable or disable features like CPU brand, temperature, shell version, etc.
- Accepts piped ASCII input for dynamic text or art (`echo "Hello" | leenfetch`)

```jsonc
// flags — Display and formatting options
{
  "flags": {
    // Select which distribution's ASCII art to display at the top.
    "ascii_distro": "auto",
    // How to show battery info.
    "battery_display": "barinfo",
    // Include shell version in the output.
    "shell_version": true
  }
}
```

#### All Options Explained
- **ascii_distro**: Which ASCII art to use. `auto`, `distro`, or a specific distro name.
- **ascii_colors**: Color palette. `distro` or a comma-separated list of color numbers.
- **custom_ascii_path**: Path to a custom ASCII art file. Empty for default.
- **battery_display**: How to show battery info. `off`, `bar`, `infobar`, `barinfo`.
- **color_blocks**: String used for color blocks (e.g., `███`, `###`).
- **cpu_brand**: Show CPU brand (true/false).
- **cpu_cores**: Show CPU core count (true/false).
- **cpu_frequency**: Show CPU frequency (true/false).
- **cpu_speed**: Show CPU speed (true/false).
- **cpu_temp**: Temperature unit: `"C"` or `"F"`.
- **cpu_show_temp**: Show CPU temperature (true/false).
- **de_version**: Show desktop environment version (true/false).
- **distro_display**: Detail level for OS info. `name`, `name_version`, `name_arch`, `name_model`, `name_model_version`, `name_model_arch`, `name_model_version_arch`.
- **disk_display**: Disk usage display: `info`, `percentage`, `infobar`, `barinfo`, `bar`.
- **disk_subtitle**: Disk label: `name`, `dir`, `none`, `mount`.
- **memory_percent**: Show memory as percent (true/false).
- **memory_unit**: Memory unit: `mib`, `gib`, `kib`.
- **package_managers**: Package info: `off`, `on`, `tiny`.
- **refresh_rate**: Show screen refresh rate (true/false).
- **shell_path**: Show full shell path (true/false).
- **shell_version**: Show shell version (true/false).
- **uptime_shorthand**: Uptime format: `full`, `tiny`, `seconds`.

---

### toggles section

Controls which blocks of information are shown in the output. Set each option to `true` to show that block, or `false` to hide it.

```jsonc
// toggles — Show/hide information blocks
{
  "toggles": {
    // Show the user@host title at the top of the output.
    "show_titles": true,
    // Show GPU information.
    "show_gpu": true,
    // Hide the currently playing song/media info.
    "show_song": false
  }
}
```

#### All Toggles Explained
- **show_titles**: Show user@host title
- **show_os**: Show base OS name
- **show_distro**: Show distro info
- **show_model**: Show hardware model
- **show_uptime**: Show system uptime
- **show_packages**: Show package info
- **show_shell**: Show shell info
- **show_wm**: Show window manager
- **show_de**: Show desktop environment
- **show_wm_theme**: Show WM theme
- **show_kernel**: Show kernel version
- **show_cpu**: Show CPU info
- **show_gpu**: Show GPU info
- **show_memory**: Show memory info
- **show_song**: Show currently playing song/media info
- **show_resolution**: Show display resolution
- **show_theme**: Show GTK/Qt theme
- **show_disks**: Show disk usage
- **show_battery**: Show battery info
- **show_terminal_colors**: Show terminal color palette

---

### layout section

Controls the order and labels of each block in the output. Rearrange, remove, or relabel any section to customize your layout.

```jsonc
// layout — Output order and labels
{
  "layout": [
    // The user@host title block (e.g., "snape@archbox").
    { "label": "Titles", "field": "titles" },
    // The distribution (distro) information (e.g., "Arch Linux").
    { "label": "Distro", "field": "distro" },
    // Terminal color palette preview.
    { "label": "", "field": "colors" }
  ]
}
```

#### Layout Fields
- `titles`
- `distro`
- `model`
- `kernel`
- `uptime`
- `packages`
- `shell`
- `wm`
- `de`
- `wm_theme`
- `cpu`
- `gpu`
- `memory`
- `disk`
- `resolution`
- `theme`
- `battery`
- `song`
- `colors`

---

### Editing Workflow

1. Edit `config.jsonc`.
2. Save the file.
3. Run `leenfetch` to see the updated output.

Changes are applied immediately—no restart required.

## Usage

Simply run:

```bash
leenfetch
```

The output will reflect your current configuration. Edit `config.jsonc` and re-run to see changes.

### Command-Line Options

- `leenfetch --list-options` — List all available config options
- `leenfetch --help` — Show help and usage

### Piped Input

LeenFetch can accept piped text or ASCII art via `stdin`. If data is piped into LeenFetch, that input will be displayed as the ASCII logo instead of the configured or auto-detected one.

**Examples:**

```bash
echo "Rustacean" | leenfetch
```

```bash
fortune | cowsay | leenfetch
```

This is great for fun dynamic banners or scriptable ASCII output.

---

## Customization

- **ASCII Art:** Use your own by setting `custom_ascii_path` in the `flags` section of `config.jsonc`.
- **Color Palette:** Set `ascii_colors` to a custom list.
- **Hide/Show Blocks:** Toggle entries in the `toggles` section.
- **Order/Labels:** Edit the `layout` array to rearrange or rename blocks.
- **Advanced:** Combine config changes for a unique look!

> If input is piped into leenfetch via `stdin`, the `ascii_distro` and `custom_ascii_path` settings are ignored, and the piped content is used as the ASCII logo.

### Dynamic Logos via Piping

Want to use real-time ASCII art or dynamic text as your logo? LeenFetch supports piped input!

**Examples:**

```bash
echo "Hack the Planet" | leenfetch
fortune | figlet | leenfetch
```

This lets you combine other CLI tools with LeenFetch for expressive, interactive output. No need to touch your config — just pipe text in!



---

## Troubleshooting

- If LeenFetch doesn't start, check your config files for typos or invalid values.
- Delete or rename a config file to regenerate it with defaults.
- For more help, open an issue on GitHub or check the discussions.

---

## Advanced Tips

- You can version-control your config files for easy sharing.
- Use symbolic links to share configs across systems.
- Explore the inline comments in `config.jsonc` for hidden features and advanced formatting.

---

## Contributing

1. Fork the repo
2. Create your branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'feat: add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Create a Pull Request

We welcome clean PRs and documented modules! ✨

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
