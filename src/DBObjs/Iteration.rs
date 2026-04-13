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
    Types::DBObj::{DBObj, DBObjBase},
};

pub const ITERATION_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "Iteration",
    dat_file_type: DatFileType::Undefined,
    db_obj_type: DBObjType::Iteration,
    header_flags: DBObjHeaderFlags::None,
    first_id: 0xFFFF0001,
    last_id: 0xFFFF0001,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Iteration {
    pub base: DBObjBase,
    pub current_iteration: i32,
    pub iterations: BTreeMap<i32, i32>,
}

impl DBObj for Iteration {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::None
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::Iteration
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

impl IUnpackable for Iteration {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::None);
        self.current_iteration = reader.read_i32();
        let mut num_iterations = self.current_iteration;
        self.iterations.clear();
        while num_iterations > 0 {
            let consecutive_iterations = reader.read_i32();
            let starting_iteration = reader.read_i32();
            self.iterations
                .insert(starting_iteration, consecutive_iterations);
            num_iterations += consecutive_iterations;
        }
        if self.base.id == 0 {
            self.base.id = 0xFFFF0001;
        }
        true
    }
}

impl IPackable for Iteration {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::None);
        writer.write_i32(self.current_iteration);
        writer.write_i32(-self.current_iteration);
        writer.write_i32(1);
        true
    }
}

impl IDBObj for Iteration {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &ITERATION_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::Iteration
    }
    fn id(&self) -> u32 {
        if self.base.id == 0 {
            0xFFFF0001
        } else {
            self.base.id
        }
    }
    fn set_id(&mut self, id: u32) {
        self.base.id = id;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
