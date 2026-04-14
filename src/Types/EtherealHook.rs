use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EtherealHook {
    pub direction: AnimationHookDir,
    pub ethereal: bool,
}

impl EtherealHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::ETHEREAL
    }
}

impl IUnpackable for EtherealHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.ethereal = reader.read_bool(4);
        true
    }
}

impl IPackable for EtherealHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_bool(self.ethereal, 4);
        true
    }
}
