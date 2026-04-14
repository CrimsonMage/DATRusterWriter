use std::{any::Any, collections::BTreeMap};

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
        BuildingInfo::BuildingInfo,
        DBObj::{DBObj, DBObjBase},
        Stab::Stab,
    },
};

pub const LAND_BLOCK_INFO_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "LandBlockInfo",
    dat_file_type: DatFileType::Cell,
    db_obj_type: DBObjType::LandBlockInfo,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0,
    last_id: 0,
    mask_id: 0x0000FFFE,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LandBlockInfo {
    pub base: DBObjBase,
    pub num_cells: u32,
    pub objects: Vec<Stab>,
    pub buildings: Vec<BuildingInfo>,
    pub restriction_table: BTreeMap<u32, u32>,
}

impl DBObj for LandBlockInfo {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LandBlockInfo
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

impl IUnpackable for LandBlockInfo {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.num_cells = reader.read_u32();
        let num_objects = reader.read_u32() as usize;
        self.objects = (0..num_objects)
            .map(|_| reader.read_item::<Stab>())
            .collect();
        let num_buildings = reader.read_u16() as usize;
        let has_restrictions = reader.read_bool(2);
        self.buildings = (0..num_buildings)
            .map(|_| reader.read_item::<BuildingInfo>())
            .collect();
        self.restriction_table.clear();
        if has_restrictions {
            let count = reader.read_u16() as usize;
            let _num_buckets = reader.read_u16();
            for _ in 0..count {
                let key = reader.read_u32();
                let value = reader.read_u32();
                self.restriction_table.insert(key, value);
            }
        }
        true
    }
}

impl IPackable for LandBlockInfo {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.num_cells);
        writer.write_u32(self.objects.len() as u32);
        for object in &self.objects {
            writer.write_item(object);
        }
        writer.write_u16(self.buildings.len() as u16);
        writer.write_bool(!self.restriction_table.is_empty(), 2);
        for building in &self.buildings {
            writer.write_item(building);
        }
        if !self.restriction_table.is_empty() {
            writer.write_u16(self.restriction_table.len() as u16);
            writer.write_u16(8);
            for (key, value) in &self.restriction_table {
                writer.write_u32(*key);
                writer.write_u32(*value);
            }
        }
        true
    }
}

impl IDBObj for LandBlockInfo {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &LAND_BLOCK_INFO_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LandBlockInfo
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
