use std::any::Any;

use crate::{
    Generated::Enums::{DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj, IPackable::IPackable, IUnpackable::IUnpackable}},
    Types::{DBObj::{DBObj, DBObjBase}, ObjectDesc::ObjectDesc},
};

pub const SCENE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute { rust_type_name: "Scene", dat_file_type: DatFileType::Portal, db_obj_type: DBObjType::Scene, header_flags: DBObjHeaderFlags::HasId, first_id: 0x12000000, last_id: 0x1200FFFF, mask_id: 0x00000000 };

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Scene {
    pub base: DBObjBase,
    pub objects: Vec<ObjectDesc>,
}

impl DBObj for Scene {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Scene }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for Scene {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let count = reader.read_u32() as usize;
        self.objects.clear();
        for _ in 0..count {
            self.objects.push(reader.read_item::<ObjectDesc>());
        }
        true
    }
}

impl IPackable for Scene {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.objects.len() as u32);
        for object in &self.objects {
            writer.write_item(object);
        }
        true
    }
}

impl IDBObj for Scene {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &SCENE_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::Scene }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
