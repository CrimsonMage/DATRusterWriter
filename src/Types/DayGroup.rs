use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::{AC1LegacyString::AC1LegacyString, SkyObject::SkyObject, SkyTimeOfDay::SkyTimeOfDay};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DayGroup {
    pub chance_of_occur: f32,
    pub day_name: AC1LegacyString,
    pub sky_objects: Vec<SkyObject>,
    pub sky_time: Vec<SkyTimeOfDay>,
}

impl IUnpackable for DayGroup {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.chance_of_occur = reader.read_single();
        self.day_name = reader.read_item::<AC1LegacyString>();
        reader.align(4);
        let object_count = reader.read_u32() as usize;
        self.sky_objects.clear();
        for _ in 0..object_count {
            self.sky_objects.push(reader.read_item::<SkyObject>());
        }
        let time_count = reader.read_u32() as usize;
        self.sky_time.clear();
        for _ in 0..time_count {
            self.sky_time.push(reader.read_item::<SkyTimeOfDay>());
        }
        true
    }
}

impl IPackable for DayGroup {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_single(self.chance_of_occur);
        writer.write_item(&self.day_name);
        writer.align(4);
        writer.write_u32(self.sky_objects.len() as u32);
        for item in &self.sky_objects {
            writer.write_item(item);
        }
        writer.write_u32(self.sky_time.len() as u32);
        for item in &self.sky_time {
            writer.write_item(item);
        }
        true
    }
}
