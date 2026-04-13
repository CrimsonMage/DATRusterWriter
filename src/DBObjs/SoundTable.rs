use std::{any::Any, collections::BTreeMap};

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        Sound::Sound,
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
        SoundData::SoundData,
        SoundHashData::SoundHashData,
    },
};

pub const SOUND_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "SoundTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::SoundTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x20000000,
    last_id: 0x2000FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SoundTable {
    pub base: DBObjBase,
    pub hash_key: i32,
    pub hashes: BTreeMap<u32, SoundHashData>,
    pub sounds: BTreeMap<Sound, SoundData>,
}

impl DBObj for SoundTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::SoundTable
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

impl IUnpackable for SoundTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.hash_key = reader.read_i32();

        let hash_count = reader.read_i32() as usize;
        self.hashes.clear();
        for _ in 0..hash_count {
            self.hashes
                .insert(reader.read_u32(), reader.read_item::<SoundHashData>());
        }

        let sound_count = reader.read_i32() as usize;
        self.sounds.clear();
        for _ in 0..sound_count {
            self.sounds.insert(
                Sound::from(reader.read_u32()),
                reader.read_item::<SoundData>(),
            );
        }
        true
    }
}

impl IPackable for SoundTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_i32(self.hash_key);
        writer.write_i32(self.hashes.len() as i32);
        for (key, value) in &self.hashes {
            writer.write_u32(*key);
            writer.write_item(value);
        }
        writer.write_i32(self.sounds.len() as i32);
        for (key, value) in &self.sounds {
            writer.write_u32((*key).into());
            writer.write_item(value);
        }
        true
    }
}

impl IDBObj for SoundTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &SOUND_TABLE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::SoundTable
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
