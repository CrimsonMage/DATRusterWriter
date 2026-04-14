use crate::{
    DBObjs::Wave::Wave,
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::QualifiedDataId::QualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SoundTweakedHook {
    pub direction: AnimationHookDir,
    pub sound_id: QualifiedDataId<Wave>,
    pub priority: f32,
    pub probability: f32,
    pub volume: f32,
}

impl SoundTweakedHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::SOUND_TWEAKED
    }
}

impl IUnpackable for SoundTweakedHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.sound_id = reader.read_item::<QualifiedDataId<Wave>>();
        self.priority = reader.read_single();
        self.probability = reader.read_single();
        self.volume = reader.read_single();
        true
    }
}

impl IPackable for SoundTweakedHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_item(&self.sound_id);
        writer.write_single(self.priority);
        writer.write_single(self.probability);
        writer.write_single(self.volume);
        true
    }
}
