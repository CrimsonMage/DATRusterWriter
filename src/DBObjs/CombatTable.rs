use std::any::Any;

use crate::{
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::DBObj::{DBObj, DBObjBase},
};

pub const COMBAT_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "CombatTable", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::CombatTable, header_flags: DBObjHeaderFlags::HasId, first_id: 0x30000000, last_id: 0x3000FFFF, mask_id: 0x00000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatTable { pub base: DBObjBase }

impl DBObj for CombatTable {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::CombatTable }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}
impl IUnpackable for CombatTable { fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool { let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId); true } }
impl IPackable for CombatTable { fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool { let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId); true } }
impl IDBObj for CombatTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &COMBAT_TABLE_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::CombatTable }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
