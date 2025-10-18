# 🧠 LeenFetch

[![Crates.io](https://img.shields.io/crates/v/leenfetch)](https://crates.io/crates/leenfetch)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build and Release](https://github.com/drunkleen/leenfetch/actions/workflows/release.yml/badge.svg)](https://github.com/drunkleen/leenfetch/actions/workflows/release.yml)



<!-- > 🌐 Available in: [English](#) | [فارسی](./readme/README-fa.md) | [Русский](./readme/README-ru.md) | [中文](./readme/README-zh.md)
**** -->


##### A fast, minimal, and customizable system information tool built in Rust — your alternative to Neofetch, for real power users.


> ⚠️ **LeenFetch is under active development. Expect bugs and improvements regularly!**
>
> Want to help? Contributions, testing, and feedback are always welcome!



## 💬 Feedback & Issues

Found a bug? Got a feature idea?  
Head over to the [issues](https://github.com/drunkleen/leenfetch/issues) or join the [discussions](https://github.com/drunkleen/leenfetch/discussions) page!


## 📚 Table of Contents

- [🧠 LeenFetch](#-leenfetch)
        - [A fast, minimal, and customizable system information tool built in Rust — your alternative to Neofetch, for real power users.](#a-fast-minimal-and-customizable-system-information-tool-built-in-rust--your-alternative-to-neofetch-for-real-power-users)
  - [💬 Feedback \& Issues](#-feedback--issues)
  - [📚 Table of Contents](#-table-of-contents)
  - [📸 Screenshots](#-screenshots)
  - [🚀 Features](#-features)
  - [▨ Packaging Status](#-packaging-status)
  - [📦 Installation](#-installation)
    - [Install on Arch Linux (via AUR)](#install-on-arch-linux-via-aur)
    - [Install on Debian / Ubuntu (via .deb)](#install-on-debian--ubuntu-via-deb)
    - [Install on Fedora / RHEL (via .rpm)](#install-on-fedora--rhel-via-rpm)
    - [Install on Windows (via .zip)](#install-on-windows-via-zip)
    - [Install from crates.io](#install-from-cratesio)
    - [🛠️ Manual Installation (Build from Source)](#️-manual-installation-build-from-source)
  - [📥 Using Piped Input](#-using-piped-input)
  - [⚙️ Configuration](#️-configuration)
    - [flags section](#flags-section)
    - [toggles section](#toggles-section)
    - [modules array](#modules-array)
    - [How to Edit](#how-to-edit)
  - [🎯 Roadmap](#-roadmap)
  - [🤝 Contributing](#-contributing)
  - [☕ Support LeenFetch](#-support-leenfetch)
  - [📄 License](#-license)
  - [💡 Inspiration](#-inspiration)


## 📸 Screenshots

<img src="./assets/TokyoNight.png" width="32%" /><img src="./assets/SandStorm.png" width="32%" /><img src="./assets/Aura.png" width="32%" />

<img src="./assets/debian.jpg" width="50%" /><img src="./assets/windows.jpg" width="50%" />
<!-- <img src="./assets/ubuntu.jpg" width="50%" /><img src="./assets/windows10.jpg" width="50%" /> -->
<img src="./assets/cowsay.jpg" width="50%" /><img src="./assets/custom.jpg" width="50%" />


## 🚀 Features

- ⚡ Blazing fast startup thanks to Rust
- 🎨 Customizable output layout with colorized terminal output
- 🧩 Modular design — enable or disable components via config
- 💾 Smart defaults but easily extendable
- 📦 Detects installed packages, shell, GPU, DE/WM, and more
- 🖼️ Custom ASCII art support and override via config
- 🎨 Supports theme-based color profiles (`ascii_colors=distro`, etc.)
- 🔌 Simple config files: `~/.config/leenfetch/*`
- 🧵 Accepts piped ASCII input — use `fortune | cowsay | leenfetch` for dynamic text logos


## ▨ Packaging Status

[![Repository status](https://repology.org/badge/vertical-allrepos/leenfetch.svg)](https://repology.org/project/leenfetch/versions)


## 📦 Installation


### Install on Arch Linux (via AUR)

If you're on Arch Linux or an Arch-based distribution (like Manjaro), you can install LeenFetch from the AUR using an AUR helper like [`yay`](https://github.com/Jguer/yay):

```bash
yay -S leenfetch
```
or

```bash
git clone https://aur.archlinux.org/leenfetch.git
cd leenfetch
makepkg -si
```



### Install on Debian / Ubuntu (via .deb)

If you're on Debian, Ubuntu, or a Debian-based distribution, you can download and install the `.deb` package from the [GitHub Releases](https://github.com/drunkleen/leenfetch/releases):

- AMD64 (x86_64)
```bash
wget https://github.com/drunkleen/leenfetch/releases/download/v0.2.1/leenfetch-v0.2.1-debian-x86_64.deb
sudo dpkg -i leenfetch-*.deb
```


- AArch64 (ARM64)
```bash
wget https://github.com/drunkleen/leenfetch/releases/download/v0.2.1/leenfetch-v0.2.1-debian-aarch64.deb
sudo dpkg -i leenfetch-*.deb
```

---

### Install on Fedora / RHEL (via .rpm)

If you're using Fedora, RHEL, or another RPM-based distro, you can install LeenFetch using the `.rpm` file from [GitHub Releases](https://github.com/drunkleen/leenfetch/releases):

- AMD64 (x86_64)
```bash
wget https://github.com/drunkleen/leenfetch/releases/download/v0.2.1/leenfetch-v0.2.1-REHL-x86_64.rpm
sudo rpm -i leenfetch-*.rpm
```

---

### Install on Windows (via .zip)

If you're on Windows, download the latest `.zip` from the [GitHub Releases](https://github.com/drunkleen/leenfetch/releases):

powershell:

- AMD64 (x86_64)
```powershell
Invoke-WebRequest -Uri "https://github.com/drunkleen/leenfetch/releases/download/v0.2.1/leenfetch-v0.2.1-windows-x86_64.zip" -OutFile "leenfetch-win.zip"
Expand-Archive .\leenfetch-win.zip -DestinationPath .

.\leenfetch-v0.2.1-windows-x86_64.exe
```

> Make sure you're in the same directory as `leenfetch.exe` when running the command.

---

### Install from crates.io

Make sure you have [Rust & Cargo](https://rustup.rs/) installed:

```bash
cargo install leenfetch
````

After that, just run:

```bash
leenfetch
```

If you hit issues with `PATH`, try adding `~/.cargo/bin` to your shell:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

### 🛠️ Manual Installation (Build from Source)

```bash
git clone https://github.com/drunkleen/leenfetch.git
cd leenfetch
cargo build --release
```

Add to PATH:

```bash
cp target/release/leenfetch ~/.local/bin/
```

Then run:

```bash
leenfetch
```

---

## 📥 Using Piped Input

LeenFetch can accept piped input to use as the ASCII logo.

This allows you to create dynamic, fun logos on the fly using other command-line tools.

**Examples:**

```bash
echo "Rustacean" | leenfetch
```

```bash
fortune | cowsay | leenfetch
```

LeenFetch will detect piped input via `stdin` and render the ASCII art above your system information.

If no piped input is provided, it will fall back to your configured or auto-detected ASCII art.


---

## ⚙️ Configuration

On first run, LeenFetch writes a single `config.jsonc` file to your configuration directory:

```bash
# Linux
~/.config/leenfetch/config.jsonc

# Windows
C:\Users\<username>\AppData\Roaming\leenfetch\config.jsonc
```

The file uses JSON with comments (JSONC), so you can keep inline explanations next to your settings. It combines the previous trio of files into three sections:

- `flags` — Controls display and formatting options for each block.
- `toggles` — Controls which information blocks are shown or hidden.
- `modules` — Controls the order, headings, and custom rows in the output.

### flags section

The `flags` object lets you fine-tune how each block of information is displayed. Pick ASCII art, choose units, and decide how detailed each line should be.

> If input is piped into `leenfetch`, the ASCII logo from `ascii_distro` or `custom_ascii_path` is ignored and the piped content is used instead.

```jsonc
{
  "flags": {
    // Select which distribution's ASCII art to display at the top.
    "ascii_distro": "auto",
    // How to show battery info: "off", "bar", "infobar", or "barinfo".
    "battery_display": "barinfo",
    // Include shell version in the output.
    "shell_version": true
  }
}
```

### toggles section

The `toggles` object controls which blocks of information are shown in the output. Set each option to `true` to show that block, or `false` to hide it.

```jsonc
{
  "toggles": {
    "show_titles": true,
    "show_gpu": true,
    "show_song": false
  }
}
```

### modules array

The `modules` array controls the rendering order. Entries can be a literal string (use `"break"` for a blank line) or an object describing a module. Objects accept a `type` (matching LeenFetch modules) and optional properties like `key` for the label or `format` for custom text.

```jsonc
{
  "modules": [
    "break",
    { "type": "custom", "format": "== System ==" },
    { "type": "titles", "key": "User" },
    { "type": "distro", "key": "Distro" },
    { "type": "cpu", "key": "CPU" },
    { "type": "memory", "key": "Memory" },
    { "type": "colors", "key": "" }
  ]
}
```

Rearrange, duplicate, or remove entries to customize your output. Insert `"break"` wherever you want an empty spacer line.

---

### How to Edit

- Open `config.jsonc` in your favorite text editor.
- Read the inline comments for a full explanation of every option.
- Change values as you like, save, and re-run `leenfetch` to see your changes.

For advanced details, see the comments in `config.jsonc` or check the [wiki](https://github.com/drunkleen/leenfetch/wiki) (if available).

## 🎯 Roadmap

| Feature                     | Status        |
| --------------------------- | ------------- |
| Base module system          | ✅ Done       |
| Config file loader          | ✅ Done       |
| Custom layout tags          | ✅ Done       |
| GPU/CPU/Mem/DE/WM detection | ✅ Done       |
| Linux support               | ✅ Done       |
| Windows support             | ✅ Done       |
| CLI override options        | 🔁 Basic      |
| ASCII art & theming         | 🔁 Basic      |
| Multi-Threading             | 🔄 Planned    |
| macOS support               | 🔄 Planned    |
| OpenBSD support             | 🔄 Planned    |
| Plugin/module system        | ❓ Maybe      |
| Fetch info over SSH         | ❓ Maybe      |

---

## 🤝 Contributing

1. Fork the repo
2. Create your branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'feat: add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Create a Pull Request

We welcome clean PRs and documented modules! ✨

---

## ☕ Support LeenFetch

If you have found LeenFetch valuable and would like to contribute to its ongoing development, your support is greatly
appreciated. You can show your appreciation by making a donation
through [PayPal](https://www.paypal.com/paypalme/RDarvishifar) or any of the following cryptocurrency networks:

- **Bitcoin (BTC):** `bc1qsmvxpn79g6wkel3w67k37r9nvzm5jnggeltxl6`
- **ETH/BNB/MATIC (ERC20, BEP20):** `0x8613aD01910d17Bc922D95cf16Dc233B92cd32d6`
- **USDT/TRON (TRC20):** `TGNru3vuDfPh5zBJ31DKzcVVvFgfMK9J48`
- **Dogecoin (DOGE):** `D8U25FjxdxdQ7pEH37cMSw8HXBdY1qZ7n3`

Your generous contribution ensures the continued improvement and maintenance of LeenFetch. ❤️

Thank you for supporting the project! 🙏

---

## 📄 License

[MIT](./LICENSE) License © [DrunkLeen](https://github.com/drunkleen)

---

## 💡 Inspiration

* [Neofetch](https://github.com/dylanaraps/neofetch)
* [Fastfetch](https://github.com/fastfetch-cli/fastfetch)
* [Rust](https://www.rust-lang.org/) — the foundation of LeenFetch
