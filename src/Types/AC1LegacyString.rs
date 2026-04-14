use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::StringBase::StringBase;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AC1LegacyString {
    pub value: String,
}

impl StringBase for AC1LegacyString {
    fn value(&self) -> &str {
        &self.value
    }
}

impl IUnpackable for AC1LegacyString {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let length_marker = reader.read_u16();
        let length = if length_marker == 0xFFFF {
            reader.read_u32() as usize
        } else {
            length_marker as usize
        };
        let bytes = reader.read_bytes(length);
        self.value = String::from_utf8_lossy(&bytes).to_string();
        reader.align(4);
        true
    }
}

impl IPackable for AC1LegacyString {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let bytes = self.value.as_bytes();
        if bytes.len() >= 0xFFFF {
            writer.write_u16(0xFFFF);
            writer.write_u32(bytes.len() as u32);
        } else {
            writer.write_u16(bytes.len() as u16);
        }
        writer.write_bytes(bytes, bytes.len());
        writer.align(4);
        true
    }
}
