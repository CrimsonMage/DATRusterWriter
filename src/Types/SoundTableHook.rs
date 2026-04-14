use crate::{
    Generated::Enums::{
        AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType, Sound::Sound,
    },
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SoundTableHook {
    pub direction: AnimationHookDir,
    pub sound_type: Sound,
}

impl SoundTableHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::SOUND_TABLE
    }
}

impl IUnpackable for SoundTableHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.sound_type = Sound::from(reader.read_u32());
        true
    }
}

impl IPackable for SoundTableHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_u32(self.sound_type.into());
        true
    }
}
