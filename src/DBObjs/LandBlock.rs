use std::any::Any;

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
    },
    Lib::{
        Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj,
            IPackable::IPackable, IUnpackable::IUnpackable,
        },
    },
    Types::{
        DBObj::{DBObj, DBObjBase},
        TerrainInfo::TerrainInfo,
    },
};

pub const LAND_BLOCK_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "LandBlock",
    dat_file_type: DatFileType::Cell,
    db_obj_type: DBObjType::LandBlock,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0,
    last_id: 0,
    mask_id: 0x0000FFFF,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandBlock {
    pub base: DBObjBase,
    pub has_objects: bool,
    pub terrain: [TerrainInfo; 81],
    pub height: [u8; 81],
}

impl Default for LandBlock {
    fn default() -> Self {
        Self {
            base: DBObjBase::default(),
            has_objects: false,
            terrain: [TerrainInfo::default(); 81],
            height: [0; 81],
        }
    }
}

impl DBObj for LandBlock {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LandBlock
    }
    fn id(&self) -> u32 {
        self.base.id
    }
    fn set_id(&mut self, id: u32) {
        self.base.id = id;
    }
    fn data_category(&self) -> u32 {
        self.base.data_category
    }
    fn set_data_category(&mut self, data_category: u32) {
        self.base.data_category = data_category;
    }
}

impl IUnpackable for LandBlock {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.has_objects = reader.read_bool(4);
        for terrain in &mut self.terrain {
            *terrain = reader.read_item::<TerrainInfo>();
        }
        for height in &mut self.height {
            *height = reader.read_byte();
        }
        reader.align(4);
        true
    }
}

impl IPackable for LandBlock {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_bool(self.has_objects, 4);
        for terrain in &self.terrain {
            writer.write_item(terrain);
        }
        for height in &self.height {
            writer.write_byte(*height);
        }
        writer.align(4);
        true
    }
}

impl IDBObj for LandBlock {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &LAND_BLOCK_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LandBlock
    }
    fn id(&self) -> u32 {
        self.base.id
    }
    fn set_id(&mut self, id: u32) {
        self.base.id = id;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
