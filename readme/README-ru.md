# 🧠 LeenFetch

[![Crates.io](https://img.shields.io/crates/v/leenfetch)](https://crates.io/crates/leenfetch)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> 🌐 Доступно на языках: [English](../README.md) | [فارسی](./README-fa.md) | [Русский](#) | [中文](./README-zh.md)

##### Быстрый, минималистичный и настраиваемый инструмент отображения информации о системе, написанный на Rust — альтернатива Neofetch для настоящих хардкорных пользователей.

> ⚠️ **LeenFetch находится в активной разработке. Ожидайте регулярные баги и улучшения!**  
>  
> Хотите помочь? Мы всегда рады вкладу, тестированию и обратной связи!

> **Этот перевод был выполнен с помощью искусственного интеллекта.**

## 💬 Обратная связь и проблемы

Нашли баг? Есть идея для функции?  
Переходите на страницу [Issues](https://github.com/drunkleen/leenfetch/issues) или [Discussions](https://github.com/drunkleen/leenfetch/discussions)!

## 📚 Содержание

- [Скриншоты](#-скриншоты)
- [Возможности](#-возможности)
- [Установка](#-установка)
- [Конфигурация](#️-конфигурация)
- [Дорожная карта](#-дорожная-карта)
- [Вклад](#-как-внести-вклад)
- [Поддержка LeenFetch](#-поддержка-leenfetch)
- [Лицензия](#-лицензия)
- [Вдохновение](#-вдохновение)

## 📸 Скриншоты

<img src="./src/TokyoNight.png" width="32%" /><img src="./src/SandStorm.png" width="32%" /><img src="./src/Aura.png" width="32%" />

<img src="./src/debian.jpg"/>
<img src="./src/windows10.jpg"/>
<img src="./src/ubuntu.jpg"/>
<img src="./src/windows.jpg"/>

## 🚀 Возможности

- ⚡ Молниеносный запуск благодаря Rust  
- 🎨 Настраиваемый вывод с цветами в терминале  
- 🧩 Модульная архитектура — включайте/отключайте блоки через конфиг  
- 💾 Умные настройки по умолчанию, легко расширяемые  
- 📦 Определение установленных пакетов, shell, GPU, DE/WM и прочего  
- 🖼️ Поддержка ASCII-артов и переопределение через конфиг  
- 🎨 Тематические цветовые профили (например, `ascii_colors=distro`)  
- 🔌 Простой конфиг-файл: `~/.config/leenfetch/config.conf`  

## 📦 Установка

### ✅ Установка через crates.io (рекомендуется)

Убедитесь, что у вас установлен [Rust и Cargo](https://rustup.rs/):

```bash
cargo install leenfetch
```

Запуск:

```bash
leenfetch
```

Если возникают проблемы с `PATH`, добавьте `~/.cargo/bin` в переменную окружения:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

### 🛠️ Ручная установка (из исходников)

```bash
git clone https://github.com/drunkleen/leenfetch.git
cd leenfetch
cargo build --release
```

Добавьте в PATH:

```bash
cp target/release/leenfetch ~/.local/bin/
```

И запустите:

```bash
leenfetch
```

---

## ⚙️ Конфигурация

При первом запуске LeenFetch создаёт три файла конфигурации:

```bash
# Linux
~/.config/leenfetch/

# Windows
C:\Users\<username>\AppData\Roaming\leenfetch\
```

- `flags.ron` — настройка отображения и форматирования каждого блока  
- `toggles.ron` — включение/отключение блоков информации  
- `print_layout.ron` — порядок и названия блоков в выводе  

Каждый файл хорошо прокомментирован — читайте прямо в нём, чтобы понять каждую опцию.

### flags.ron

Позволяет тонко настроить отображение каждого блока информации:

- Выбор ASCII-арта и цветовой палитры  
- Способ отображения батареи, дисков, памяти и пакетов  
- Единицы измерения, уровень деталей, формат  
- Включение/отключение фич (например, бренд CPU, температура, версия shell)

**Пример:**
```ron
(
    // Select which distribution's ASCII art to display at the top.
    // Options:
    //   "auto"   - Automatically detect and use your current distribution's ASCII art.
    //   "distro" - Use the detected distribution's art, but fallback to a generic one if not found.
    //   <name>   - Use a specific distro's art (e.g., "arch", "ubuntu", "debian").
    ascii_distro: "auto",
    ...
)
```

### toggles.ron

Управляет, какие блоки отображаются в выводе.  
Установите `true`, чтобы включить блок, `false` — чтобы скрыть.

**Пример:**
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

Настраивает порядок и названия блоков вывода.  
Вы можете переставлять, удалять или переименовывать секции.

**Пример:**
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

### Как редактировать

- Откройте любой из `.ron` файлов в любимом текстовом редакторе  
- Прочитайте комментарии к каждой опции  
- Измените значения, сохраните файл, снова запустите `leenfetch`  

Дополнительную информацию можно найти в самих конфиг-файлах или в [wiki](https://github.com/drunkleen/leenfetch/wiki) (если доступна).

## 🎯 Дорожная карта

| Функция                      | Статус        |
|-----------------------------|---------------|
| Базовая система модулей     | ✅ Готово      |
| Загрузка конфигов           | ✅ Готово      |
| Теги пользовательской схемы | ✅ Готово      |
| Обнаружение CPU/GPU/Mem/DE/WM | ✅ Готово  |
| Поддержка Linux             | ✅ Готово      |
| Поддержка Windows           | ✅ Готово      |
| CLI-переопределения         | 🔁 Базовые     |
| ASCII-арт и темы            | 🔁 Базовые     |
| Многопоточность             | 🔄 В планах    |
| Поддержка macOS             | 🔄 В планах    |
| Поддержка OpenBSD           | 🔄 В планах    |
| Плагины/модули              | ❓ Возможно    |
| Получение данных по SSH     | ❓ Возможно    |

---

## 🤝 Как внести вклад

1. Сделайте fork репозитория  
2. Создайте ветку (`git checkout -b feature/my-feature`)  
3. Зафиксируйте изменения (`git commit -m 'feat: add my feature'`)  
4. Отправьте ветку (`git push origin feature/my-feature`)  
5. Создайте Pull Request

Мы ценим чистый код и задокументированные модули ✨

---

## ☕ Поддержка LeenFetch

Если вы считаете LeenFetch полезным и хотите поддержать его развитие — будем благодарны!

Можно сделать пожертвование через [PayPal](https://www.paypal.com/paypalme/RDarvishifar) или использовать криптовалюты:

- **Bitcoin (BTC):** `bc1qsmvxpn79g6wkel3w67k37r9nvzm5jnggeltxl6`  
- **ETH/BNB/MATIC (ERC20, BEP20):** `0x8613aD01910d17Bc922D95cf16Dc233B92cd32d6`  
- **USDT/TRON (TRC20):** `TGNru3vuDfPh5zBJ31DKzcVVvFgfMK9J48`  
- **Dogecoin (DOGE):** `D8U25FjxdxdQ7pEH37cMSw8HXBdY1qZ7n3`

Спасибо за вашу поддержку! ❤️ 🙏

---

## 📄 Лицензия

Проект распространяется по лицензии [MIT](./LICENSE) © [DrunkLeen](https://github.com/drunkleen)

---

## 💡 Вдохновение

* [Neofetch](https://github.com/dylanaraps/neofetch)  
* [Fastfetch](https://github.com/fastfetch-cli/fastfetch)  
* [Rust](https://www.rust-lang.org/) — основа LeenFetch
