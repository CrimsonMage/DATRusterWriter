use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransparentHook {
    pub direction: AnimationHookDir,
    pub start: f32,
    pub end: f32,
    pub time: f32,
}

impl TransparentHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::TRANSPARENT
    }
}

impl IUnpackable for TransparentHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.start = reader.read_single();
        self.end = reader.read_single();
        self.time = reader.read_single();
        true
    }
}

impl IPackable for TransparentHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_single(self.start);
        writer.write_single(self.end);
        writer.write_single(self.time);
        true
    }
}
