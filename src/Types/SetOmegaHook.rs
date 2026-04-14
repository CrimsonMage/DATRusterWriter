use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable, Numerics::Vector3,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetOmegaHook {
    pub direction: AnimationHookDir,
    pub axis: Vector3,
}

impl SetOmegaHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::SET_OMEGA
    }
}

impl IUnpackable for SetOmegaHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.axis = reader.read_vector3();
        true
    }
}

impl IPackable for SetOmegaHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_vector3(self.axis);
        true
    }
}
