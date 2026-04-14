use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

use crate::Types::Frame::Frame;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stab {
    pub id: u32,
    pub frame: Frame,
}

impl IUnpackable for Stab {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.id = reader.read_u32();
        self.frame = reader.read_item::<Frame>();
        true
    }
}

impl IPackable for Stab {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.id);
        writer.write_item(&self.frame);
        true
    }
}
