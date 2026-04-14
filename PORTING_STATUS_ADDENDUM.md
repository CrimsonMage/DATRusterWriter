# Porting Status Addendum

Superseded by the repaired `PORTING_STATUS.md` on 2026-04-13. Kept only as a historical recovery note.

This addendum tracks progress made after `PORTING_STATUS.md` became unreadable on disk due to null-padded contents.
It is intended to preserve the current port history until the main tracker is repaired safely.

## Newly Ported Read-First DBObjs

| Original C# source | Rust port | Status | Notes |
| --- | --- | --- | --- |
| `Generated/DBObjs/Font.generated.cs` | `src/DBObjs/Font.rs` | Verified | Read-side port with typed `QualifiedDataId<Surface>` links and `FontCharDesc` list support. |
| `Generated/DBObjs/LanguageInfo.generated.cs` | `src/DBObjs/LanguageInfo.rs` | Verified | Full read-side local DBObj port using UTF-16 `PStringBase<u16>` strings and alignment matching the original. |
| `Generated/DBObjs/NameFilterTable.generated.cs` | `src/DBObjs/NameFilterTable.rs` | Verified | Read-side portal DBObj port over `HashTable<u32, NameFilterLanguageData>`. |

## Newly Ported Supporting Types

| Original C# source | Rust port | Status | Notes |
| --- | --- | --- | --- |
| `Generated/Types/FontCharDesc.generated.cs` | `src/Types/FontCharDesc.rs` | Verified | Full unpack/pack parity for glyph metrics. |
| `Generated/Types/NameFilterLanguageData.generated.cs` | `src/Types/NameFilterLanguageData.rs` | Verified | Full unpack/pack parity for naming rules and compound letter groups. |

## Infrastructure Updates

| Area | Rust file | Status | Notes |
| --- | --- | --- | --- |
| DBObj registration | `src/DBObjs/mod.rs` | Verified | Added `Font`, `LanguageInfo`, and `NameFilterTable`. |
| Type registration | `src/Types/mod.rs` | Verified | Added `FontCharDesc` and `NameFilterLanguageData`. |
| Attribute resolution | `src/Lib/DBObjAttributeCache.rs` | Verified | Added typed attribute entries for the newly ported DBObjs. |
| Synthetic typed tests | `tests/typed_dbobj_tests.rs` | Verified | Added attribute-cache and typed read coverage for `Font`, `LanguageInfo`, and `NameFilterTable`. |
| External retail validation | `tests/real_dat_validation_tests.rs` | Verified | Added live DAT coverage for `Font`, `LanguageInfo`, and `NameFilterTable`. |

## Validation Completed

- `cargo check --lib`
- `cargo test --test typed_dbobj_tests -- --nocapture`
- `cargo test --test real_dat_validation_tests -- --ignored --nocapture`
  - External path used only through `DAT_READER_WRITER_REAL_DAT_DIR`
  - Validated against retail DATs in `C:\Turbine\Asheron's call\`

## Remaining High-Value Read-First Targets

- `Generated/DBObjs/NameFilterTable.generated.cs` dependencies beyond the current table body if deeper helpers are needed later.
- `Generated/Types/StringInfo.generated.cs`
- `Generated/Types/StringInfoBaseProperty.generated.cs`
- `Generated/Types/MediaDesc*.generated.cs`
- Additional local/client-facing DBObjs near the new string/language path.
- Repair or replace the main `PORTING_STATUS.md` tracker safely so the single-source status document is usable again.
