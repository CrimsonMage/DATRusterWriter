# Porting Rules

This repository is a Rust crate port of `C:\Repo\NewAC_Client\vitaeum\ref\DatReaderWriter`.

## Required Tracking Rules

- Every meaningful port must update `PORTING_STATUS.md` in the same work block.
- Every original C# source area must have an obvious Rust destination, even if it is still pending.
- When a file is ported, the status document must record the original source path and the Rust destination path.
- When a file is only scaffolded or partially ported, the status document must mark it as `Scaffolded` or `Partial`, not `Ported`.
- If a Rust file replaces multiple C# files or collapses helper abstractions, note that explicitly in the tracker.
- If a C# file has no Rust equivalent yet, it remains listed as `Pending`.
- Tests should be tracked separately from implementation files.
- Generated C# sources must still be tracked individually or by well-defined generated groups.

## Porting Conventions

- Preserve the reference project layout as closely as Rust allows.
- Prefer mirrored folders under `src/` using the same major grouping names:
- `DBObjs`
- `Enums`
- `Generated`
- `Lib`
- `Options`
- `Types`
- Keep Rust file names close to the source names when practical.
- Treat the Rust target as a reusable crate, not a standalone executable rewrite.
- Keep a running note of blockers, intentional deviations, and unported dependencies.

## Status Labels

- `Pending`: no Rust implementation exists yet
- `Scaffolded`: folder/module/file exists but logic is not meaningfully ported yet
- `Partial`: some logic is ported, but the source file is not feature-complete
- `Ported`: logic is substantially ported and builds in the crate
- `Verified`: ported and backed by Rust tests or direct validation

## Minimum Update Format

Each progress update in `PORTING_STATUS.md` should include:

- Source path
- Rust path
- Status
- Notes

## Review Rule

Before closing any chunk of work, verify that:

- new Rust files are reflected in `PORTING_STATUS.md`
- remaining source files are still represented in the backlog
- any partial ports clearly say what is missing
