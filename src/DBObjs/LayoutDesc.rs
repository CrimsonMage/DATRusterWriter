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
        ElementDesc::ElementDesc,
        HashTable::HashTable,
    },
};

pub const LAYOUT_DESC_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "LayoutDesc",
    dat_file_type: DatFileType::Local,
    db_obj_type: DBObjType::LayoutDesc,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x21000000,
    last_id: 0x21FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutDesc {
    pub base: DBObjBase,
    pub width: u32,
    pub height: u32,
    pub elements: HashTable<u32, ElementDesc>,
}

impl DBObj for LayoutDesc {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LayoutDesc
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

impl IUnpackable for LayoutDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.width = reader.read_u32();
        self.height = reader.read_u32();
        self.elements = reader.read_item::<HashTable<u32, ElementDesc>>();
        true
    }
}

impl IPackable for LayoutDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.width);
        writer.write_u32(self.height);
        writer.write_item(&self.elements);
        true
    }
}

impl IDBObj for LayoutDesc {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &LAYOUT_DESC_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LayoutDesc
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
