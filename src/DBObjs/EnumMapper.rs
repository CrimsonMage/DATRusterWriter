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
        AutoGrowHashTable::AutoGrowHashTable,
        DBObj::{DBObj, DBObjBase},
        PStringBase::PStringBase,
    },
};

pub const ENUM_MAPPER_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "EnumMapper",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::EnumMapper,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x22000000,
    last_id: 0x22FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnumMapper {
    pub base: DBObjBase,
    pub base_enum_map: u32,
    pub id_to_string_map: AutoGrowHashTable<u32, PStringBase<u8>>,
}

impl DBObj for EnumMapper {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::EnumMapper
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

impl IUnpackable for EnumMapper {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.base_enum_map = reader.read_u32();
        self.id_to_string_map = reader.read_item::<AutoGrowHashTable<u32, PStringBase<u8>>>();
        true
    }
}

impl IPackable for EnumMapper {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.base_enum_map);
        writer.write_item(&self.id_to_string_map);
        true
    }
}

impl IDBObj for EnumMapper {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &ENUM_MAPPER_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::EnumMapper
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
