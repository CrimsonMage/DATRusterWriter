use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::ControlSpecification::ControlSpecification,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QualifiedControl {
    pub key: ControlSpecification,
    pub activation: u32,
    pub unknown: u32,
}

impl IUnpackable for QualifiedControl {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.key = reader.read_item::<ControlSpecification>();
        self.activation = reader.read_u32();
        self.unknown = reader.read_u32();
        true
    }
}

impl IPackable for QualifiedControl {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.key);
        writer.write_u32(self.activation);
        writer.write_u32(self.unknown);
        true
    }
}
