# AGENTS.md

Scope: `.github/` automation, release, and project meta files.

This file supplements the repository root `AGENTS.md`.

## Workflow Rules
- Keep CI and release changes deliberate and minimal.
- Preserve artifact names, paths, and release metadata unless the user explicitly requests a change.
- Treat workflow edits as production changes: avoid speculative rewrites.
- Prefer deterministic steps over clever shell logic.

## Release Discipline
- Do not change packaging formats, file names, or publish behavior without confirmation.
- Keep workflow-generated artifacts aligned with `README.md`, `CHANGELOG.md`, `PKGBUILD`, `install.sh`, `install.ps1`, and the man page.
- If a release step changes, update the corresponding docs and scripts in the same change.

## Safety Rules
- Do not add secrets, tokens, or credentials to the repository.
- Avoid broad workflow refactors unless the task is specifically about release engineering.
- Keep permissions as narrow as practical.

## Verification
- For workflow or release script changes, inspect the resulting artifact names and publish paths carefully.
- Confirm that release notes, checksums, and package outputs remain consistent.

## Ask Before Changing
- Release artifact naming.
- Publishing targets such as crates.io, GitHub Releases, or AUR behavior.
- Workflow permissions or new infrastructure requirements.
