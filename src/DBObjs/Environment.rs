use std::{any::Any, collections::BTreeMap};

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
        CellStruct::CellStruct,
        DBObj::{DBObj, DBObjBase},
    },
};

pub const ENVIRONMENT_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "Environment",
    dat_file_type: DatFileType::Cell,
    db_obj_type: DBObjType::Environment,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0,
    last_id: 0,
    mask_id: 0x0D000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Environment {
    pub base: DBObjBase,
    pub cells: BTreeMap<u32, CellStruct>,
}

impl DBObj for Environment {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::Environment
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

impl IUnpackable for Environment {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let cell_count = reader.read_u32() as usize;
        self.cells.clear();
        for _ in 0..cell_count {
            let key = reader.read_u32();
            let value = reader.read_item::<CellStruct>();
            self.cells.insert(key, value);
        }
        true
    }
}

impl IPackable for Environment {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.cells.len() as u32);
        for (key, value) in &self.cells {
            writer.write_u32(*key);
            writer.write_item(value);
        }
        true
    }
}

impl IDBObj for Environment {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &ENVIRONMENT_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::Environment
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
