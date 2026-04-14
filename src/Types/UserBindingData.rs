use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UserBindingData {
    pub action_class: u32,
    pub action_name: u32,
    pub action_description: u32,
}

impl IUnpackable for UserBindingData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.action_class = reader.read_u32();
        self.action_name = reader.read_u32();
        self.action_description = reader.read_u32();
        true
    }
}

impl IPackable for UserBindingData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.action_class);
        writer.write_u32(self.action_name);
        writer.write_u32(self.action_description);
        true
    }
}
