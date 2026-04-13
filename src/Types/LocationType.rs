use crate::{
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::Frame::Frame,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LocationType {
    pub part_id: i32,
    pub frame: Frame,
}

impl IUnpackable for LocationType {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.part_id = reader.read_i32();
        self.frame = reader.read_item::<Frame>();
        true
    }
}

impl IPackable for LocationType {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.part_id);
        writer.write_item(&self.frame);
        true
    }
}
