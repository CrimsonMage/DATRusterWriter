use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::QualifiedControl::QualifiedControl,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InputMapBinding {
    pub control: QualifiedControl,
    pub action_id: u32,
}

impl IUnpackable for InputMapBinding {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.control = reader.read_item::<QualifiedControl>();
        self.action_id = reader.read_u32();
        true
    }
}

impl IPackable for InputMapBinding {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.control);
        writer.write_u32(self.action_id);
        true
    }
}
