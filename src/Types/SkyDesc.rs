use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::DayGroup::DayGroup;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkyDesc {
    pub tick_size: f64,
    pub light_tick_size: f64,
    pub day_groups: Vec<DayGroup>,
}

impl IUnpackable for SkyDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.tick_size = reader.read_double();
        self.light_tick_size = reader.read_double();
        let count = reader.read_u32() as usize;
        self.day_groups.clear();
        for _ in 0..count {
            self.day_groups.push(reader.read_item::<DayGroup>());
        }
        true
    }
}

impl IPackable for SkyDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_double(self.tick_size);
        writer.write_double(self.light_tick_size);
        writer.write_u32(self.day_groups.len() as u32);
        for item in &self.day_groups {
            writer.write_item(item);
        }
        true
    }
}
