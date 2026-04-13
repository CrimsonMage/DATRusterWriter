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
        HashTable::HashTable,
        StringTableString::StringTableString,
    },
};

pub const STRING_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "StringTable",
    dat_file_type: DatFileType::Local,
    db_obj_type: DBObjType::StringTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x23000000,
    last_id: 0x24FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StringTable {
    pub base: DBObjBase,
    pub language: u32,
    pub strings: HashTable<u32, StringTableString>,
}

impl DBObj for StringTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::StringTable
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

impl IUnpackable for StringTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.language = reader.read_u32();
        self.strings = reader.read_item::<HashTable<u32, StringTableString>>();
        true
    }
}

impl IPackable for StringTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.language);
        writer.write_item(&self.strings);
        true
    }
}

impl IDBObj for StringTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &STRING_TABLE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::StringTable
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
