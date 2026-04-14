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
        HashTable::HashTableKey,
        PackableHashTable::PackableHashTable,
        QualifiedDataId::QualifiedDataId,
    },
};

pub const BAD_DATA_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "BadDataTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::BadDataTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E00001A,
    last_id: 0x0E00001A,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BadDataTable {
    pub base: DBObjBase,
    pub bad_ids: PackableHashTable<QualifiedDataId<BadDataTable>, u32>,
}

impl DBObj for BadDataTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::BadDataTable
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

impl IUnpackable for BadDataTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let count = reader.read_u16() as usize;
        let bucket_size = reader.read_u16();
        self.bad_ids.bucket_size = bucket_size;
        self.bad_ids.entries.clear();
        for _ in 0..count {
            let key = reader.read_item::<QualifiedDataId<BadDataTable>>();
            let value = reader.read_u32();
            self.bad_ids.entries.insert(key, value);
        }
        true
    }
}

impl IPackable for BadDataTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u16(self.bad_ids.entries.len() as u16);
        writer.write_u16(self.bad_ids.bucket_size);
        let bucket_size = self.bad_ids.bucket_size.max(1) as u64;
        let mut items: Vec<_> = self.bad_ids.entries.iter().collect();
        items.sort_by_key(|(key, _)| key.hash_key() % bucket_size);
        for (key, value) in items {
            key.write_key(writer);
            writer.write_u32(*value);
        }
        true
    }
}

impl IDBObj for BadDataTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &BAD_DATA_TABLE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::BadDataTable
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
