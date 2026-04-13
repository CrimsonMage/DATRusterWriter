use std::any::Any;

use crate::{
    DBObjs::Palette::Palette,
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::{DBObj::{DBObj, DBObjBase}, QualifiedDataId::QualifiedDataId},
};

pub const PAL_SET_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "PalSet", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::PalSet, header_flags: DBObjHeaderFlags::HasId, first_id: 0x0F000000, last_id: 0x0F00FFFF, mask_id: 0x00000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PalSet {
    pub base: DBObjBase,
    pub palettes: Vec<QualifiedDataId<Palette>>,
}

impl DBObj for PalSet {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::PalSet }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for PalSet {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let count = reader.read_u32() as usize;
        self.palettes.clear();
        for _ in 0..count { self.palettes.push(reader.read_item::<QualifiedDataId<Palette>>()); }
        true
    }
}

impl IPackable for PalSet {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.palettes.len() as u32);
        for item in &self.palettes { writer.write_item(item); }
        true
    }
}

impl IDBObj for PalSet {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &PAL_SET_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::PalSet }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
