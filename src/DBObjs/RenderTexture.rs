use std::any::Any;

use crate::{
    DBObjs::RenderSurface::RenderSurface,
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        TextureType::TextureType,
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
        QualifiedDataId::QualifiedDataId,
    },
};

pub const RENDER_TEXTURE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "RenderTexture",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::RenderTexture,
    header_flags: DBObjHeaderFlags::from_bits_retain(
        DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
    ),
    first_id: 0x15000000,
    last_id: 0x15FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderTexture {
    pub base: DBObjBase,
    pub texture_type: TextureType,
    pub source_levels: Vec<QualifiedDataId<RenderSurface>>,
}

impl DBObj for RenderTexture {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::from_bits_retain(
            DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
        )
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::RenderTexture
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

impl IUnpackable for RenderTexture {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(
            reader,
            DBObjHeaderFlags::from_bits_retain(
                DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
            ),
        );
        self.texture_type = TextureType::from(reader.read_byte());
        let count = reader.read_u32() as usize;
        self.source_levels.clear();
        self.source_levels.reserve(count);
        for _ in 0..count {
            self.source_levels
                .push(reader.read_item::<QualifiedDataId<RenderSurface>>());
        }
        true
    }
}

impl IPackable for RenderTexture {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(
            writer,
            DBObjHeaderFlags::from_bits_retain(
                DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
            ),
        );
        writer.write_byte(self.texture_type.into());
        writer.write_u32(self.source_levels.len() as u32);
        for item in &self.source_levels {
            writer.write_item(item);
        }
        true
    }
}

impl IDBObj for RenderTexture {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &RENDER_TEXTURE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::RenderTexture
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}
