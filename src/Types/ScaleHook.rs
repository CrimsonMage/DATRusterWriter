use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScaleHook {
    pub direction: AnimationHookDir,
    pub end: f32,
    pub time: f32,
}

impl ScaleHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::SCALE
    }
}

impl IUnpackable for ScaleHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.end = reader.read_single();
        self.time = reader.read_single();
        true
    }
}

impl IPackable for ScaleHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_single(self.end);
        writer.write_single(self.time);
        true
    }
}
