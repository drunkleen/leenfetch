# AGENTS.md

Scope: all Rust source under `src/`.

This file supplements the repository root `AGENTS.md`.

## Source Code Standard
- Keep changes minimal and localized.
- Prefer small, composable functions only when reuse is real.
- Avoid opportunistic refactors outside the task.
- Preserve public behavior unless the user explicitly asks to change it.
- Do not add dependencies without a concrete need and user-visible benefit.

## Rust Engineering Rules
- Favor direct system reads over shelling out.
- Keep error handling user-safe: return `Result`/`Option` or fall back gracefully.
- Avoid panics in user-facing code.
- Keep conversions and serialization stable, especially in `src/system_info.rs`.
- Treat config parsing and layout resolution as compatibility-sensitive code.

## Testing Rules
- Add or update tests when touching parsing, rendering, collectors, caching, or serialization.
- Prefer deterministic tests with isolated fixtures or temp directories.
- Keep tests close to the code they verify.
- For env-sensitive tests, use the repo test utilities.

## File-Specific Expectations
- `src/main.rs`: CLI behavior, early exits, SSH, output routing, config application.
- `src/lib.rs`: stable library API and cache behavior.
- `src/core/`: module collection, layout rendering, ASCII/color resolution.
- `src/config/`: config schema, loading, defaults, and file lifecycle.
- `src/modules/`: shared helpers, enums, formatting, distro assets, OS-specific collectors.

## Verification
- Use the narrowest useful command first.
- For source changes, prefer `cargo test`, then `cargo fmt --check`, then `cargo clippy -- -D warnings` when relevant.

## Ask Before Changing
- Public function signatures.
- `SystemInfo` fields or JSON behavior.
- CLI flags, config keys, or layout semantics.
- Platform-specific behavior that could affect output compatibility.
