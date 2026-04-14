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
        NameFilterLanguageData::NameFilterLanguageData,
    },
};

pub const NAME_FILTER_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "NameFilterTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::NameFilterTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E000020,
    last_id: 0x0E000020,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NameFilterTable {
    pub base: DBObjBase,
    pub language_data: HashTable<u32, NameFilterLanguageData>,
}

impl DBObj for NameFilterTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::NameFilterTable
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

impl IUnpackable for NameFilterTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.language_data = reader.read_item::<HashTable<u32, NameFilterLanguageData>>();
        true
    }
}

impl IPackable for NameFilterTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.language_data);
        true
    }
}

impl IDBObj for NameFilterTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &NAME_FILTER_TABLE_ATTR
    }

    fn db_obj_type(&self) -> DBObjType {
        DBObjType::NameFilterTable
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
