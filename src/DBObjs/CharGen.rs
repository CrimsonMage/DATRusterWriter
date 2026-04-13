use std::any::Any;

use crate::{
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::{DBObj::{DBObj, DBObjBase}, QualifiedDataId::QualifiedDataId, StartingArea::StartingArea, HashTable::HashTable, HeritageGroupCG::HeritageGroupCG},
};

pub const CHAR_GEN_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "CharGen", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::CharGen, header_flags: DBObjHeaderFlags::HasId, first_id: 0x0E000002, last_id: 0x0E000002, mask_id: 0x00000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharGen {
    pub base: DBObjBase,
    pub data_id: QualifiedDataId<CharGen>,
    pub starting_areas: Vec<StartingArea>,
    pub heritage_groups: HashTable<u32, HeritageGroupCG>,
}

impl DBObj for CharGen {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::CharGen }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for CharGen {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.data_id = reader.read_item::<QualifiedDataId<CharGen>>();
        let count = reader.read_compressed_uint() as usize;
        self.starting_areas.clear();
        for _ in 0..count { self.starting_areas.push(reader.read_item::<StartingArea>()); }
        self.heritage_groups = reader.read_item::<HashTable<u32, HeritageGroupCG>>();
        true
    }
}

impl IPackable for CharGen {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.data_id);
        writer.write_compressed_uint(self.starting_areas.len() as u32);
        for item in &self.starting_areas { writer.write_item(item); }
        writer.write_item(&self.heritage_groups);
        true
    }
}

impl IDBObj for CharGen {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &CHAR_GEN_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::CharGen }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
