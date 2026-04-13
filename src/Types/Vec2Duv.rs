use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2Duv {
    pub u: f32,
    pub v: f32,
}

impl IUnpackable for Vec2Duv {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.u = reader.read_single();
        self.v = reader.read_single();
        true
    }
}

impl IPackable for Vec2Duv {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_single(self.u);
        writer.write_single(self.v);
        true
    }
}
