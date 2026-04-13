use crate::{
    DBObjs::Wave::Wave,
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::QualifiedDataId::QualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SoundEntry {
    pub id: QualifiedDataId<Wave>,
    pub priority: f32,
    pub probability: f32,
    pub volume: f32,
}

impl IUnpackable for SoundEntry {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.id = reader.read_item::<QualifiedDataId<Wave>>();
        self.priority = reader.read_single();
        self.probability = reader.read_single();
        self.volume = reader.read_single();
        true
    }
}

impl IPackable for SoundEntry {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.id);
        writer.write_single(self.priority);
        writer.write_single(self.probability);
        writer.write_single(self.volume);
        true
    }
}
