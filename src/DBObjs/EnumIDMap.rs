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
        IntrusiveHashTable::IntrusiveHashTable,
        PStringBase::PStringBase,
    },
};

pub const ENUM_ID_MAP_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "EnumIDMap",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::EnumIDMap,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x25000000,
    last_id: 0x25FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnumIDMap {
    pub base: DBObjBase,
    pub client_enum_to_id: IntrusiveHashTable<u32, u32>,
    pub client_enum_to_name: IntrusiveHashTable<u32, PStringBase<u8>>,
    pub server_enum_to_id: IntrusiveHashTable<u32, u32>,
    pub server_enum_to_name: IntrusiveHashTable<u32, PStringBase<u8>>,
}

impl DBObj for EnumIDMap {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::EnumIDMap }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for EnumIDMap {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.client_enum_to_id = reader.read_item::<IntrusiveHashTable<u32, u32>>();
        self.client_enum_to_name = reader.read_item::<IntrusiveHashTable<u32, PStringBase<u8>>>();
        self.server_enum_to_id = reader.read_item::<IntrusiveHashTable<u32, u32>>();
        self.server_enum_to_name = reader.read_item::<IntrusiveHashTable<u32, PStringBase<u8>>>();
        true
    }
}

impl IPackable for EnumIDMap {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.client_enum_to_id);
        writer.write_item(&self.client_enum_to_name);
        writer.write_item(&self.server_enum_to_id);
        writer.write_item(&self.server_enum_to_name);
        true
    }
}

impl IDBObj for EnumIDMap {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &ENUM_ID_MAP_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::EnumIDMap }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
