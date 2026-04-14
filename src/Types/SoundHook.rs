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
pub struct SoundHook {
    pub direction: AnimationHookDir,
    pub id: QualifiedDataId<Wave>,
}

impl SoundHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::SOUND
    }
}

impl IUnpackable for SoundHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.id = reader.read_item::<QualifiedDataId<Wave>>();
        true
    }
}

impl IPackable for SoundHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_item(&self.id);
        true
    }
}
