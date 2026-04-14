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

Most callers should start with:

- `dat_ruster_writer::databases::ClientDatStore`
- `dat_ruster_writer::prelude::*`

`ClientDatStore` is the top-layer API for loading from all four DATs through one handle.

Primary load API:

- `store.load::<T>(id)`
- `store.load_cached::<T>(id)`
- `store.load_ids::<T>()`

Common convenience methods:

- `store.region()`
- `store.master_property()`
- `store.layout(id)`
- `store.setup(id)`
- `store.env_cell(id)`
- `store.land_block(id)`
- `store.land_block_info(id)`
- `store.palette(id)`
- `store.render_texture(id)`
- `store.font(id)`
- `store.string_table(id)`
- `store.gfx_obj(id)`
- `store.animation(id)`
- `store.motion_table(id)`

Curated common asset exports:

- `dat_ruster_writer::assets::*`

Lower-level database access is still available when needed:

- `dat_ruster_writer::databases::DatCollection`
- `dat_ruster_writer::databases::DatDatabase`
- `dat_ruster_writer::databases::PortalDatabase`
- `dat_ruster_writer::databases::CellDatabase`
- `dat_ruster_writer::databases::LocalDatabase`

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

Open all four DATs through one store:

```rust
use dat_ruster_writer::{
    databases::ClientDatStore,
    Options::DatAccessType::DatAccessType,
};

let store = ClientDatStore::open(
    "C:\\Games\\AsheronsCall",
    DatAccessType::Read,
)?;
# Ok::<(), std::io::Error>(())
```

Load a typed object through one generic API:

```rust
use dat_ruster_writer::{
    databases::ClientDatStore,
    DBObjs::Palette::Palette,
    Options::DatAccessType::DatAccessType,
};

let store = ClientDatStore::open(
    "C:\\Games\\AsheronsCall",
    DatAccessType::Read,
)?;

let palette = store.load::<Palette>(0x0400_0010)?;
# let _ = palette;
# Ok::<(), std::io::Error>(())
```

Load common client assets through convenience methods:

```rust
use dat_ruster_writer::{
    assets::{LayoutDesc, Region, Setup},
    databases::ClientDatStore,
    Options::DatAccessType::DatAccessType,
};

let store = ClientDatStore::open(
    "C:\\Games\\AsheronsCall",
    DatAccessType::Read,
)?;

let region: Option<Region> = store.region()?;
let layout: Option<LayoutDesc> = store.layout(0x2100_0010)?;
let setup: Option<Setup> = store.setup(0x0200_0010)?;
# let _ = (region, layout, setup);
# Ok::<(), std::io::Error>(())
```

Inspect raw file metadata:

```rust
use dat_ruster_writer::{
    databases::ClientDatStore,
    Generated::Enums::DatFileType::DatFileType,
    Options::DatAccessType::DatAccessType,
};

let store = ClientDatStore::open(
    "C:\\Games\\AsheronsCall",
    DatAccessType::Read,
)?;

let entry = store.load_file_entry(DatFileType::Portal, 0x0400_0010)?;
# let _ = entry;
# Ok::<(), std::io::Error>(())
```

## Notes

- This crate is intentionally still close to the original ported layout.
- `ClientDatStore` is the preferred public entry point for new client code.
- `assets::*` provides a curated set of common DB object types for client-facing code.
- The higher-level grouped modules in `databases`, `io`, `btree`, and `prelude` are the preferred public entry points for new code.
