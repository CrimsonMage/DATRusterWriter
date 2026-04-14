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

pub const MATERIAL_INSTANCE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "MaterialInstance",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::MaterialInstance,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x18000000,
    last_id: 0x18FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MaterialInstance {
    pub base: DBObjBase,
    pub material_id: u32,
    pub material_type: u32,
    pub modifier_refs: Vec<u32>,
    pub allow_stencil_shadows: bool,
    pub want_discard_geometry: bool,
}

impl DBObj for MaterialInstance {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::MaterialInstance
    }
    fn id(&self) -> u32 {
        self.base.id
    }
    fn set_id(&mut self, id: u32) {
        self.base.id = id;
    }
    fn data_category(&self) -> u32 {
        self.base.data_category
    }
    fn set_data_category(&mut self, data_category: u32) {
        self.base.data_category = data_category;
    }
}

impl IUnpackable for MaterialInstance {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.material_id = reader.read_u32();
        self.material_type = reader.read_u32();
        let count = reader.read_u32() as usize;
        self.modifier_refs.clear();
        self.modifier_refs.reserve(count);
        for _ in 0..count {
            self.modifier_refs.push(reader.read_u32());
        }
        self.allow_stencil_shadows = reader.read_bool(1);
        self.want_discard_geometry = reader.read_bool(1);
        true
    }
}

impl IPackable for MaterialInstance {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.material_id);
        writer.write_u32(self.material_type);
        writer.write_u32(self.modifier_refs.len() as u32);
        for item in &self.modifier_refs {
            writer.write_u32(*item);
        }
        writer.write_bool(self.allow_stencil_shadows, 1);
        writer.write_bool(self.want_discard_geometry, 1);
        true
    }
}

impl IDBObj for MaterialInstance {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &MATERIAL_INSTANCE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::MaterialInstance
    }
    fn id(&self) -> u32 {
        self.base.id
    }
    fn set_id(&mut self, id: u32) {
        self.base.id = id;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
