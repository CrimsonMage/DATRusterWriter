use encoding_rs::WINDOWS_1252;

use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObfuscatedPStringBase {
    pub value: String,
}

impl From<&str> for ObfuscatedPStringBase {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl IUnpackable for ObfuscatedPStringBase {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let string_length = reader.read_u16() as usize;
        let mut bytes = reader.read_bytes(string_length);
        for byte in &mut bytes {
            *byte = (*byte >> 4) | (*byte << 4);
        }
        let (decoded, _, _) = WINDOWS_1252.decode(&bytes);
        self.value = decoded.into_owned();
        reader.align(4);
        true
    }
}

impl IPackable for ObfuscatedPStringBase {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let (encoded, _, _) = WINDOWS_1252.encode(&self.value);
        writer.write_u16(encoded.len() as u16);
        for byte in encoded.iter() {
            writer.write_byte((*byte >> 4) | (*byte << 4));
        }
        writer.align(4);
        true
    }
}
