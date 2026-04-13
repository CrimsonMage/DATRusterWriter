use std::any::Any;

use crate::{
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::{ColorARGB::ColorARGB, DBObj::{DBObj, DBObjBase}},
};

pub const PALETTE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "Palette", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::Palette, header_flags: DBObjHeaderFlags::HasId, first_id: 0x04000000, last_id: 0x0400FFFF, mask_id: 0x04000000 };

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Palette {
    pub base: DBObjBase,
    pub colors: Vec<ColorARGB>,
}

impl DBObj for Palette {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Palette }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for Palette {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let num_colors = reader.read_i32().max(0) as usize;
        self.colors.clear();
        for _ in 0..num_colors { self.colors.push(reader.read_item::<ColorARGB>()); }
        true
    }
}
impl IPackable for Palette { fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool { let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId); writer.write_i32(self.colors.len() as i32); for c in &self.colors { let _=c.pack(writer);} true } }
impl IDBObj for Palette { fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self:Sized { &PALETTE_ATTR } fn db_obj_type(&self)->DBObjType{DBObjType::Palette} fn id(&self)->u32{self.base.id} fn set_id(&mut self,id:u32){self.base.id=id;} fn as_any(&self)->&dyn Any{self} }
