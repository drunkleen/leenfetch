# AGENTS.md

Operating standard for AI agents working in this repository.

## Purpose
- `leenfetch` is a Rust CLI and library for system information display.
- The binary entry point is `src/main.rs`.
- The reusable API lives in `src/lib.rs` and exposes `gather_system_info`, cache helpers, and `SystemInfo`.
- User configuration is a single JSONC file at `~/.config/leenfetch/config.jsonc`.
- Platform collectors are split by OS under `src/modules/linux/` and `src/modules/windows/`.

## Decision Hierarchy
When work is ambiguous, follow this order:
1. User instructions in the current conversation.
2. Repository-specific instructions in this file.
3. Existing code, docs, and established patterns in the repo.
4. Minimal change that preserves current behavior.

If a choice materially changes behavior, API shape, config schema, packaging, or docs, pause and confirm with the user unless the request explicitly authorizes the change.

Scoped `AGENTS.md` files may exist in subdirectories. They add narrower rules for that subtree and take precedence over this root file within their scope.

## Non-Negotiables
- Make the smallest correct change.
- Read the relevant code and docs before editing.
- Preserve public behavior unless the user asks otherwise.
- Do not revert or overwrite unrelated workspace changes.
- Use ASCII by default.
- Do not introduce new dependencies unless there is a concrete need.
- Do not add compatibility shims unless there is a shipped consumer or persisted-data reason.

## Architecture Map
- `src/main.rs`: CLI flow, config initialization, SSH handling, JSON output, rendering.
- `src/core/mod.rs`: module collection, data gathering, layout rendering, ASCII/color selection.
- `src/core/data.rs`: transient display-oriented data model.
- `src/system_info.rs`: stable serialized JSON model and conversions.
- `src/config/mod.rs` and `src/config/settings.rs`: config loading, defaults, schema, parsing.
- `src/modules/helper.rs`: clap args, overrides, help text, layout filtering.
- `src/modules/utils.rs`: ASCII loading, palette generation, colorization, bars.
- `src/modules/ascii.rs` and `src/modules/colors.rs`: distro assets.
- `src/modules/linux/` and `src/modules/windows/`: OS-specific collectors and tests.

## Runtime Flow
1. Parse CLI args.
2. Handle early exits: `--help`, `--list-options`, `--init`, `--reinit`, `--print_config`.
3. Load config or built-in defaults.
4. Apply CLI overrides to flags and layout.
5. Read stdin if piped.
6. If `--ssh` is set, fetch remote JSON and render locally.
7. Otherwise gather local system data and render pretty or JSON output.

## Implementation Standards
- Prefer direct system reads over spawning subprocesses.
- Keep platform-specific code in the matching OS tree.
- Preserve config key names, CLI names, and layout semantics unless a rename is explicitly requested.
- Keep JSON output stable; changes to `SystemInfo` are a compatibility decision.
- Treat layout resolution as field-name driven behavior in `src/core/mod.rs`.
- Avoid panic paths in user-facing code; prefer `Result`, `Option`, or graceful fallback.
- Keep helper functions small and local unless reuse is obvious.
- Use comments only when the reason is non-obvious.

## Editing Rules
- Use `apply_patch` for manual edits.
- Do not use destructive git commands like `git reset --hard` or `git checkout --`.
- Do not amend commits unless explicitly requested.
- Ignore unrelated dirty worktree changes unless they block the task.
- Keep changes scoped to the task; avoid opportunistic refactors.

## Verification Gates
- Default checks: `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`.
- Use the narrowest useful verification first when the change is local.
- If you touch CLI behavior, config parsing, packaging, or release output, verify the relevant docs/scripts too.
- If tests fail because of pre-existing issues, report that clearly and do not hide it.

## Packaging And Release
- Linux installer: `install.sh`.
- Windows installer: `install.ps1`.
- Arch packaging: `PKGBUILD`.
- Man page source: `leenfetch.1.md`; generated output: `leenfetch.1`.
- CI/release automation: `.github/workflows/*.yml`.

## Documentation Expectations
- Keep `README.md`, `leenfetch.1.md`, and release/installer scripts aligned with CLI or config changes.
- If behavior changes, update the user-facing documentation in the same change.
- Prefer concrete examples over vague prose.

## Testing Expectations
- Add or update tests when changing parsing, rendering, collectors, or serialization.
- Prefer deterministic tests with isolated fixtures or temp directories.
- For env-sensitive tests, use the repo test utilities and restore state.
- For platform-specific logic, keep tests close to the implementation.

## When To Ask The User
- Before renaming public config keys, CLI flags, or JSON fields.
- Before changing packaging formats or release artifact names.
- Before broad refactors that span multiple subsystems.
- Before adding new dependencies or OS-level requirements.

## If Unsure
- Read the existing implementation first.
- Match the repo’s current style and conventions.
- Choose the simplest correct production-ready option.
