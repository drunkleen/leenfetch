# AGENTS.md

Scope: `src/modules/` and all platform collector code.

This file supplements the repository root `AGENTS.md` and `src/AGENTS.md`.

## Module Design Rules
- Keep collectors direct, deterministic, and OS-appropriate.
- Prefer system interfaces and file reads over external commands.
- Keep Linux-only logic in `src/modules/linux/` and Windows-only logic in `src/modules/windows/`.
- Preserve module names and field mappings used by layout resolution.
- Keep shared helpers in the smallest sensible place.

## Output Stability
- Treat formatting changes as user-visible behavior.
- Keep ASCII art, distro colors, and enum parsing compatible unless the task explicitly changes them.
- Do not rename module keys, enum values, or layout field names without confirmation.

## Collector Rules
- Use graceful fallbacks when a system source is missing.
- Avoid process spawning unless there is a concrete fallback need.
- Keep Windows and Linux feature parity when practical, but do not force identical implementation details.
- Update tests with any collector logic change.

## Shared Utilities
- `utils.rs` should stay focused on ASCII, color, bar, and formatting helpers.
- `ascii.rs` and `colors.rs` should remain data-driven and easy to extend.
- `enums.rs` should keep parsing rules explicit and conservative.

## Verification
- Run module-specific tests where possible before broader checks.
- For collector changes, verify both happy-path and fallback behavior.

## Ask Before Changing
- Artifact-facing text or labels that affect output compatibility.
- New external command dependencies.
- Layout interpretation or module selection semantics.
