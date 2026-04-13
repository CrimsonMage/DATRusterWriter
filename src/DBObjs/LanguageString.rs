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
        PStringBase::PStringBase,
    },
};

pub const LANGUAGE_STRING_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "LanguageString",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::LanguageString,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x31000000,
    last_id: 0x3100FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LanguageString {
    pub base: DBObjBase,
    pub value: PStringBase<u8>,
}

impl DBObj for LanguageString {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LanguageString
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

impl IUnpackable for LanguageString {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.value = reader.read_item::<PStringBase<u8>>();
        true
    }
}

impl IPackable for LanguageString {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.value);
        true
    }
}

impl IDBObj for LanguageString {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &LANGUAGE_STRING_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LanguageString
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
