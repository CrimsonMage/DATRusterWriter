#![allow(non_camel_case_types)]

use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LM_UVTranslate {
    pub type_discriminator: u8,
    pub offset_u: f32,
    pub offset_v: f32,
}

impl IUnpackable for LM_UVTranslate {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.type_discriminator = reader.read_byte();
        self.offset_u = reader.read_single();
        self.offset_v = reader.read_single();
        true
    }
}

impl IPackable for LM_UVTranslate {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(1);
        writer.write_single(self.offset_u);
        writer.write_single(self.offset_v);
        true
    }
}
