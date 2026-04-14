use crate::{
    Generated::Enums::AttributeId::AttributeId,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SkillFormula {
    pub additive_bonus: i32,
    pub attribute1_multiplier: i32,
    pub attribute2_multiplier: i32,
    pub divisor: i32,
    pub attribute1: AttributeId,
    pub attribute2: AttributeId,
}

impl IUnpackable for SkillFormula {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.additive_bonus = reader.read_i32();
        self.attribute1_multiplier = reader.read_i32();
        self.attribute2_multiplier = reader.read_i32();
        self.divisor = reader.read_i32();
        self.attribute1 = AttributeId::from(reader.read_u32());
        self.attribute2 = AttributeId::from(reader.read_u32());
        true
    }
}

impl IPackable for SkillFormula {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.additive_bonus);
        writer.write_i32(self.attribute1_multiplier);
        writer.write_i32(self.attribute2_multiplier);
        writer.write_i32(self.divisor);
        writer.write_u32(self.attribute1.into());
        writer.write_u32(self.attribute2.into());
        true
    }
}
