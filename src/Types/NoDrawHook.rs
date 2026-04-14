use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NoDrawHook {
    pub direction: AnimationHookDir,
    pub no_draw: bool,
}

impl NoDrawHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::NO_DRAW
    }
}

impl IUnpackable for NoDrawHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.no_draw = reader.read_bool(4);
        true
    }
}

impl IPackable for NoDrawHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_bool(self.no_draw, 4);
        true
    }
}
