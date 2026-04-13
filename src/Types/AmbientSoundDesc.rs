use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AmbientSoundDesc {
    pub s_type: crate::Generated::Enums::Sound::Sound,
    pub volume: f32,
    pub base_chance: f32,
    pub min_rate: f32,
    pub max_rate: f32,
}

impl IUnpackable for AmbientSoundDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.s_type = reader.read_u32().into();
        self.volume = reader.read_single();
        self.base_chance = reader.read_single();
        self.min_rate = reader.read_single();
        self.max_rate = reader.read_single();
        true
    }
}

impl IPackable for AmbientSoundDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.s_type.into());
        writer.write_single(self.volume);
        writer.write_single(self.base_chance);
        writer.write_single(self.min_rate);
        writer.write_single(self.max_rate);
        true
    }
}
