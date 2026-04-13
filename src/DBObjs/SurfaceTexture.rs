use std::any::Any;

use crate::{
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

pub const SURFACE_TEXTURE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "SurfaceTexture",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::SurfaceTexture,
    header_flags: DBObjHeaderFlags::from_bits_retain(
        DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
    ),
    first_id: 0x05000000,
    last_id: 0x05FFFFFF,
    mask_id: 0x05000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SurfaceTexture {
    pub base: DBObjBase,
    pub texture_type: TextureType,
    pub textures: Vec<QualifiedDataId<()>>,
}

impl DBObj for SurfaceTexture {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::from_bits_retain(
            DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
        )
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::SurfaceTexture
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

impl IUnpackable for SurfaceTexture {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(
            reader,
            DBObjHeaderFlags::from_bits_retain(
                DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
            ),
        );
        self.texture_type = TextureType::from(reader.read_byte());
        let count = reader.read_i32().max(0) as usize;
        self.textures.clear();
        for _ in 0..count {
            self.textures
                .push(reader.read_item::<QualifiedDataId<()>>());
        }
        true
    }
}
impl IPackable for SurfaceTexture {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(
            writer,
            DBObjHeaderFlags::from_bits_retain(
                DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
            ),
        );
        writer.write_byte(self.texture_type.0);
        writer.write_i32(self.textures.len() as i32);
        for t in &self.textures {
            let _ = t.pack(writer);
        }
        true
    }
}
impl IDBObj for SurfaceTexture {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &SURFACE_TEXTURE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::SurfaceTexture
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
