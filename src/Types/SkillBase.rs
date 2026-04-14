use crate::{
    DBObjs::RenderSurface::RenderSurface,
    Generated::Enums::SkillCategory::SkillCategory,
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::{AC1LegacyString::AC1LegacyString, QualifiedDataId::QualifiedDataId, SkillFormula::SkillFormula},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillBase {
    pub description: AC1LegacyString,
    pub name: AC1LegacyString,
    pub icon_id: QualifiedDataId<RenderSurface>,
    pub trained_cost: i32,
    pub specialized_cost: i32,
    pub category: SkillCategory,
    pub chargen_use: bool,
    pub min_level: u32,
    pub formula: SkillFormula,
    pub upper_bound: f64,
    pub lower_bound: f64,
    pub learn_mod: f64,
}

impl IUnpackable for SkillBase {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.description = reader.read_item::<AC1LegacyString>();
        self.name = reader.read_item::<AC1LegacyString>();
        self.icon_id = reader.read_item::<QualifiedDataId<RenderSurface>>();
        self.trained_cost = reader.read_i32();
        self.specialized_cost = reader.read_i32();
        self.category = SkillCategory::from(reader.read_u32());
        self.chargen_use = reader.read_bool(4);
        self.min_level = reader.read_u32();
        self.formula = reader.read_item::<SkillFormula>();
        self.upper_bound = reader.read_double();
        self.lower_bound = reader.read_double();
        self.learn_mod = reader.read_double();
        true
    }
}

impl IPackable for SkillBase {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.description);
        writer.write_item(&self.name);
        writer.write_item(&self.icon_id);
        writer.write_i32(self.trained_cost);
        writer.write_i32(self.specialized_cost);
        writer.write_u32(self.category.into());
        writer.write_bool(self.chargen_use, 4);
        writer.write_u32(self.min_level);
        writer.write_item(&self.formula);
        writer.write_double(self.upper_bound);
        writer.write_double(self.lower_bound);
        writer.write_double(self.learn_mod);
        true
    }
}
