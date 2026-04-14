use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::PStringBase::PStringBase,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TabooTableEntry {
    pub key: u32,
    pub unknown2: u16,
    pub banned_patterns: Vec<PStringBase<u8>>,
}

impl IUnpackable for TabooTableEntry {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.key = reader.read_u32();
        self.unknown2 = reader.read_u16();
        let count = reader.read_u32() as usize;
        self.banned_patterns.clear();
        self.banned_patterns.reserve(count);
        for _ in 0..count {
            self.banned_patterns
                .push(reader.read_item::<PStringBase<u8>>());
        }
        true
    }
}

impl IPackable for TabooTableEntry {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.key);
        writer.write_u16(self.unknown2);
        writer.write_u32(self.banned_patterns.len() as u32);
        for item in &self.banned_patterns {
            writer.write_item(item);
        }
        true
    }
}
