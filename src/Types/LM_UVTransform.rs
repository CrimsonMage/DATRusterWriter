#![allow(non_camel_case_types)]

use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LM_UVTransform {
    pub type_discriminator: u8,
}

impl IUnpackable for LM_UVTransform {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.type_discriminator = reader.read_byte();
        true
    }
}

impl IPackable for LM_UVTransform {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(4);
        true
    }
}
