# 🧠 LeenFetch

[![Crates.io](https://img.shields.io/crates/v/leenfetch)](https://crates.io/crates/leenfetch)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> 🌐 Доступно на: [English](../README.md) | [فارسی](./README-fa.md) | [Русский](#) | [中文](./README-zh.md)

##### Быстрый, минималистичный и настраиваемый инструмент для отображения информации о системе, написанный на Rust — настоящая альтернатива Neofetch для продвинутых пользователей.

> ⚠️ **LeenFetch активно разрабатывается. Ожидайте частые обновления и возможные баги.**
> Хотите помочь? Мы рады вашему вкладу, тестированию и обратной связи!

---

## 💬 Обратная связь и проблемы

Нашли баг? Есть идея новой функции?
Пишите в [Issues](https://github.com/drunkleen/leenfetch/issues) или заходите на [Discussions](https://github.com/drunkleen/leenfetch/discussions)!

---

## 📸 Скриншоты

<img src="./src/TokyoNight.png" width="32%" /><img src="./src/SandStorm.png" width="32%" /><img src="./src/Aura.png" width="32%" />

<img src="./src/debian.jpg"/>
<img src="./src/windows10.jpg"/>
<img src="./src/ubuntu.jpg"/>
<img src="./src/windows.jpg"/>

---

## 🚀 Возможности

* ⚡ Молниеносный запуск благодаря Rust
* 🎨 Полностью настраиваемый вывод с поддержкой ANSI-цветов
* 🧩 Модульная архитектура — включайте и отключайте компоненты в конфиге
* 💾 Умные настройки по умолчанию и легкость в расширении
* 📦 Автоматическое определение пакетов, оболочки, GPU, DE/WM и др.
* 🖼️ Поддержка кастомного ASCII-арта
* 🎨 Тематические цветовые схемы (`ascii_colors=distro`, и т.д.)
* 🔌 Простой конфигурационный файл: `~/.config/leenfetch/config.conf`

---

## 📦 Установка

### ✅ Установка через crates.io (рекомендуется)

Убедитесь, что установлен [Rust и Cargo](https://rustup.rs/):

```bash
cargo install leenfetch
```

После установки запустите:

```bash
leenfetch
```

Если возникли проблемы с `$PATH`, добавьте это в `.bashrc` или `.zshrc`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

### 🛠️ Установка из исходников

```bash
git clone https://github.com/drunkleen/leenfetch.git
cd leenfetch
cargo build --release
```

Добавьте в `PATH`:

```bash
cp target/release/leenfetch ~/.local/bin/
```

И затем запустите:

```bash
leenfetch
```

---

## ⚙️ Конфигурация

При первом запуске LeenFetch создаёт:

```bash
~/.config/leenfetch/config.conf
```

Редактируйте этот файл для включения/отключения модулей, изменения разметки и других настроек.

### Примеры параметров:

```ini
# Включение/выключение модулей
show_os=on
show_kernel=on
show_gpu=on
show_packages=off

# Форматирование
distro_shorthand=tiny
cpu_temp=C
memory_unit=mib
```

Настраиваемая разметка:

```ini
layout="
[titles]
{username}@{hostname}
[/titles]

[cpu]
CPU: {cpu_index}
[/cpu]
"
```

Полный список опций:

```bash
leenfetch --list-options
```

---

## 🎯 Планы

| Функция                   | Статус      |
| ------------------------- | ----------- |
| Базовая модульная система | ✅ Готово    |
| Загрузка конфигурации     | ✅ Готово    |
| Кастомные теги в шаблонах | ✅ Готово    |
| Обнаружение GPU/CPU/DE/WM | ✅ Готово    |
| Поддержка Linux           | ✅ Готово    |
| Поддержка Windows         | ✅ Готово    |
| CLI-переопределения       | 🔁 Базовая  |
| Темы и ASCII-арт          | 🔁 Базовая  |
| Многопоточность           | 🔄 В планах |
| Поддержка macOS           | 🔄 В планах |
| Поддержка OpenBSD         | 🔄 В планах |
| Система плагинов/модулей  | ❓ Возможно  |
| Получение данных по SSH   | ❓ Возможно  |

---

## 🤝 Вклад в проект

1. Сделайте форк репозитория
2. Создайте новую ветку: `git checkout -b feature/my-feature`
3. Закоммитьте изменения: `git commit -m 'feat: добавлена моя фича'`
4. Отправьте ветку: `git push origin feature/my-feature`
5. Создайте Pull Request

Мы приветствуем чистые PR и хорошо документированные модули! ✨

---

## ☕ Поддержка LeenFetch

Если вам нравится LeenFetch и вы хотите поддержать разработку:

* [PayPal](https://www.paypal.com/paypalme/RDarvishifar)
* **Bitcoin (BTC):** `bc1qsmvxpn79g6wkel3w67k37r9nvzm5jnggeltxl6`
* **ETH/BNB/MATIC (ERC20/BEP20):** `0x8613aD01910d17Bc922D95cf16Dc233B92cd32d6`
* **USDT/TRON (TRC20):** `TGNru3vuDfPh5zBJ31DKzcVVvFgfMK9J48`
* **Dogecoin (DOGE):** `D8U25FjxdxdQ7pEH37cMSw8HXBdY1qZ7n3`

💖 Спасибо за поддержку LeenFetch!

---

## 📄 Лицензия

[MIT](./LICENSE) Лицензия © [DrunkLeen](https://github.com/drunkleen)

---

## 💡 Вдохновение

* [Neofetch](https://github.com/dylanaraps/neofetch)
* [Fastfetch](https://github.com/fastfetch-cli/fastfetch)
* [Rust](https://www.rust-lang.org/) — основа проекта LeenFetch
