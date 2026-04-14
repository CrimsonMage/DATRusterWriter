use crate::{
    DBObjs::RenderSurface::RenderSurface,
    Generated::Enums::SkillId::SkillId,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{PStringBase::PStringBase, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemplateCG {
    pub name: PStringBase<u8>,
    pub icon_id: QualifiedDataId<RenderSurface>,
    pub title: u32,
    pub strength: i32,
    pub endurance: i32,
    pub coordination: i32,
    pub quickness: i32,
    pub focus: i32,
    pub self_value: i32,
    pub normal_skills: Vec<SkillId>,
    pub primary_skills: Vec<SkillId>,
}

impl IUnpackable for TemplateCG {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.name = reader.read_item::<PStringBase<u8>>();
        self.icon_id = reader.read_item::<QualifiedDataId<RenderSurface>>();
        self.title = reader.read_u32();
        self.strength = reader.read_i32();
        self.endurance = reader.read_i32();
        self.coordination = reader.read_i32();
        self.quickness = reader.read_i32();
        self.focus = reader.read_i32();
        self.self_value = reader.read_i32();
        let normal_count = reader.read_compressed_uint() as usize;
        self.normal_skills.clear();
        for _ in 0..normal_count {
            self.normal_skills.push(SkillId::from(reader.read_i32()));
        }
        let primary_count = reader.read_compressed_uint() as usize;
        self.primary_skills.clear();
        for _ in 0..primary_count {
            self.primary_skills.push(SkillId::from(reader.read_i32()));
        }
        true
    }
}

impl IPackable for TemplateCG {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.name);
        writer.write_item(&self.icon_id);
        writer.write_u32(self.title);
        writer.write_i32(self.strength);
        writer.write_i32(self.endurance);
        writer.write_i32(self.coordination);
        writer.write_i32(self.quickness);
        writer.write_i32(self.focus);
        writer.write_i32(self.self_value);
        writer.write_compressed_uint(self.normal_skills.len() as u32);
        for item in &self.normal_skills {
            writer.write_i32((*item).into());
        }
        writer.write_compressed_uint(self.primary_skills.len() as u32);
        for item in &self.primary_skills {
            writer.write_i32((*item).into());
        }
        true
    }
}

