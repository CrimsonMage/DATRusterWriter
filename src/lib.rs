#![allow(non_snake_case)]

#[path = "CellDatabase.rs"]
pub mod CellDatabase;
pub mod DBObjs;
#[path = "DatCollection.rs"]
pub mod DatCollection;
#[path = "DatDatabase.rs"]
pub mod DatDatabase;
pub mod Enums;
pub mod Generated;
#[path = "LocalDatabase.rs"]
pub mod LocalDatabase;
pub mod Options;
#[path = "PortalDatabase.rs"]
pub mod PortalDatabase;
pub mod Types;
pub mod client;
#[path = "Lib/mod.rs"]
pub mod dat_lib;

pub mod databases {
    pub use crate::CellDatabase::CellDatabase;
    pub use crate::DatCollection::DatCollection;
    pub use crate::DatDatabase::DatDatabase;
    pub use crate::LocalDatabase::LocalDatabase;
    pub use crate::PortalDatabase::PortalDatabase;
    pub use crate::client::ClientDatStore;
}

pub mod assets {
    pub use crate::DBObjs::{
        Animation::Animation, EnvCell::EnvCell, Font::Font, GfxObj::GfxObj, LandBlock::LandBlock,
        LandBlockInfo::LandBlockInfo, LayoutDesc::LayoutDesc, MasterProperty::MasterProperty,
        MotionTable::MotionTable, Palette::Palette, Region::Region, RenderTexture::RenderTexture,
        Setup::Setup, StringTable::StringTable,
    };
}

// Curated low-level entry points for callers that need raw DAT access without
// depending on the full direct-port module tree.
pub mod io {
    pub use crate::dat_lib::IO::DatBTree::DatBTreeFile::DatBTreeFile;
    pub use crate::dat_lib::IO::DatBTree::DatBTreeFileFlags::DatBTreeFileFlags;
    pub use crate::dat_lib::IO::DatBinReader::DatBinReader;
    pub use crate::dat_lib::IO::DatBinWriter::DatBinWriter;
    pub use crate::dat_lib::IO::DatHeader::DatHeader;
    pub use crate::dat_lib::IO::IDBObj::IDBObj;
    pub use crate::dat_lib::IO::IPackable::IPackable;
    pub use crate::dat_lib::IO::IUnpackable::IUnpackable;
}

// B-tree types are exposed separately so callers can inspect file metadata
// without reaching through internal allocator modules.
pub mod btree {
    pub use crate::dat_lib::IO::DatBTree::DatBTreeFile::DatBTreeFile;
    pub use crate::dat_lib::IO::DatBTree::DatBTreeFileFlags::DatBTreeFileFlags;
    pub use crate::dat_lib::IO::DatBTree::DatBTreeNode::DatBTreeNode;
    pub use crate::dat_lib::IO::DatBTree::DatBTreeReaderWriter::DatBTreeReaderWriter;
}

pub mod prelude {
    pub use crate::assets::{
        Animation, EnvCell, Font, GfxObj, LandBlock, LandBlockInfo, LayoutDesc, MasterProperty,
        MotionTable, Palette, Region, RenderTexture, Setup, StringTable,
    };
    pub use crate::dat_lib::IO::IDBObj::IDBObj;
    pub use crate::dat_lib::IO::IPackable::IPackable;
    pub use crate::dat_lib::IO::IUnpackable::IUnpackable;
    pub use crate::databases::{
        CellDatabase, ClientDatStore, DatCollection, DatDatabase, LocalDatabase, PortalDatabase,
    };
}

// Keep the direct-port alias available for compatibility with existing code.
pub use dat_lib as Lib;
