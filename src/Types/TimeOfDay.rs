use crate::{Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable}, Types::AC1LegacyString::AC1LegacyString};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeOfDay {
    pub start: f32,
    pub is_night: bool,
    pub name: AC1LegacyString,
}

impl IUnpackable for TimeOfDay {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.start = reader.read_single();
        self.is_night = reader.read_bool(4);
        self.name = reader.read_item::<AC1LegacyString>();
        reader.align(4);
        true
    }
}

impl IPackable for TimeOfDay {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_single(self.start);
        writer.write_bool(self.is_night, 4);
        let _ = self.name.pack(writer);
        writer.align(4);
        true
    }
}
