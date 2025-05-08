# 🧠 LeenFetch


<p align="center">
    <a href="#">ENGLISH</a>
    | <a href="./readme/README-fa.md">فارسی</a>
    | <a href="./readme/README-ru.md">Русский </a>
    | <a href="./readme/README-zh.md">中文</a>
</p>

### A fast, minimal, and customizable system information tool built in Rust — your alternative to Neofetch, for real power users.

---

<img src="./readme/src/TokyoNight.png" width="30%" />
<img src="./readme/src/SandStorm.png" width="30%" />
<img src="./readme/src/Aura.png" width="30%" />


## 🚀 Features

- ⚡ Blazing fast startup thanks to Rust
- 🎨 Customizable output layout with colorized terminal output
- 🧩 Modular design — enable or disable components via config
- 💾 Smart defaults but easily extendable
- 📦 Detects installed packages, shell, GPU, DE/WM, and more
- 🔌 Simple config file: `~/.config/leenfetch/config.conf`
<!-- - 🖼️ Supports custom ASCII art and dynamic themes -->

---




## 📦 Installation

### ✅ Install from crates.io (recommended)

make sure you have [cargo](https://rustup.rs/) installed

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

## ⚙️ Configuration

On first run, LeenFetch will generate:

```bash
~/.config/leenfetch/config.conf
```

Edit it to enable/disable modules, change layout, or tweak shorthand formats.

### Example Options:

```ini
# Output toggles
show_os=on
show_kernel=on
show_gpu=on
show_packages=off

# Shorthands and formatting
distro_shorthand=tiny
cpu_temp=C
memory_unit=mib
```

The layout is fully customizable with `[[tags]]`, e.g.:

```ini
layout="
[[titles]]
{username}@{hostname}
[[/titles]]

[[cpu]]
CPU: {cpu_index}
[[/cpu]]
"
```

For a full list of options:

```bash
leenfetch --list-options
```

---

## 🎯 Roadmap

| Feature                     | Status        |
| --------------------------- | ------------- |
| Base module system          | ✅ Done       |
| Config file loader          | ✅ Done       |
| Custom layout tags          | ✅ Done       |
| GPU/CPU/Mem/DE/WM detection | ✅ Done       |
| Theming / color profiles    | ✅ Basic      |
| ASCII art support           | 🔄 Planned    |
| Multi-Threading             | 🔄 Planned    |
| Windows support             | ✅ Planned    |
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

## Donation

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

---
