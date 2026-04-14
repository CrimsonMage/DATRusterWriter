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
    Types::{DBObj::{DBObj, DBObjBase}, GfxObjInfo::GfxObjInfo},
};

pub const GFX_OBJ_DEGRADE_INFO_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "GfxObjDegradeInfo",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::GfxObjDegradeInfo,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x11000000,
    last_id: 0x1100FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GfxObjDegradeInfo {
    pub base: DBObjBase,
    pub degrades: Vec<GfxObjInfo>,
}

impl DBObj for GfxObjDegradeInfo {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::GfxObjDegradeInfo }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for GfxObjDegradeInfo {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let count = reader.read_u32() as usize;
        self.degrades.clear();
        self.degrades.reserve(count);
        for _ in 0..count {
            self.degrades.push(reader.read_item::<GfxObjInfo>());
        }
        true
    }
}

impl IPackable for GfxObjDegradeInfo {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.degrades.len() as u32);
        for item in &self.degrades {
            writer.write_item(item);
        }
        true
    }
}

impl IDBObj for GfxObjDegradeInfo {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &GFX_OBJ_DEGRADE_INFO_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::GfxObjDegradeInfo }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
