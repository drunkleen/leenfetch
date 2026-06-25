# 🧠 LeenFetch

[![Crates.io](https://img.shields.io/crates/v/leenfetch)](https://crates.io/crates/leenfetch)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/drunkleen/leenfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/drunkleen/leenfetch/actions/workflows/ci.yml)
[![Build](https://github.com/drunkleen/leenfetch/actions/workflows/build-check.yml/badge.svg)](https://github.com/drunkleen/leenfetch/actions/workflows/build-check.yml)
[![Release](https://github.com/drunkleen/leenfetch/actions/workflows/release.yml/badge.svg)](https://github.com/drunkleen/leenfetch/actions/workflows/release.yml)

A fast, minimal, and customizable system information tool built in Rust — your alternative to Neofetch.

## 📸 Screenshots

<img src="./assets/TokyoNight.png" width="32%" /><img src="./assets/SandStorm.png" width="32%" /><img src="./assets/Aura.png" width="32%" />

## 🚀 Quick Install

```bash
# Linux
curl -sSfL https://raw.githubusercontent.com/drunkleen/leenfetch/master/install.sh | sh

# Windows (PowerShell)
iwr https://raw.githubusercontent.com/drunkleen/leenfetch/master/install.ps1 | iex
```

### Update

```bash
# Linux
curl -sSfL https://raw.githubusercontent.com/drunkleen/leenfetch/master/install.sh | sh -s -- --update

# Windows
iwr https://raw.githubusercontent.com/drunkleen/leenfetch/master/install.ps1 | iex; Install-LeenFetch -Update
```

> See the [wiki](https://github.com/drunkleen/leenfetch/wiki) for manual installation on Arch/Debian/RPM/Windows/crates.io and build from source.

## ⚡ Features

- Blazing fast startup written in Rust
- Customizable output layout with colorized terminal output
- Modular — enable or disable components via config
- Detects packages, shell, GPU, DE/WM, and more
- Custom ASCII art support
- Piped ASCII input (`fortune | cowsay | leenfetch`)
- Remote system info over SSH
- Machine-readable JSON output

## 📖 Full Documentation

Everything else — configuration, module reference, CLI options, SSH, theming, troubleshooting — is in the **[wiki](https://github.com/drunkleen/leenfetch/wiki)**.

## 🎯 Roadmap

| Feature                     | Status  |
| --------------------------- | ------- |
| Core module system          | ✅ Done |
| Linux + Windows support     | ✅ Done |
| JSON output + SSH fetching  | ✅ Done |
| CLI override options        | ✅ Done |
| ASCII art & theming         | ✅ Done |
| Plugin/module system        | ❓ Maybe |

## 🤝 Contributing

1. Fork the repo
2. Create your branch (`git checkout -b feature/my-feature`)
3. Commit your changes
4. Push and create a Pull Request

## ☕ Support

- **Patreon** — [patreon.com/drunkleen](https://patreon.com/drunkleen)
- **Buy Me a Coffee** — [buymeacoffee.com/drunkleen](https://buymeacoffee.com/drunkleen)

## 📄 License

[MIT](./LICENSE) © [DrunkLeen](https://github.com/drunkleen)
