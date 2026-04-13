use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ColorARGB {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}

impl IUnpackable for ColorARGB {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.blue = reader.read_byte();
        self.green = reader.read_byte();
        self.red = reader.read_byte();
        self.alpha = reader.read_byte();
        true
    }
}

impl IPackable for ColorARGB {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.blue);
        writer.write_byte(self.green);
        writer.write_byte(self.red);
        writer.write_byte(self.alpha);
        true
    }
}
