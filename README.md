# dat_ruster_writer

`dat_ruster_writer` is a Rust port of the Asheron's Call `DatReaderWriter` library.

This crate provides public APIs for reading, writing, and inspecting Asheron's Call DAT files, including typed DB objects and lower-level DAT metadata access.

## Scope

This crate is for DAT file access, not for network protocol parsing.

Use it when you need to:

- open `client_portal.dat`, `client_cell_1.dat`, `client_local_English.dat`, or `client_highres.dat`
- read typed objects such as `Palette`, `Region`, `MasterProperty`, and other ported DB objects
- inspect raw DAT file entries and bytes
- write typed objects back into DAT files

## API Entry Points

Most callers should start with one of these:

- `dat_ruster_writer::databases::DatCollection`
- `dat_ruster_writer::databases::DatDatabase`
- `dat_ruster_writer::databases::PortalDatabase`
- `dat_ruster_writer::databases::CellDatabase`
- `dat_ruster_writer::databases::LocalDatabase`
- `dat_ruster_writer::prelude::*`

For lower-level access:

- `dat_ruster_writer::io`
- `dat_ruster_writer::btree`

The original direct-port module layout is still available for compatibility:

- `dat_ruster_writer::DBObjs`
- `dat_ruster_writer::Types`
- `dat_ruster_writer::Generated`
- `dat_ruster_writer::Options`
- `dat_ruster_writer::Lib`

## Examples

Open a DAT collection from a game directory:

```rust
use dat_ruster_writer::{
    databases::DatCollection,
    Options::DatAccessType::DatAccessType,
};

let collection = DatCollection::from_directory(
    "C:\\Games\\AsheronsCall",
    DatAccessType::Read,
)?;
# Ok::<(), std::io::Error>(())
```

Read a typed object:

```rust
use dat_ruster_writer::{
    databases::DatCollection,
    DBObjs::Palette::Palette,
    Options::DatAccessType::DatAccessType,
};

let collection = DatCollection::from_directory(
    "C:\\Games\\AsheronsCall",
    DatAccessType::Read,
)?;

let palette = collection.try_get::<Palette>(0x0400_0010)?;
# let _ = palette;
# Ok::<(), std::io::Error>(())
```

Inspect raw file metadata:

```rust
use dat_ruster_writer::{
    databases::DatCollection,
    Generated::Enums::DatFileType::DatFileType,
    Options::DatAccessType::DatAccessType,
};

let collection = DatCollection::from_directory(
    "C:\\Games\\AsheronsCall",
    DatAccessType::Read,
)?;

let entry = collection.try_get_file_entry(DatFileType::Portal, 0x0400_0010)?;
# let _ = entry;
# Ok::<(), std::io::Error>(())
```

## Notes

- This crate is intentionally still close to the original ported layout.
- The higher-level grouped modules in `databases`, `io`, `btree`, and `prelude` are the preferred public entry points for new code.
