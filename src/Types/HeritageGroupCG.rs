use crate::{
    DBObjs::{RenderSurface::RenderSurface, Setup::Setup},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{
        AC1LegacyString::AC1LegacyString, HashTable::HashTable, QualifiedDataId::QualifiedDataId,
        SexCG::SexCG, SkillCG::SkillCG, TemplateCG::TemplateCG,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HeritageGroupCG {
    pub name: AC1LegacyString,
    pub icon_id: QualifiedDataId<RenderSurface>,
    pub setup_id: QualifiedDataId<Setup>,
    pub environment_setup_id: QualifiedDataId<Setup>,
    pub attribute_credits: u32,
    pub skill_credits: u32,
    pub primary_start_areas: Vec<i32>,
    pub secondary_start_areas: Vec<i32>,
    pub skills: Vec<SkillCG>,
    pub templates: Vec<TemplateCG>,
    pub genders: HashTable<i32, SexCG>,
}

impl IUnpackable for HeritageGroupCG {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.name = reader.read_item::<AC1LegacyString>();
        self.icon_id = reader.read_item::<QualifiedDataId<RenderSurface>>();
        self.setup_id = reader.read_item::<QualifiedDataId<Setup>>();
        self.environment_setup_id = reader.read_item::<QualifiedDataId<Setup>>();
        self.attribute_credits = reader.read_u32();
        self.skill_credits = reader.read_u32();

        let primary_count = reader.read_compressed_uint() as usize;
        self.primary_start_areas.clear();
        for _ in 0..primary_count {
            self.primary_start_areas.push(reader.read_i32());
        }

        let secondary_count = reader.read_compressed_uint() as usize;
        self.secondary_start_areas.clear();
        for _ in 0..secondary_count {
            self.secondary_start_areas.push(reader.read_i32());
        }

        let skills_count = reader.read_compressed_uint() as usize;
        self.skills.clear();
        for _ in 0..skills_count {
            self.skills.push(reader.read_item::<SkillCG>());
        }

        let templates_count = reader.read_compressed_uint() as usize;
        self.templates.clear();
        for _ in 0..templates_count {
            self.templates.push(reader.read_item::<TemplateCG>());
        }

        self.genders = reader.read_item::<HashTable<i32, SexCG>>();
        true
    }
}

impl IPackable for HeritageGroupCG {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.name);
        writer.write_item(&self.icon_id);
        writer.write_item(&self.setup_id);
        writer.write_item(&self.environment_setup_id);
        writer.write_u32(self.attribute_credits);
        writer.write_u32(self.skill_credits);
        writer.write_compressed_uint(self.primary_start_areas.len() as u32);
        for item in &self.primary_start_areas {
            writer.write_i32(*item);
        }
        writer.write_compressed_uint(self.secondary_start_areas.len() as u32);
        for item in &self.secondary_start_areas {
            writer.write_i32(*item);
        }
        writer.write_compressed_uint(self.skills.len() as u32);
        for item in &self.skills {
            writer.write_item(item);
        }
        writer.write_compressed_uint(self.templates.len() as u32);
        for item in &self.templates {
            writer.write_item(item);
        }
        writer.write_item(&self.genders);
        true
    }
}
