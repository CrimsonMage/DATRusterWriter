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
        ObjHierarchyNode::ObjHierarchyNode,
    },
};

pub const OBJECT_HIERARCHY_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "ObjectHierarchy",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::ObjectHierarchy,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x0E00000D,
    last_id: 0x0E00000D,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectHierarchy {
    pub base: DBObjBase,
    pub root_node: ObjHierarchyNode,
}

impl DBObj for ObjectHierarchy {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ObjectHierarchy
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

impl IUnpackable for ObjectHierarchy {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.root_node = reader.read_item::<ObjHierarchyNode>();
        true
    }
}

impl IPackable for ObjectHierarchy {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_item(&self.root_node);
        true
    }
}

impl IDBObj for ObjectHierarchy {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &OBJECT_HIERARCHY_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ObjectHierarchy
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
