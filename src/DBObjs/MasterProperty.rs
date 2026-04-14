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
        BasePropertyDesc::BasePropertyDesc,
        DBObj::{DBObj, DBObjBase},
        EnumMapperData::EnumMapperData,
    },
};

pub const MASTER_PROPERTY_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "MasterProperty",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::MasterProperty,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x39000000,
    last_id: 0x39FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MasterProperty {
    pub base: DBObjBase,
    pub enum_mapper: EnumMapperData,
    pub properties: BTreeMap<u32, BasePropertyDesc>,
}

impl DBObj for MasterProperty {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::MasterProperty
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

impl IUnpackable for MasterProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.enum_mapper = reader.read_item::<EnumMapperData>();
        let _bucket_size = reader.read_byte();
        let count = reader.read_compressed_uint() as usize;
        self.properties.clear();
        for _ in 0..count {
            let key = reader.read_u32();
            let value = reader.read_item::<BasePropertyDesc>();
            self.properties.insert(key, value);
        }
        true
    }
}

impl IPackable for MasterProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.enum_mapper);
        writer.write_byte(6);
        writer.write_compressed_uint(self.properties.len() as u32);
        for (key, value) in &self.properties {
            writer.write_u32(*key);
            writer.write_item(value);
        }
        true
    }
}

impl IDBObj for MasterProperty {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &MASTER_PROPERTY_ATTR
    }

    fn db_obj_type(&self) -> DBObjType {
        DBObjType::MasterProperty
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
