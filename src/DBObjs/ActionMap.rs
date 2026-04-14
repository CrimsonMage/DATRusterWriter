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
        ActionMapValue::ActionMapValue,
        DBObj::{DBObj, DBObjBase},
        InputsConflictsValue::InputsConflictsValue,
    },
};

pub const ACTION_MAP_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "ActionMap",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::ActionMap,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x26000000,
    last_id: 0x2600FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActionMap {
    pub base: DBObjBase,
    pub input_maps: BTreeMap<u32, BTreeMap<u32, ActionMapValue>>,
    pub string_table_id: u32,
    pub conflicting_maps: BTreeMap<u32, InputsConflictsValue>,
}

impl DBObj for ActionMap {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ActionMap
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

fn find_closest_number(target: usize, numbers: &[u8]) -> u8 {
    let mut closest = numbers[0];
    let mut min_difference = (target as i32 - numbers[0] as i32).abs();
    for num in numbers.iter().copied() {
        let difference = (target as i32 - num as i32).abs();
        if difference < min_difference {
            min_difference = difference;
            closest = num;
        }
    }
    closest
}

fn calculate_child_bucket_size(count: usize) -> u8 {
    if count == 21 {
        11
    } else {
        find_closest_number(count, &[11, 23, 47])
    }
}

impl IUnpackable for ActionMap {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);

        let _bucket_size = reader.read_byte();
        let num_input_maps = reader.read_compressed_uint() as usize;
        self.input_maps.clear();
        for _ in 0..num_input_maps {
            let key = reader.read_u32();
            let _child_bucket_size = reader.read_byte();
            let child_count = reader.read_compressed_uint() as usize;
            let mut child_map = BTreeMap::new();
            for _ in 0..child_count {
                let child_key = reader.read_u32();
                let child_val = reader.read_item::<ActionMapValue>();
                child_map.insert(child_key, child_val);
            }
            self.input_maps.insert(key, child_map);
        }

        self.string_table_id = reader.read_u32();

        let _conflicting_bucket_size = reader.read_byte();
        let conflicting_count = reader.read_compressed_uint() as usize;
        self.conflicting_maps.clear();
        for _ in 0..conflicting_count {
            let key = reader.read_u32();
            let value = reader.read_item::<InputsConflictsValue>();
            self.conflicting_maps.insert(key, value);
        }
        true
    }
}

impl IPackable for ActionMap {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);

        writer.write_byte(23);
        writer.write_compressed_uint(self.input_maps.len() as u32);
        for (key, child_map) in &self.input_maps {
            writer.write_u32(*key);
            writer.write_byte(calculate_child_bucket_size(child_map.len()));
            writer.write_compressed_uint(child_map.len() as u32);
            for (child_key, child_value) in child_map {
                writer.write_u32(*child_key);
                writer.write_item(child_value);
            }
        }

        writer.write_u32(self.string_table_id);
        writer.write_byte(1);
        writer.write_compressed_uint(self.conflicting_maps.len() as u32);
        for (key, value) in &self.conflicting_maps {
            writer.write_u32(*key);
            writer.write_item(value);
        }
        true
    }
}

impl IDBObj for ActionMap {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &ACTION_MAP_ATTR
    }

    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ActionMap
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
