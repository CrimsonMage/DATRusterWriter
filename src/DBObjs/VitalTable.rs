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
    Types::{
        DBObj::{DBObj, DBObjBase},
        SkillFormula::SkillFormula,
    },
};

pub const VITAL_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "VitalTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::VitalTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E000003,
    last_id: 0x0E000003,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VitalTable {
    pub base: DBObjBase,
    pub health: SkillFormula,
    pub stamina: SkillFormula,
    pub mana: SkillFormula,
}

impl DBObj for VitalTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::VitalTable
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

impl IUnpackable for VitalTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.health = reader.read_item::<SkillFormula>();
        self.stamina = reader.read_item::<SkillFormula>();
        self.mana = reader.read_item::<SkillFormula>();
        true
    }
}

impl IPackable for VitalTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.health);
        writer.write_item(&self.stamina);
        writer.write_item(&self.mana);
        true
    }
}

impl IDBObj for VitalTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &VITAL_TABLE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::VitalTable
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
