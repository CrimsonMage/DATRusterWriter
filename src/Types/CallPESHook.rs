use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CallPESHook {
    pub direction: AnimationHookDir,
    pub pes: u32,
    pub pause: f32,
}

impl CallPESHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::CALL_PES
    }
}

impl IUnpackable for CallPESHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.pes = reader.read_u32();
        self.pause = reader.read_single();
        true
    }
}

impl IPackable for CallPESHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_u32(self.pes);
        writer.write_single(self.pause);
        true
    }
}
