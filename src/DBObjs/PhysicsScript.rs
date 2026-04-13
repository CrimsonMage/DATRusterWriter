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
    Types::{
        DBObj::{DBObj, DBObjBase},
        PhysicsScriptData::PhysicsScriptData,
    },
};

pub const PHYSICS_SCRIPT_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "PhysicsScript",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::PhysicsScript,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x33000000,
    last_id: 0x3300FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsScript {
    pub base: DBObjBase,
    pub script_data: Vec<PhysicsScriptData>,
}

impl DBObj for PhysicsScript {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::PhysicsScript
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

impl IUnpackable for PhysicsScript {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let count = reader.read_u32() as usize;
        self.script_data.clear();
        for _ in 0..count {
            self.script_data
                .push(reader.read_item::<PhysicsScriptData>());
        }
        true
    }
}

impl IPackable for PhysicsScript {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.script_data.len() as u32);
        for item in &self.script_data {
            writer.write_item(item);
        }
        true
    }
}

impl IDBObj for PhysicsScript {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &PHYSICS_SCRIPT_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::PhysicsScript
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
