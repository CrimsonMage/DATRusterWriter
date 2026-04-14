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
        BaseProperty::BaseProperty,
        DBObj::{DBObj, DBObjBase},
    },
};

pub const DB_PROPERTIES_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "DBProperties",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::DBProperties,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x78000000,
    last_id: 0x7FFFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DBProperties {
    pub base: DBObjBase,
    pub properties: BTreeMap<u32, BaseProperty>,
}

impl DBObj for DBProperties {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::DBProperties
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

impl IUnpackable for DBProperties {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let _bucket_size = reader.read_byte();
        let count = reader.read_byte() as usize;
        self.properties.clear();
        for _ in 0..count {
            let key = reader.read_u32();
            let Some(value) = BaseProperty::unpack_generic(reader) else {
                return false;
            };
            self.properties.insert(key, value);
        }
        true
    }
}

impl IPackable for DBProperties {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_byte(0);
        writer.write_byte(self.properties.len() as u8);
        for (key, value) in &self.properties {
            writer.write_u32(*key);
            writer.write_item(value);
        }
        true
    }
}

impl IDBObj for DBProperties {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &DB_PROPERTIES_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::DBProperties
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
