#![allow(non_camel_case_types)]

use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Waveform {
    pub raw_data: u8,
}

impl IUnpackable for Waveform {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.raw_data = reader.read_byte();
        true
    }
}

impl IPackable for Waveform {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.raw_data);
        true
    }
}
