use std::any::Any;

use crate::{
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType, SkillId::SkillId},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::{DBObj::{DBObj, DBObjBase}, PackableHashTable::PackableHashTable, SkillBase::SkillBase},
};

pub const SKILL_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "SkillTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::SkillTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E000004,
    last_id: 0x0E000004,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillTable {
    pub base: DBObjBase,
    pub skills: PackableHashTable<SkillId, SkillBase>,
}

impl DBObj for SkillTable {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::SkillTable }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for SkillTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.skills = reader.read_item::<PackableHashTable<SkillId, SkillBase>>();
        true
    }
}

impl IPackable for SkillTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.skills);
        true
    }
}

impl IDBObj for SkillTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &SKILL_TABLE_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::SkillTable }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
