use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ControlSpecification {
    pub key: u32,
    pub modifier: u32,
}

impl IUnpackable for ControlSpecification {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.key = reader.read_u32();
        self.modifier = reader.read_u32();
        true
    }
}

impl IPackable for ControlSpecification {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.key);
        writer.write_u32(self.modifier);
        true
    }
}
