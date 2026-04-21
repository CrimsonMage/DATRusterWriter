use std::{any::Any, collections::BTreeMap};

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
        CInputMap::CInputMap,
        ControlSpecification::ControlSpecification,
        DBObj::{DBObj, DBObjBase},
        DeviceKeyMapEntry::DeviceKeyMapEntry,
        PStringBase::PStringBase,
    },
};

pub const MASTER_INPUT_MAP_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "MasterInputMap",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::MasterInputMap,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x14000000,
    last_id: 0x1400FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MasterInputMap {
    pub base: DBObjBase,
    pub name: PStringBase<u8>,
    pub guid_map: uuid::Uuid,
    pub devices: Vec<DeviceKeyMapEntry>,
    pub meta_keys: Vec<(ControlSpecification, u32)>,
    pub input_maps: BTreeMap<u32, CInputMap>,
}

impl DBObj for MasterInputMap {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::MasterInputMap
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

impl IUnpackable for MasterInputMap {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.name = reader.read_item::<PStringBase<u8>>();
        self.guid_map = reader.read_guid();

        let device_count = reader.read_u32() as usize;
        self.devices.clear();
        self.devices.reserve(device_count);
        for _ in 0..device_count {
            self.devices.push(reader.read_item::<DeviceKeyMapEntry>());
        }

        let meta_key_count = reader.read_u32() as usize;
        self.meta_keys.clear();
        self.meta_keys.reserve(meta_key_count);
        for _ in 0..meta_key_count {
            let control = reader.read_item::<ControlSpecification>();
            let meta_mode = reader.read_u32();
            self.meta_keys.push((control, meta_mode));
        }

        let input_map_count = reader.read_u32() as usize;
        self.input_maps.clear();
        for _ in 0..input_map_count {
            let key = reader.read_u32();
            let value = reader.read_item::<CInputMap>();
            self.input_maps.insert(key, value);
        }
        true
    }
}

impl IPackable for MasterInputMap {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.name);
        writer.write_guid(self.guid_map);
        writer.write_u32(self.devices.len() as u32);
        for item in &self.devices {
            writer.write_item(item);
        }
        writer.write_u32(self.meta_keys.len() as u32);
        for (control, meta_mode) in &self.meta_keys {
            writer.write_item(control);
            writer.write_u32(*meta_mode);
        }
        writer.write_u32(self.input_maps.len() as u32);
        for (key, value) in &self.input_maps {
            writer.write_u32(*key);
            writer.write_item(value);
        }
        true
    }
}

impl IDBObj for MasterInputMap {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &MASTER_INPUT_MAP_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::MasterInputMap
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
