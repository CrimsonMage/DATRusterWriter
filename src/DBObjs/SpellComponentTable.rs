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
        PackableHashTable::PackableHashTable,
        SpellComponentBase::SpellComponentBase,
    },
};

pub const SPELL_COMPONENT_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "SpellComponentTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::SpellComponentTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E00000F,
    last_id: 0x0E00000F,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpellComponentTable {
    pub base: DBObjBase,
    pub components: PackableHashTable<u32, SpellComponentBase>,
}

impl DBObj for SpellComponentTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::SpellComponentTable
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

impl IUnpackable for SpellComponentTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.components = reader.read_item::<PackableHashTable<u32, SpellComponentBase>>();
        true
    }
}

impl IPackable for SpellComponentTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.components);
        true
    }
}

impl IDBObj for SpellComponentTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &SPELL_COMPONENT_TABLE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::SpellComponentTable
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
