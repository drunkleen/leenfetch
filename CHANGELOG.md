# Changelog

All notable changes to LeenFetch are documented here.  
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions follow [Semantic Versioning](https://semver.org/).

---

## [1.2.2] — 2026-06-24

### Added
- 3 CI workflows: `ci.yml` (tests), `build-check.yml` (build check), `release.yml` (release)
- Community files: `SUPPORT.md`, `.github/FUNDING.yml`, updated issue/PR templates
- Install scripts: `scripts/install.sh` (Linux) and `scripts/install.ps1` (Windows) — one-liner install/update
- Static musl builds for x86_64 and ARM64 (fully static, glibc-independent)

### Changed
- `atty` replaced with `std::io::IsTerminal` (zero-dependency, stable since Rust 1.70)
- Windows FFI migrated from `winapi` to `windows-sys` 0.61.2
- `rust-version` set to `1.96.0` (latest)
- Updated all dependencies to latest compatible versions (`cargo update` — 13 package upgrades)
- GitHub Actions rewritten to 3-phase pipeline (build → package → release) — 9 artifacts per release
- Arch Linux packaging added: `.pkg.tar.zst` for both x86_64 and ARM64 (`makepkg` + manual format)
- Removed X11 feature: screen resolution now uses DRM only (no libX11/libXrandr linkage)
- Removed `refresh_rate` config option and `--refresh-rate` CLI flag (DRM doesn't expose Hz)
- `extract_cpu_model` refactored into a pure function; filesystem fallbacks moved to `get_cpu`
- README download links now point to latest release with auto-detection scripts
- Support section migrated from PayPal/crypto to Patreon and Buy Me a Coffee
- Windows release artifacts are `.zip` only (no standalone `.exe`)

### Fixed
- `EnvLock` test utility recovers from mutex poison instead of panicking (prevents cascading test failures)
- `test_hostname_from_env` corrected to reflect `gethostname()` syscall priority over `HOSTNAME` env var
- `returns_none_when_no_managers_found` test now properly isolates via `LEENFETCH_PKG_ROOT` env var instead of `PATH`
- RPM cross-build for aarch64 now uses `rpmbuild` with inline spec (bypasses `cargo-rpm` cross issues)
- Duplicated `cross build` call for aarch64 removed (saves ~30s per run)
- Windows MinGW install slimmed: `mingw-w64` → `gcc-mingw-w64-x86-64` (avoids no-output timeout)
- mold linker scoped to project build only, not `cargo install` (fixes linker crash)
- `REHL` typos in README fixed (`RHEL`)

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
