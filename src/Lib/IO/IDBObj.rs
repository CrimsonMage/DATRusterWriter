use std::any::Any;

use crate::{
    Generated::Enums::{DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{Attributes::DBObjTypeAttribute::DBObjTypeAttribute, IO::{IPackable::IPackable, IUnpackable::IUnpackable}},
};

pub trait IDBObj: IUnpackable + IPackable + Any + Send + Sync {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized;

    fn db_obj_type(&self) -> DBObjType;
    fn dat_file_type(&self) -> DatFileType where Self: Sized {
        Self::db_obj_type_attr().dat_file_type
    }
    fn id(&self) -> u32;
    fn set_id(&mut self, id: u32);
    fn as_any(&self) -> &dyn Any;
}
