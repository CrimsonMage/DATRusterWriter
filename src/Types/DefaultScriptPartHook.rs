use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DefaultScriptPartHook {
    pub direction: AnimationHookDir,
    pub part_index: u32,
}

impl DefaultScriptPartHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::DEFAULT_SCRIPT_PART
    }
}

impl IUnpackable for DefaultScriptPartHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.part_index = reader.read_u32();
        true
    }
}

impl IPackable for DefaultScriptPartHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_u32(self.part_index);
        true
    }
}
