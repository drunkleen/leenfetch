# 🧠 LeenFetch

[![Crates.io](https://img.shields.io/crates/v/leenfetch)](https://crates.io/crates/leenfetch)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> 🌐 支持语言: [English](../README.md) | [فارسی](./README-fa.md) | [Русский](./README-ru.md) | [中文](#)

##### 一个用 Rust 构建的快速、简洁且可自定义的系统信息工具 — 真正适合高级用户的 Neofetch 替代品。

> ⚠️ **LeenFetch 正在积极开发中，可能会遇到 bug 和频繁的更新！**  
>  
> 想参与项目？欢迎贡献代码、测试和提出建议！

> **本中文翻译由人工智能完成。**

## 💬 反馈与问题

发现 Bug 或有新功能的想法？  
请前往 [Issues](https://github.com/drunkleen/leenfetch/issues) 或参与 [Discussions](https://github.com/drunkleen/leenfetch/discussions)！

## 📚 目录

- [截图](#-截图)
- [功能](#-功能亮点)
- [安装](#-安装)
- [配置](#️-配置)
- [开发计划](#-开发计划)
- [贡献指南](#-贡献指南)
- [支持 LeenFetch](#-支持-leenfetch)
- [许可证](#-许可证)
- [灵感来源](#-灵感来源)

## 📸 截图

<img src="./src/TokyoNight.png" width="32%" /><img src="./src/SandStorm.png" width="32%" /><img src="./src/Aura.png" width="32%" />

<img src="./src/debian.jpg"/>
<img src="./src/windows10.jpg"/>
<img src="./src/ubuntu.jpg"/>
<img src="./src/windows.jpg"/>

## 🚀 功能亮点

- ⚡ 基于 Rust，启动极快  
- 🎨 可定制的终端输出布局和配色  
- 🧩 模块化设计 — 可通过配置启用/禁用组件  
- 💾 默认设置智能，亦可灵活扩展  
- 📦 自动检测已安装的软件包、Shell、GPU、桌面环境/窗口管理器等  
- 🖼️ 支持自定义 ASCII 艺术图，并可在配置中覆盖  
- 🎨 支持基于主题的配色方案（如 `ascii_colors=distro`）  
- 🔌 简洁的配置文件：`~/.config/leenfetch/config.conf`  

## 📦 安装

### ✅ 推荐使用 crates.io 安装

请确保你已安装了 [Rust 和 Cargo](https://rustup.rs/)：

```bash
cargo install leenfetch
```

安装完成后，直接运行：

```bash
leenfetch
```

如果遇到 `PATH` 问题，请将以下路径加入 shell 环境变量：

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

### 🛠️ 手动安装（从源码构建）

```bash
git clone https://github.com/drunkleen/leenfetch.git
cd leenfetch
cargo build --release
```

添加到 PATH：

```bash
cp target/release/leenfetch ~/.local/bin/
```

然后运行：

```bash
leenfetch
```

---

## ⚙️ 配置

首次运行时，LeenFetch 会在以下路径生成三个配置文件：

```bash
# Linux
~/.config/leenfetch/

# Windows
C:\Users\<username>\AppData\Roaming\leenfetch\
```

- `flags.ron` — 控制每个信息块的显示样式和格式  
- `toggles.ron` — 控制哪些信息块显示/隐藏  
- `print_layout.ron` — 设置输出中各块的顺序和标签名  

每个配置文件都包含了详尽注释，直接阅读即可理解各项配置。

### flags.ron

用于精细控制每个模块的显示方式：

- 选择 ASCII 图案和配色方案  
- 配置电池、磁盘、内存、软件包等信息的显示样式  
- 设定单位、详细程度和格式  
- 控制是否显示 CPU 品牌、温度、Shell 版本等内容  

**示例：**
```ron
(
    // Select which distribution's ASCII art to display at the top.
    // Options:
    //   "auto"   - Automatically detect and use your current distribution's ASCII art.
    //   <name>   - Use a specific distro's art (e.g., "arch", "ubuntu", "debian").
    ascii_distro: "auto",
    ...
)
```

### toggles.ron

用于开启或关闭输出中的各个信息块。设置为 `true` 显示，`false` 则隐藏。

**示例：**
```ron
(
    // Show the user@host title at the top of the output.
    // true  - Display the title block (e.g., "snape@archbox").
    // false - Hide the title block.
    show_titles: true,
    ...
)
```

### print_layout.ron

用于控制输出时各模块的显示顺序和标签。你可以自由调整顺序、重命名或移除。

**示例：**
```ron
[
    // The user@host title block (e.g., "snape@archbox").
    (label: "Titles", field: "titles"),
    // The distribution (distro) information (e.g., "Arch Linux").
    (label: "Distro", field: "distro"),
    ...
]
```

---

### 编辑方式

- 使用你喜欢的文本编辑器打开 `.ron` 文件  
- 阅读注释来了解每个配置项的作用  
- 修改后保存，重新运行 `leenfetch` 查看效果  

详细说明可查看配置文件中的注释，或访问 [wiki](https://github.com/drunkleen/leenfetch/wiki)（如有）。

## 🎯 开发计划

| 功能                          | 状态       |
|-------------------------------|------------|
| 模块化系统                    | ✅ 已完成  |
| 配置文件加载器                | ✅ 已完成  |
| 自定义布局标签                | ✅ 已完成  |
| GPU/CPU/内存/桌面检测         | ✅ 已完成  |
| Linux 支持                    | ✅ 已完成  |
| Windows 支持                  | ✅ 已完成  |
| CLI 参数覆盖功能              | 🔁 基础支持|
| ASCII 艺术和主题              | 🔁 基础支持|
| 多线程支持                    | 🔄 计划中  |
| macOS 支持                    | 🔄 计划中  |
| OpenBSD 支持                  | 🔄 计划中  |
| 插件/模块系统                 | ❓ 待评估  |
| 通过 SSH 获取远程信息         | ❓ 待评估  |

---

## 🤝 贡献指南

1. Fork 本仓库  
2. 创建新分支 (`git checkout -b feature/my-feature`)  
3. 提交更改 (`git commit -m 'feat: add my feature'`)  
4. 推送分支 (`git push origin feature/my-feature`)  
5. 创建 Pull Request

欢迎干净整洁的 PR 和有注释的模块 ✨

---

## ☕ 支持 LeenFetch

如果你觉得 LeenFetch 对你有帮助，并愿意支持它的持续开发，我们将不胜感激！

你可以通过 [PayPal](https://www.paypal.com/paypalme/RDarvishifar) 或以下加密货币进行捐赠：

- **Bitcoin (BTC):** `bc1qsmvxpn79g6wkel3w67k37r9nvzm5jnggeltxl6`  
- **ETH/BNB/MATIC (ERC20, BEP20):** `0x8613aD01910d17Bc922D95cf16Dc233B92cd32d6`  
- **USDT/TRON (TRC20):** `TGNru3vuDfPh5zBJ31DKzcVVvFgfMK9J48`  
- **狗狗币 (DOGE):** `D8U25FjxdxdQ7pEH37cMSw8HXBdY1qZ7n3`

感谢你的支持！❤️ 🙏

---

## 📄 许可证

本项目采用 [MIT](./LICENSE) 许可证 © [DrunkLeen](https://github.com/drunkleen)

---

## 💡 灵感来源

* [Neofetch](https://github.com/dylanaraps/neofetch)  
* [Fastfetch](https://github.com/fastfetch-cli/fastfetch)  
* [Rust](https://www.rust-lang.org/) — LeenFetch 的基础语言
