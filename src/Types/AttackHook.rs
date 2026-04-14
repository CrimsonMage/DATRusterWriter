use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::AttackCone::AttackCone,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttackHook {
    pub direction: AnimationHookDir,
    pub attack_cone: AttackCone,
}

impl AttackHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::ATTACK
    }
}

impl IUnpackable for AttackHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.attack_cone = reader.read_item::<AttackCone>();
        true
    }
}

impl IPackable for AttackHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_item(&self.attack_cone);
        true
    }
}
