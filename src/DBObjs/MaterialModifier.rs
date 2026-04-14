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
    Types::{DBObj::{DBObj, DBObjBase}, MaterialProperty::MaterialProperty},
};

pub const MATERIAL_MODIFIER_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "MaterialModifier",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::MaterialModifier,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x17000000,
    last_id: 0x17FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MaterialModifier {
    pub base: DBObjBase,
    pub material_properties: Vec<MaterialProperty>,
}

impl DBObj for MaterialModifier {
    fn header_flags(&self) -> DBObjHeaderFlags { DBObjHeaderFlags::HasId }
    fn db_obj_type(&self) -> DBObjType { DBObjType::MaterialModifier }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn data_category(&self) -> u32 { self.base.data_category }
    fn set_data_category(&mut self, data_category: u32) { self.base.data_category = data_category; }
}

impl IUnpackable for MaterialModifier {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        let count = reader.read_u32() as usize;
        self.material_properties.clear();
        self.material_properties.reserve(count);
        for _ in 0..count {
            self.material_properties.push(reader.read_item::<MaterialProperty>());
        }
        true
    }
}

impl IPackable for MaterialModifier {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.material_properties.len() as u32);
        for item in &self.material_properties {
            writer.write_item(item);
        }
        true
    }
}

impl IDBObj for MaterialModifier {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute where Self: Sized { &MATERIAL_MODIFIER_ATTR }
    fn db_obj_type(&self) -> DBObjType { DBObjType::MaterialModifier }
    fn id(&self) -> u32 { self.base.id }
    fn set_id(&mut self, id: u32) { self.base.id = id; }
    fn as_any(&self) -> &dyn Any { self }
}
