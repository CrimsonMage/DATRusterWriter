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
    Types::DBObj::{DBObj, DBObjBase},
};

pub const RENDER_MATERIAL_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "RenderMaterial",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::RenderMaterial,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x16000000,
    last_id: 0x16FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderMaterial {
    pub base: DBObjBase,
}

impl DBObj for RenderMaterial {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::RenderMaterial }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for RenderMaterial {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        true
    }
}

impl IPackable for RenderMaterial {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        true
    }
}

impl IDBObj for RenderMaterial {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &RENDER_MATERIAL_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::RenderMaterial }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
