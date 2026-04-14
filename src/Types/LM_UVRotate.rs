#![allow(non_camel_case_types)]

use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LM_UVRotate {
    pub type_discriminator: u8,
    pub center_u: f32,
    pub center_v: f32,
    pub angle: f32,
}

impl IUnpackable for LM_UVRotate {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.type_discriminator = reader.read_byte();
        self.center_u = reader.read_single();
        self.center_v = reader.read_single();
        self.angle = reader.read_single();
        true
    }
}

impl IPackable for LM_UVRotate {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(2);
        writer.write_single(self.center_u);
        writer.write_single(self.center_v);
        writer.write_single(self.angle);
        true
    }
}
