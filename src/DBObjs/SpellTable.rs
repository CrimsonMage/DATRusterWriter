use std::any::Any;

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        EquipmentSet::EquipmentSet,
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
        PHashTable::PHashTable,
        PackableHashTable::PackableHashTable,
        SpellBase::SpellBase,
        SpellSet::SpellSet,
    },
};

pub const SPELL_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "SpellTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::SpellTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E00000E,
    last_id: 0x0E00000E,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpellTable {
    pub base: DBObjBase,
    pub spells: PackableHashTable<u32, SpellBase>,
    pub spell_sets: PHashTable<EquipmentSet, SpellSet>,
}

impl DBObj for SpellTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::SpellTable
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

impl IUnpackable for SpellTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.spells = reader.read_item::<PackableHashTable<u32, SpellBase>>();
        self.spell_sets = reader.read_item::<PHashTable<EquipmentSet, SpellSet>>();
        true
    }
}

impl IPackable for SpellTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.spells);
        writer.write_item(&self.spell_sets);
        true
    }
}

impl IDBObj for SpellTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &SPELL_TABLE_ATTR
    }

    fn db_obj_type(&self) -> DBObjType {
        DBObjType::SpellTable
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
