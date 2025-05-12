# 🧠 LeenFetch

<p align="center">
    <a href="../README.md">ENGLISH</a>
    | <a href="./README-fa.md">فارسی</a>
    | <a href="./README-ru.md">Русский </a>
    | <a href="#">中文</a>
</p>

### 一款快速、极简且可自定义的系统信息工具，由 Rust 编写 —— 真正面向高端用户的 Neofetch 替代品。

---

<img src="./src/TokyoNight.png" width="32%" /><img src="./src/SandStorm.png" width="32%" /><img src="./src/Aura.png" width="32%" />


<img src="./src/debian.jpg"/>
<img src="./src/windows10.jpg"/>
<img src="./src/ubuntu.jpg"/>
<img src="./src/windows.jpg"/>

## 🚀 功能特点

- ⚡ 极速启动，归功于 Rust 的高性能
- 🎨 支持自定义布局并提供彩色终端输出
- 🧩 模块化设计 — 可在配置中启用或禁用组件
- 💾 智能默认值，轻松拓展
- 📦 自动检测已安装软件包、Shell、GPU、桌面环境/窗口管理器等
- 🔌 简洁的配置文件：`~/.config/leenfetch/config.conf`
<!-- - 🖼️ 支持自定义 ASCII 图与动态主题（计划中） -->

---

## 📦 安装方式

### ✅ 通过 crates.io 安装（推荐）

确保你已安装 [cargo](https://rustup.rs/)：

```bash
cargo install leenfetch
````

安装完成后运行：

```bash
leenfetch
```

如果出现 PATH 问题，请将 `~/.cargo/bin` 添加至你的环境变量：

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

## ⚙️ 配置文件

首次运行时，LeenFetch 会生成默认配置：

```bash
~/.config/leenfetch/config.conf
```

你可以通过编辑该文件来启用/禁用模块、更改布局或调整缩写格式。

### 示例选项：

```ini
# 模块开关
show_os=on
show_kernel=on
show_gpu=on
show_packages=off

# 缩写与格式设置
distro_shorthand=tiny
cpu_temp=C
memory_unit=mib
```

布局完全支持自定义 `[标签]`，例如：

```ini
layout="
[titles]]
{username}@{hostname}
[/titles]

[cpu]
CPU: {cpu_index}
[/cpu]
"
```

查看完整配置选项：

```bash
leenfetch --list-options
```

---

## 🎯 开发计划

| 功能                  | 状态     |
| ------------------- | ------ |
| 基础模块系统              | ✅ 已完成  |
| 配置文件加载器             | ✅ 已完成  |
| 自定义布局标签             | ✅ 已完成  |
| GPU/CPU/内存/DE/WM 检测 | ✅ 已完成  |
| 主题与颜色配置             | ✅ 基础实现 |
| ASCII 艺术支持          | ✅ 基础实现 |
| 多线程处理               | 🔄 计划中 |
| 支持 Windows          | ✅ 已完成 |
| 支持 macOS            | 🔄 计划中 |
| 支持 OpenBSD          | 🔄 计划中 |
| 插件/模块系统             | ❓ 考虑中  |
| 通过 SSH 获取远程信息       | ❓ 考虑中  |

---

## 🤝 贡献指南

1. Fork 本仓库
2. 创建新分支 (`git checkout -b feature/my-feature`)
3. 提交更改 (`git commit -m 'feat: add my feature'`)
4. 推送分支 (`git push origin feature/my-feature`)
5. 创建 Pull Request 请求合并

欢迎提交清晰、规范的 PR 和模块文档 ✨

---

## 💰 捐赠支持

如果你觉得 LeenFetch 有用，并希望支持它的持续开发，非常感谢你的支持！

你可以通过 [PayPal](https://www.paypal.com/paypalme/RDarvishifar) 捐赠，或者使用以下任意加密货币地址：

* **比特币 (BTC)：** `bc1qsmvxpn79g6wkel3w67k37r9nvzm5jnggeltxl6`
* **以太坊/BNB/MATIC (ERC20, BEP20)：** `0x8613aD01910d17Bc922D95cf16Dc233B92cd32d6`
* **USDT / 波场 (TRC20)：** `TGNru3vuDfPh5zBJ31DKzcVVvFgfMK9J48`
* **狗狗币 (DOGE)：** `D8U25FjxdxdQ7pEH37cMSw8HXBdY1qZ7n3`

感谢你对 LeenFetch 的支持 ❤️

---

## 📄 许可证

[MIT](./LICENSE) 许可证 © [DrunkLeen](https://github.com/drunkleen)

---

## 💡 灵感来源

* [Neofetch](https://github.com/dylanaraps/neofetch)
* [Fastfetch](https://github.com/fastfetch-cli/fastfetch)
* [Rust](https://www.rust-lang.org/) —— LeenFetch 的基石
