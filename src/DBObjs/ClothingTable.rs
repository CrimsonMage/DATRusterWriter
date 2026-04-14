use std::any::Any;

use crate::{
    DBObjs::Setup::Setup,
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::{CloSubPalEffect::CloSubPalEffect, ClothingBaseEffect::ClothingBaseEffect, DBObj::{DBObj, DBObjBase}, PackableHashTable::PackableHashTable, QualifiedDataId::QualifiedDataId},
};

pub const CLOTHING_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "ClothingTable", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::ClothingTable, header_flags: DBObjHeaderFlags::HasId, first_id: 0x10000000, last_id: 0x1000FFFF, mask_id: 0x00000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClothingTable {
    pub base: DBObjBase,
    pub clothing_base_effects: PackableHashTable<QualifiedDataId<Setup>, ClothingBaseEffect>,
    pub clothing_sub_pal_effects: PackableHashTable<u32, CloSubPalEffect>,
}

impl DBObj for ClothingTable {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::ClothingTable }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for ClothingTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.clothing_base_effects = reader.read_item::<PackableHashTable<QualifiedDataId<Setup>, ClothingBaseEffect>>();
        self.clothing_sub_pal_effects = reader.read_item::<PackableHashTable<u32, CloSubPalEffect>>();
        true
    }
}

impl IPackable for ClothingTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.clothing_base_effects);
        writer.write_item(&self.clothing_sub_pal_effects);
        true
    }
}

impl IDBObj for ClothingTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &CLOTHING_TABLE_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::ClothingTable }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
