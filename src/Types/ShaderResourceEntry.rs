use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ShaderResourceEntry {
    pub start_offset: u32,
    pub resource_id: u32,
    pub resource_data: u32,
}

impl IUnpackable for ShaderResourceEntry {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.start_offset = reader.read_u32();
        self.resource_id = reader.read_u32();
        self.resource_data = reader.read_u32();
        true
    }
}

impl IPackable for ShaderResourceEntry {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.start_offset);
        writer.write_u32(self.resource_id);
        writer.write_u32(self.resource_data);
        true
    }
}
