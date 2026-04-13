use std::{any::Any, collections::BTreeMap};

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        PlayScript::PlayScript,
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
        PhysicsScriptTableData::PhysicsScriptTableData,
    },
};

pub const PHYSICS_SCRIPT_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "PhysicsScriptTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::PhysicsScriptTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x34000000,
    last_id: 0x3400FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsScriptTable {
    pub base: DBObjBase,
    pub script_table: BTreeMap<PlayScript, PhysicsScriptTableData>,
}

impl DBObj for PhysicsScriptTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::PhysicsScriptTable
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

impl IUnpackable for PhysicsScriptTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let count = reader.read_u32() as usize;
        self.script_table.clear();
        for _ in 0..count {
            self.script_table.insert(
                PlayScript::from(reader.read_u32()),
                reader.read_item::<PhysicsScriptTableData>(),
            );
        }
        true
    }
}

impl IPackable for PhysicsScriptTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.script_table.len() as u32);
        for (key, value) in &self.script_table {
            writer.write_u32((*key).into());
            writer.write_item(value);
        }
        true
    }
}

impl IDBObj for PhysicsScriptTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &PHYSICS_SCRIPT_TABLE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::PhysicsScriptTable
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
