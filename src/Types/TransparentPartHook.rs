use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransparentPartHook {
    pub direction: AnimationHookDir,
    pub part_index: u32,
    pub start: f32,
    pub end: f32,
    pub time: f32,
}

impl TransparentPartHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::TRANSPARENT_PART
    }
}

impl IUnpackable for TransparentPartHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.part_index = reader.read_u32();
        self.start = reader.read_single();
        self.end = reader.read_single();
        self.time = reader.read_single();
        true
    }
}

impl IPackable for TransparentPartHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_u32(self.part_index);
        writer.write_single(self.start);
        writer.write_single(self.end);
        writer.write_single(self.time);
        true
    }
}
