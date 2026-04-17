# Contributing to LeenFetch

First off — thanks for taking the time to contribute! LeenFetch is a community-driven project and every bit of help counts, whether it's a bug report, a new distro logo, a performance tweak, or a full feature.

---

## Ways to Contribute

### Report Bugs
Found something broken? Open an [issue](https://github.com/drunkleen/leenfetch/issues) and include:
- Your OS and distro (e.g. `Arch Linux`, `Windows 11`)
- LeenFetch version (`leenfetch --version`)
- Steps to reproduce
- What you expected vs what happened

### Suggest Features
Have an idea? Start a [discussion](https://github.com/drunkleen/leenfetch/discussions) or open a feature request issue. Describe the use case — not just the solution.

### Add a Distro ASCII Art
LeenFetch supports built-in ASCII art for many distros. To add yours:
1. Find `src/modules/ascii.rs` and add a `const YOUR_DISTRO: &str = r#"..."#;` entry at the bottom
2. Add a match arm in `get_builtin_ascii_art()` — e.g. `("yourdistro", _) => YOUR_DISTRO`
3. Optionally add colors in `src/modules/colors.rs` under `get_builtin_distro_colors()`

### Fix Bugs or Improve Code
Check the [open issues](https://github.com/drunkleen/leenfetch/issues) — anything tagged `good first issue` or `help wanted` is a great starting point. Performance improvements and correctness fixes are always welcome.

### Improve Docs
Typos, unclear explanations, missing examples — all fair game. Docs live in `README.md`, `manual.md`, and `leenfetch.1.md`.

---

## Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (stable, edition 2024)
- On Linux: `libc` headers (usually pre-installed)
- On Windows: MSVC toolchain or MinGW

### Build
```bash
git clone https://github.com/drunkleen/leenfetch.git
cd leenfetch
cargo build
```

### Run
```bash
cargo run
```

### Test
```bash
cargo test
```

### Lint
```bash
cargo clippy -- -D warnings
cargo fmt --check
```

---

## Submitting a Pull Request

1. Fork the repo and create a branch from `master`
2. Make your changes — keep commits focused and descriptive
3. Make sure `cargo build`, `cargo test`, and `cargo clippy` all pass
4. Open a PR with a clear title and description of what you changed and why
5. Link any related issues (e.g. `Closes #42`)

Keep PRs small and scoped to one thing — it's much easier to review and merge.

---

## Code Style

- Format with `cargo fmt` before committing
- No unnecessary `unwrap()` on user-facing paths — use proper error handling
- Prefer clarity over cleverness
- No comments explaining *what* the code does — only *why* when it's non-obvious

---

## Project Structure

```
src/
  main.rs                  — CLI entry point
  lib.rs                   — Public API (gather_system_info)
  config/                  — Config loading and defaults
  modules/
    ascii.rs               — Built-in distro ASCII art
    colors.rs              — Distro color schemes
    helper.rs              — CLI argument definitions
    linux/                 — Linux-specific info collectors
    windows/               — Windows-specific info collectors
```

---

## Need Help?

Open a [discussion](https://github.com/drunkleen/leenfetch/discussions) or drop a comment on any issue. We're happy to help you get oriented.
