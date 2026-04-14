use std::any::Any;

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
    },
    Lib::{
        Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
        DBObjAttributeCache,
        IO::{IPackable::IPackable, IUnpackable::IUnpackable},
    },
};

pub trait IDBObj: IUnpackable + IPackable + Any + Send + Sync {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized;

    fn db_obj_type(&self) -> DBObjType;
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjAttributeCache::all_ported_attributes()
            .iter()
            .copied()
            .find(|attr| attr.db_obj_type == self.db_obj_type())
            .map(|attr| attr.header_flags)
            .unwrap_or(DBObjHeaderFlags::None)
    }
    fn dat_file_type(&self) -> DatFileType
    where
        Self: Sized,
    {
        Self::db_obj_type_attr().dat_file_type
    }
    fn id(&self) -> u32;
    fn set_id(&mut self, id: u32);
    fn data_category(&self) -> u32 {
        0
    }
    fn set_data_category(&mut self, _data_category: u32) {}
    fn as_any(&self) -> &dyn Any;
}
