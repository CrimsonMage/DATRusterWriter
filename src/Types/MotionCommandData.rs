use std::collections::BTreeMap;

use crate::{Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable}, Types::MotionData::MotionData};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotionCommandData {
    pub motion_data: BTreeMap<i32, MotionData>,
}

impl IUnpackable for MotionCommandData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let num_commands = reader.read_u32() as usize;
        self.motion_data.clear();
        for _ in 0..num_commands {
            let key = reader.read_i32();
            let value = reader.read_item::<MotionData>();
            self.motion_data.insert(key, value);
        }
        true
    }
}

impl IPackable for MotionCommandData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.motion_data.len() as u32);
        for (key, value) in &self.motion_data {
            writer.write_i32(*key);
            let _ = value.pack(writer);
        }
        true
    }
}
