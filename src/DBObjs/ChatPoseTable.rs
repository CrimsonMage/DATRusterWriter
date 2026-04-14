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
        AC1LegacyPStringBase::AC1LegacyPStringBase,
        ChatEmoteData::ChatEmoteData,
        DBObj::{DBObj, DBObjBase},
        PackableHashTable::PackableHashTable,
    },
};

pub const CHAT_POSE_TABLE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "ChatPoseTable",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::ChatPoseTable,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E000007,
    last_id: 0x0E000007,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChatPoseTable {
    pub base: DBObjBase,
    pub chat_poses: PackableHashTable<AC1LegacyPStringBase<u8>, AC1LegacyPStringBase<u8>>,
    pub chat_emotes: PackableHashTable<AC1LegacyPStringBase<u8>, ChatEmoteData>,
}

fn unpack_string_key_table<V>(
    reader: &mut DatBinReader<'_>,
) -> PackableHashTable<AC1LegacyPStringBase<u8>, V>
where
    V: IUnpackable + Default,
{
    let count = reader.read_u16() as usize;
    let bucket_size = reader.read_u16();
    let mut table = PackableHashTable {
        bucket_size,
        ..Default::default()
    };
    for _ in 0..count {
        let key = reader.read_item::<AC1LegacyPStringBase<u8>>();
        let value = reader.read_item::<V>();
        table.entries.insert(key, value);
    }
    table
}

fn pack_string_key_table<V>(
    writer: &mut DatBinWriter<'_>,
    table: &PackableHashTable<AC1LegacyPStringBase<u8>, V>,
) where
    V: IPackable,
{
    writer.write_u16(table.entries.len() as u16);
    writer.write_u16(table.bucket_size);
    for (key, value) in &table.entries {
        writer.write_item(key);
        writer.write_item(value);
    }
}

impl DBObj for ChatPoseTable {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ChatPoseTable
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

impl IUnpackable for ChatPoseTable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.chat_poses = unpack_string_key_table::<AC1LegacyPStringBase<u8>>(reader);
        self.chat_emotes = unpack_string_key_table::<ChatEmoteData>(reader);
        true
    }
}

impl IPackable for ChatPoseTable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        pack_string_key_table(writer, &self.chat_poses);
        pack_string_key_table(writer, &self.chat_emotes);
        true
    }
}

impl IDBObj for ChatPoseTable {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &CHAT_POSE_TABLE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ChatPoseTable
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
