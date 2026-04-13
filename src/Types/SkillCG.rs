use crate::{
    Generated::Enums::SkillId::SkillId,
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillCG {
    pub id: SkillId,
    pub normal_cost: i32,
    pub primary_cost: i32,
}

impl IUnpackable for SkillCG {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.id = SkillId::from(reader.read_i32());
        self.normal_cost = reader.read_i32();
        self.primary_cost = reader.read_i32();
        true
    }
}

impl IPackable for SkillCG {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.id.into());
        writer.write_i32(self.normal_cost);
        writer.write_i32(self.primary_cost);
        true
    }
}
