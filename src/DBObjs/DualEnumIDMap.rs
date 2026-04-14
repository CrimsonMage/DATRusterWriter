use std::{any::Any, collections::BTreeMap};

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        NumberingType::NumberingType,
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

pub const DUAL_ENUM_ID_MAP_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "DualEnumIDMap",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::DualEnumIDMap,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x27000000,
    last_id: 0x27FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DualEnumIDMap {
    pub base: DBObjBase,
    pub client_id_numbering_type: NumberingType,
    pub client_enum_to_id: BTreeMap<u32, u32>,
    pub client_name_numbering_type: NumberingType,
    pub client_enum_to_name: BTreeMap<u32, String>,
    pub server_id_numbering_type: NumberingType,
    pub server_enum_to_id: BTreeMap<u32, u32>,
    pub server_name_numbering_type: NumberingType,
    pub server_enum_to_name: BTreeMap<u32, String>,
}

impl DBObj for DualEnumIDMap {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::DualEnumIDMap
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

impl IUnpackable for DualEnumIDMap {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);

        self.client_id_numbering_type = NumberingType::from(reader.read_byte());
        let client_id_count = reader.read_compressed_uint() as usize;
        self.client_enum_to_id.clear();
        for _ in 0..client_id_count {
            self.client_enum_to_id
                .insert(reader.read_u32(), reader.read_u32());
        }

        self.client_name_numbering_type = NumberingType::from(reader.read_byte());
        let client_name_count = reader.read_compressed_uint() as usize;
        self.client_enum_to_name.clear();
        for _ in 0..client_name_count {
            self.client_enum_to_name
                .insert(reader.read_u32(), reader.read_string16_l_byte());
        }

        self.server_id_numbering_type = NumberingType::from(reader.read_byte());
        let server_id_count = reader.read_compressed_uint() as usize;
        self.server_enum_to_id.clear();
        for _ in 0..server_id_count {
            self.server_enum_to_id
                .insert(reader.read_u32(), reader.read_u32());
        }

        self.server_name_numbering_type = NumberingType::from(reader.read_byte());
        let server_name_count = reader.read_compressed_uint() as usize;
        self.server_enum_to_name.clear();
        for _ in 0..server_name_count {
            self.server_enum_to_name
                .insert(reader.read_u32(), reader.read_string16_l_byte());
        }

        true
    }
}

impl IPackable for DualEnumIDMap {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);

        writer.write_byte(self.client_id_numbering_type.into());
        writer.write_compressed_uint(self.client_enum_to_id.len() as u32);
        for (key, value) in &self.client_enum_to_id {
            writer.write_u32(*key);
            writer.write_u32(*value);
        }

        writer.write_byte(self.client_name_numbering_type.into());
        writer.write_compressed_uint(self.client_enum_to_name.len() as u32);
        for (key, value) in &self.client_enum_to_name {
            writer.write_u32(*key);
            writer.write_string16_l_byte(value);
        }

        writer.write_byte(self.server_id_numbering_type.into());
        writer.write_compressed_uint(self.server_enum_to_id.len() as u32);
        for (key, value) in &self.server_enum_to_id {
            writer.write_u32(*key);
            writer.write_u32(*value);
        }

        writer.write_byte(self.server_name_numbering_type.into());
        writer.write_compressed_uint(self.server_enum_to_name.len() as u32);
        for (key, value) in &self.server_enum_to_name {
            writer.write_u32(*key);
            writer.write_string16_l_byte(value);
        }

        true
    }
}

impl IDBObj for DualEnumIDMap {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &DUAL_ENUM_ID_MAP_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::DualEnumIDMap
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
