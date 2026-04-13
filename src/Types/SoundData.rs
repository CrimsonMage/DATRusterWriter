use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::SoundEntry::SoundEntry,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SoundData {
    pub entries: Vec<SoundEntry>,
    pub unknown: i32,
}

impl IUnpackable for SoundData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.entries.clear();
        for _ in 0..count {
            self.entries.push(reader.read_item::<SoundEntry>());
        }
        self.unknown = reader.read_i32();
        true
    }
}

impl IPackable for SoundData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.entries.len() as u32);
        for entry in &self.entries {
            writer.write_item(entry);
        }
        writer.write_i32(self.unknown);
        true
    }
}
