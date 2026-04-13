use std::{any::Any, collections::BTreeMap};

use crate::{
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType, MotionCommand::MotionCommand},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::{DBObj::{DBObj, DBObjBase}, MotionCommandData::MotionCommandData, MotionData::MotionData},
};

pub const MOTION_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "MotionTable", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::MotionTable, header_flags: DBObjHeaderFlags::HasId, first_id: 0x09000000, last_id: 0x0900FFFF, mask_id: 0x09000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotionTable {
    pub base: DBObjBase,
    pub default_style: MotionCommand,
    pub style_defaults: BTreeMap<MotionCommand, MotionCommand>,
    pub cycles: BTreeMap<i32, MotionData>,
    pub modifiers: BTreeMap<i32, MotionData>,
    pub links: BTreeMap<i32, MotionCommandData>,
}

impl DBObj for MotionTable {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::MotionTable }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for MotionTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.default_style = MotionCommand::from(reader.read_u32());
        let num_style_defaults = reader.read_u32() as usize;
        self.style_defaults.clear();
        for _ in 0..num_style_defaults { self.style_defaults.insert(MotionCommand::from(reader.read_u32()), MotionCommand::from(reader.read_u32())); }
        let num_cycles = reader.read_u32() as usize;
        self.cycles.clear();
        for _ in 0..num_cycles { let key = reader.read_i32(); let val = reader.read_item::<MotionData>(); self.cycles.insert(key, val); }
        let num_modifiers = reader.read_u32() as usize;
        self.modifiers.clear();
        for _ in 0..num_modifiers { let key = reader.read_i32(); let val = reader.read_item::<MotionData>(); self.modifiers.insert(key, val); }
        let num_links = reader.read_u32() as usize;
        self.links.clear();
        for _ in 0..num_links { let key = reader.read_i32(); let val = reader.read_item::<MotionCommandData>(); self.links.insert(key, val); }
        true
    }
}
impl IPackable for MotionTable { fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool { let _=self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId); writer.write_u32(self.default_style.0); writer.write_u32(self.style_defaults.len() as u32); for (k,v) in &self.style_defaults { writer.write_u32(k.0); writer.write_u32(v.0);} writer.write_u32(self.cycles.len() as u32); for (k,v) in &self.cycles { writer.write_i32(*k); let _=v.pack(writer);} writer.write_u32(self.modifiers.len() as u32); for (k,v) in &self.modifiers { writer.write_i32(*k); let _=v.pack(writer);} writer.write_u32(self.links.len() as u32); for (k,v) in &self.links { writer.write_i32(*k); let _=v.pack(writer);} true } }
impl IDBObj for MotionTable { fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self:Sized { &MOTION_TABLE_ATTR } fn db_obj_type(&self)->DBObjType{DBObjType::MotionTable} fn id(&self)->u32{self.base.id} fn set_id(&mut self,id:u32){self.base.id=id;} fn as_any(&self)->&dyn Any{self} }
