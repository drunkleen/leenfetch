# Changelog

All notable changes to LeenFetch are documented here.  
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions follow [Semantic Versioning](https://semver.org/).

---

## [1.2.1] — 2026-04-17

### Added
- Built-in ASCII art and color scheme for the `leenium` distro (`--ascii_distro leenium`)

### Changed
- Boolean CLI flags now use a single `--flag <true|false>` syntax instead of paired `--flag` / `--no-flag` flags

### Fixed
- `stdout` flush no longer panics on broken pipe — replaced `.unwrap()` with `let _`
- Config loading now returns a `Result` and falls back to defaults with an error message instead of panicking
- Mutex locks recover gracefully from poison instead of panicking
- Removed test suite entries that called a commented-out function (caused compile failure)
- Memory percent display now guards against division by zero when `MemTotal` is 0
- `UBUNTU_CODENAME=` value slice was off by one (`[17..]` → `[16..]`)
- Disk percentage clamped with `.round().clamp(0, 100)` before casting to `u8`
- CPU core count uses `try_into().unwrap_or(u32::MAX)` instead of a bare `as u32` cast

---

## [1.2.0] — 2025-10-25

### Added
- System info cache — results are cached for 5 seconds to avoid redundant computation on rapid successive calls
- Default config is now parsed once and cached at startup

### Changed
- Linux module probes rewritten to use direct system calls instead of spawning shell commands
- Module deduplication preserves original config order
- Progress bar segments avoid redundant allocations

### Fixed
- CI pipeline now uses `GITHUB_TOKEN` and makes release notes optional
- Windows builds correctly link `ws2_32` via MinGW and MSVC targets
- Cleanup of unused dependencies and DE module data

---

## [1.1.0] — 2025-08-??

### Added
- JSON output mode (`--format json`) for machine-readable output
- Remote fetching over SSH (`--ssh user@host`) — runs `leenfetch --format json` on the remote and renders locally
- SSH remote host uses its own distro ASCII art while layout comes from local config

### Changed
- CLI migrated to `clap`-based argument parsing
- `anyhow` added for ergonomic CLI error handling
- Install methods reprioritized and SSH/JSON usage documented in README and man page

### Fixed
- X11 dependency made optional
- Release pipeline binary path fixed for native x86_64 builds
- Raspberry Pi CPU/GPU detection improved; `vc4`/`v3d` classified as integrated

---

## [1.0.4] — 2025-??-??

### Added
- Windows safe mode (`AV-friendly`) to avoid false positives from antivirus tools
- RPM `.spec` file for Fedora/RHEL packaging

### Changed
- Windows modules significantly sped up by removing slow external tool calls

---

## [1.0.3] — 2025-??-??

### Added
- Expanded CLI overrides with documentation
- New tests for Windows modules

### Changed
- Migrated Rust edition from 2021 to 2024
- Windows execution time substantially reduced by avoiding slow external tools

### Fixed
- Default `config.jsonc` syntax corrected

---

## [1.0.2] — 2025-??-??

### Changed
- Parallelized module data collection with `rayon` — LeenFetch is now ~35% faster than fastfetch and ~95% faster than neofetch
- GPU info fetch reimplemented with more accurate model details

---

## [1.0.1] — 2025-??-??

### Fixed
- Windows function compatibility updates

---

## [1.0.0] — 2025-??-??

Initial stable release.

- Fast, minimal system info tool written in Rust
- Cross-platform: Linux and Windows
- Configurable via `config.jsonc`
- Built-in ASCII art for major distros
- Color block display, battery bar, disk usage bar
- Neofetch-compatible layout system
