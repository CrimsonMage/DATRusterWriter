use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SoundHashData {
    pub priority: f32,
    pub probability: f32,
    pub volume: f32,
}

impl IUnpackable for SoundHashData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.priority = reader.read_single();
        self.probability = reader.read_single();
        self.volume = reader.read_single();
        true
    }
}

impl IPackable for SoundHashData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_single(self.priority);
        writer.write_single(self.probability);
        writer.write_single(self.volume);
        true
    }
}
