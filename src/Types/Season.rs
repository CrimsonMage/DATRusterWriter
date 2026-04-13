use crate::{Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable}, Types::AC1LegacyString::AC1LegacyString};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Season {
    pub start: u32,
    pub name: AC1LegacyString,
}

impl IUnpackable for Season {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.start = reader.read_u32();
        self.name = reader.read_item::<AC1LegacyString>();
        reader.align(4);
        true
    }
}

impl IPackable for Season {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.start);
        let _ = self.name.pack(writer);
        writer.align(4);
        true
    }
}
