use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SetLightHook {
    pub direction: AnimationHookDir,
    pub lights_on: bool,
}

impl SetLightHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::SET_LIGHT
    }
}

impl IUnpackable for SetLightHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.lights_on = reader.read_bool(4);
        true
    }
}

impl IPackable for SetLightHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_bool(self.lights_on, 4);
        true
    }
}
