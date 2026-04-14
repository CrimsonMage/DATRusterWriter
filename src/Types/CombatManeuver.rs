use crate::{
    Generated::Enums::{
        AttackHeight::AttackHeight, AttackType::AttackType, MotionCommand::MotionCommand,
        MotionStance::MotionStance,
    },
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatManeuver {
    pub style: MotionStance,
    pub attack_height: AttackHeight,
    pub attack_type: AttackType,
    pub min_skill_level: u32,
    pub motion: MotionCommand,
}

impl IUnpackable for CombatManeuver {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.style = MotionStance::from(reader.read_u32());
        self.attack_height = AttackHeight::from(reader.read_i32());
        self.attack_type = AttackType::from_bits_truncate(reader.read_i32());
        self.min_skill_level = reader.read_u32();
        self.motion = MotionCommand::from(reader.read_u32());
        true
    }
}

impl IPackable for CombatManeuver {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.style.into());
        writer.write_i32(self.attack_height.into());
        writer.write_i32(self.attack_type.bits());
        writer.write_u32(self.min_skill_level);
        writer.write_u32(self.motion.into());
        true
    }
}
