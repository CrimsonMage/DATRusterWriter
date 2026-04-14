use std::any::Any;

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
    },
    Lib::{
        Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj,
            IPackable::IPackable, IUnpackable::IUnpackable,
        },
    },
    Types::DBObj::{DBObj, DBObjBase},
};

pub const EXPERIENCE_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "ExperienceTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::ExperienceTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E000018,
    last_id: 0x0E000018,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExperienceTable {
    pub base: DBObjBase,
    pub attributes: Vec<u32>,
    pub vitals: Vec<u32>,
    pub trained_skills: Vec<u32>,
    pub specialized_skills: Vec<u32>,
    pub levels: Vec<u64>,
    pub skill_credits: Vec<u32>,
}

impl DBObj for ExperienceTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ExperienceTable
    }
    fn id(&self) -> u32 {
        self.base.id
    }
    fn set_id(&mut self, id: u32) {
        self.base.id = id;
    }
    fn data_category(&self) -> u32 {
        self.base.data_category
    }
    fn set_data_category(&mut self, data_category: u32) {
        self.base.data_category = data_category;
    }
}

impl IUnpackable for ExperienceTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let attribute_count = reader.read_i32().max(0) as usize;
        let vital_count = reader.read_i32().max(0) as usize;
        let trained_skill_count = reader.read_i32().max(0) as usize;
        let specialized_skill_count = reader.read_i32().max(0) as usize;
        let level_count = reader.read_u32() as usize;

        self.attributes = (0..=attribute_count).map(|_| reader.read_u32()).collect();
        self.vitals = (0..=vital_count).map(|_| reader.read_u32()).collect();
        self.trained_skills = (0..=trained_skill_count)
            .map(|_| reader.read_u32())
            .collect();
        self.specialized_skills = (0..=specialized_skill_count)
            .map(|_| reader.read_u32())
            .collect();
        self.levels = (0..=level_count).map(|_| reader.read_u64()).collect();
        self.skill_credits = (0..=level_count).map(|_| reader.read_u32()).collect();
        true
    }
}

impl IPackable for ExperienceTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_i32(self.attributes.len().saturating_sub(1) as i32);
        writer.write_i32(self.vitals.len().saturating_sub(1) as i32);
        writer.write_i32(self.trained_skills.len().saturating_sub(1) as i32);
        writer.write_i32(self.specialized_skills.len().saturating_sub(1) as i32);
        writer.write_u32(self.levels.len().saturating_sub(1) as u32);

        for value in &self.attributes {
            writer.write_u32(*value);
        }
        for value in &self.vitals {
            writer.write_u32(*value);
        }
        for value in &self.trained_skills {
            writer.write_u32(*value);
        }
        for value in &self.specialized_skills {
            writer.write_u32(*value);
        }
        for value in &self.levels {
            writer.write_u64(*value);
        }
        for value in &self.skill_credits {
            writer.write_u32(*value);
        }
        true
    }
}

impl IDBObj for ExperienceTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &EXPERIENCE_TABLE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ExperienceTable
    }
    fn id(&self) -> u32 {
        self.base.id
    }
    fn set_id(&mut self, id: u32) {
        self.base.id = id;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
