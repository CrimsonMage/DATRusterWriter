#![allow(non_camel_case_types)]

use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LM_UVScale {
    pub type_discriminator: u8,
    pub scale_u: f32,
    pub scale_v: f32,
}

impl IUnpackable for LM_UVScale {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.type_discriminator = reader.read_byte();
        self.scale_u = reader.read_single();
        self.scale_v = reader.read_single();
        true
    }
}

impl IPackable for LM_UVScale {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(3);
        writer.write_single(self.scale_u);
        writer.write_single(self.scale_v);
        true
    }
}
