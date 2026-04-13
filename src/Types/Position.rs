use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::Frame::Frame,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Position {
    pub cell_id: u32,
    pub frame: Frame,
}

impl IUnpackable for Position {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.cell_id = reader.read_u32();
        self.frame = reader.read_item::<Frame>();
        true
    }
}

impl IPackable for Position {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.cell_id);
        writer.write_item(&self.frame);
        true
    }
}
