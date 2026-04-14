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
        CloSubPalEffect::CloSubPalEffect,
        ClothingBaseEffect::ClothingBaseEffect,
        DBObj::{DBObj, DBObjBase},
    },
};

pub const CLOTHING_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "Clothing",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::ClothingTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x10000000,
    last_id: 0x1000FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Clothing {
    pub base: DBObjBase,
    pub clothing_base_effects: BTreeMap<u32, ClothingBaseEffect>,
    pub clothing_sub_pal_effects: BTreeMap<u32, CloSubPalEffect>,
}

impl DBObj for Clothing {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ClothingTable
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

impl IUnpackable for Clothing {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);

        let base_count = reader.read_u16() as usize;
        let _base_buckets = reader.read_u16();
        self.clothing_base_effects.clear();
        for _ in 0..base_count {
            self.clothing_base_effects
                .insert(reader.read_u32(), reader.read_item::<ClothingBaseEffect>());
        }

        let sub_pal_count = reader.read_u16() as usize;
        let _sub_pal_buckets = reader.read_u16();
        self.clothing_sub_pal_effects.clear();
        for _ in 0..sub_pal_count {
            self.clothing_sub_pal_effects
                .insert(reader.read_u32(), reader.read_item::<CloSubPalEffect>());
        }

        true
    }
}

impl IPackable for Clothing {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);

        writer.write_u16(self.clothing_base_effects.len() as u16);
        writer.write_u16(8);
        for (key, value) in &self.clothing_base_effects {
            writer.write_u32(*key);
            writer.write_item(value);
        }

        writer.write_u16(self.clothing_sub_pal_effects.len() as u16);
        writer.write_u16(32);
        for (key, value) in &self.clothing_sub_pal_effects {
            writer.write_u32(*key);
            writer.write_item(value);
        }

        true
    }
}

impl IDBObj for Clothing {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &CLOTHING_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ClothingTable
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
