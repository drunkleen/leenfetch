# 🧠 LeenFetch

[![Crates.io](https://img.shields.io/crates/v/leenfetch)](https://crates.io/crates/leenfetch)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

<div dir="rtl">

> 🌐 موجود به زبان‌های: [English](../README.md) | [فارسی](#) | [Русский](./README-ru.md) | [中文](./README-zh.md)

##### ابزاری سریع، مینیمال و قابل تنظیم برای نمایش اطلاعات سیستم نوشته‌شده با Rust — جایگزینی برای Neofetch مخصوص کاربران قدرتمند واقعی.

> ⚠️ **LeenFetch هنوز در حال توسعه فعال است. انتظار باگ و بهبودهای منظم را داشته باشید!**  
>  
> می‌خواهید کمک کنید؟ مشارکت، تست و ارائه بازخورد همیشه خوش‌آمد هستند!

> **این ترجمه با کمک هوش مصنوعی انجام شده است.**

## 💬 بازخورد و گزارش مشکلات

باگی پیدا کرده‌اید؟ ایده‌ای برای یک قابلیت دارید؟  
به صفحه [Issues](https://github.com/drunkleen/leenfetch/issues) یا [Discussions](https://github.com/drunkleen/leenfetch/discussions) سر بزنید!

## 📚 فهرست مطالب

- [تصاویر](#-تصاویر)
- [قابلیت‌ها](#-قابلیتها)
- [نصب](#-نصب)
- [پیکربندی](#️-پیکربندی)
- [نقشه راه](#-نقشه-راه)
- [مشارکت](#-مشارکت)
- [حمایت از LeenFetch](#-حمایت-از-leenfetch)
- [لایسنس](#-لایسنس)
- [الهام‌بخش‌ها](#-الهامبخشها)

## 📸 تصاویر

<img src="./src/TokyoNight.png" width="32%" /><img src="./src/SandStorm.png" width="32%" /><img src="./src/Aura.png" width="32%" />

<img src="./src/debian.jpg"/>
<img src="./src/windows10.jpg"/>
<img src="./src/ubuntu.jpg"/>
<img src="./src/windows.jpg"/>

## 🚀 قابلیت‌ها

- ⚡ راه‌اندازی فوق‌العاده سریع به لطف Rust  
- 🎨 خروجی قابل تنظیم همراه با رنگ‌آمیزی در ترمینال  
- 🧩 طراحی ماژولار — می‌توانید اجزا را از طریق تنظیمات فعال یا غیرفعال کنید  
- 💾 تنظیمات پیش‌فرض هوشمند و قابل توسعه  
- 📦 شناسایی پکیج‌ها، شل، GPU، محیط دسکتاپ و موارد دیگر  
- 🖼️ پشتیبانی از هنر ASCII سفارشی با قابلیت تنظیم از طریق تنظیمات  
- 🎨 پشتیبانی از پروفایل‌های رنگی مبتنی بر تم (مانند `ascii_colors=distro`)  
- 🔌 فایل تنظیمات ساده: `~/.config/leenfetch/config.conf`  

## 📦 نصب

### ✅ نصب از crates.io (توصیه‌شده)

ابتدا مطمئن شوید که [Rust و Cargo](https://rustup.rs/) نصب هستند:

```bash
cargo install leenfetch
```

سپس اجرا کنید:

```bash
leenfetch
```

اگر با خطای مربوط به `PATH` مواجه شدید، مسیر زیر را به شل خود اضافه کنید:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

### 🛠️ نصب دستی (ساخت از سورس)

```bash
git clone https://github.com/drunkleen/leenfetch.git
cd leenfetch
cargo build --release
```

افزودن به PATH:

```bash
cp target/release/leenfetch ~/.local/bin/
```

سپس اجرا کنید:

```bash
leenfetch
```

---

## ⚙️ پیکربندی

در اولین اجرا، LeenFetch سه فایل پیکربندی در مسیر زیر ایجاد می‌کند:

```bash
# در لینوکس
~/.config/leenfetch/

# در ویندوز
C:\Users\<username>\AppData\Roaming\leenfetch\
```

- `flags.ron` — گزینه‌های نمایش و قالب‌بندی برای هر بخش را کنترل می‌کند.  
- `toggles.ron` — تعیین می‌کند کدام بلوک‌های اطلاعات نمایش داده شوند یا پنهان بمانند.  
- `print_layout.ron` — ترتیب و برچسب‌های هر بلوک در خروجی را مشخص می‌کند.  

هر فایل به‌خوبی کامنت‌گذاری شده تا با خواندن خود فایل بتوانید همه گزینه‌ها را درک کنید.

### flags.ron

این فایل امکان تنظیم دقیق نمایش هر بخش اطلاعات را فراهم می‌کند. می‌توانید:

- انتخاب کنید کدام ASCII art و پالت رنگی استفاده شود  
- روش نمایش باتری، دیسک، حافظه و اطلاعات پکیج را مشخص کنید  
- واحدها، سطح جزئیات و قالب‌بندی را انتخاب کنید  
- فعال یا غیرفعال کردن ویژگی‌هایی مانند برند CPU، دما، نسخه شل و غیره  

**مثال:**
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

این فایل تعیین می‌کند کدام بخش‌ها در خروجی نشان داده شوند. اگر یک گزینه روی `true` باشد، آن بخش نمایش داده می‌شود، و اگر `false` باشد، پنهان می‌ماند.

**مثال:**
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

این فایل ترتیب و برچسب‌های هر بخش در خروجی را کنترل می‌کند. می‌توانید بخش‌ها را جابجا کنید، حذف کنید یا نام‌گذاری مجدد انجام دهید.

**مثال:**
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

### نحوه ویرایش

- هرکدام از فایل‌های `.ron` را با ویرایشگر متن دلخواه خود باز کنید  
- توضیحات بالا یا کنار هر گزینه را بخوانید  
- مقدارها را تغییر دهید، ذخیره کنید، و دوباره `leenfetch` را اجرا کنید  

برای جزئیات بیشتر، می‌توانید به توضیحات داخل هر فایل یا [ویکی](https://github.com/drunkleen/leenfetch/wiki) (در صورت وجود) مراجعه کنید.

## 🎯 نقشه راه

| ویژگی                         | وضعیت         |
|-------------------------------|---------------|
| سیستم پایه ماژول              | ✅ انجام شده   |
| بارگذاری فایل تنظیمات        | ✅ انجام شده   |
| برچسب‌های چیدمان سفارشی       | ✅ انجام شده   |
| شناسایی GPU/CPU/Mem/DE/WM    | ✅ انجام شده   |
| پشتیبانی از لینوکس           | ✅ انجام شده   |
| پشتیبانی از ویندوز           | ✅ انجام شده   |
| گزینه‌های CLI برای override  | 🔁 پایه‌ای     |
| ASCII art و تم‌ها             | 🔁 پایه‌ای     |
| چندریسمانی (Multi-Threading) | 🔄 در برنامه   |
| پشتیبانی از macOS            | 🔄 در برنامه   |
| پشتیبانی از OpenBSD          | 🔄 در برنامه   |
| سیستم افزونه/ماژول           | ❓ شاید        |
| دریافت اطلاعات از راه دور    | ❓ شاید        |

---

## 🤝 مشارکت

1. مخزن را fork کنید  
2. یک شاخه جدید بسازید (`git checkout -b feature/my-feature`)  
3. تغییرات خود را commit کنید (`git commit -m 'feat: add my feature'`)  
4. به شاخه push کنید (`git push origin feature/my-feature`)  
5. یک Pull Request ارسال کنید

ما از PRهای تمیز و ماژول‌های مستندسازی‌شده استقبال می‌کنیم! ✨

---

## ☕ حمایت از LeenFetch

اگر LeenFetch برای شما مفید بوده و مایل به حمایت از توسعه آن هستید، کمک شما واقعاً قدردانی می‌شود. می‌توانید از طریق [PayPal](https://www.paypal.com/paypalme/RDarvishifar) یا ارزهای دیجیتال زیر حمایت مالی داشته باشید:

- **بیت‌کوین (BTC):** `bc1qsmvxpn79g6wkel3w67k37r9nvzm5jnggeltxl6`  
- **ETH/BNB/MATIC (ERC20, BEP20):** `0x8613aD01910d17Bc922D95cf16Dc233B92cd32d6`  
- **USDT/TRON (TRC20):** `TGNru3vuDfPh5zBJ31DKzcVVvFgfMK9J48`  
- **دوج‌کوین (DOGE):** `D8U25FjxdxdQ7pEH37cMSw8HXBdY1qZ7n3`

کمک سخاوتمندانه شما باعث ادامه توسعه و نگهداری LeenFetch می‌شود. ❤️

سپاس از حمایت شما! 🙏

---

## 📄 لایسنس

پروژه تحت [لایسنس MIT](./LICENSE) و متعلق به [DrunkLeen](https://github.com/drunkleen) است.

---

## 💡 الهام‌بخش‌ها

* [Neofetch](https://github.com/dylanaraps/neofetch)  
* [Fastfetch](https://github.com/fastfetch-cli/fastfetch)  
* [Rust](https://www.rust-lang.org/) — زیربنای LeenFetch


<div>