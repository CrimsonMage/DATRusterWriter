use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::ControlSpecification::ControlSpecification,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedControl {
    pub key: ControlSpecification,
    pub meta_mode: u32,
    pub activation: u32,
}

impl IUnpackable for QualifiedControl {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.key = reader.read_item::<ControlSpecification>();
        self.meta_mode = reader.read_u32();
        self.activation = reader.read_u32();
        true
    }
}

impl IPackable for QualifiedControl {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.key);
        writer.write_u32(self.meta_mode);
        writer.write_u32(self.activation);
        true
    }
}
