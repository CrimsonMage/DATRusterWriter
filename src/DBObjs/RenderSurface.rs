use std::any::Any;

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        PixelFormat::PixelFormat,
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

pub const RENDER_SURFACE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "RenderSurface",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::RenderSurface,
    header_flags: DBObjHeaderFlags::from_bits_retain(
        DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
    ),
    first_id: 0x06000000,
    last_id: 0x07FFFFFF,
    mask_id: 0x06000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderSurface {
    pub base: DBObjBase,
    pub width: i32,
    pub height: i32,
    pub format: PixelFormat,
    pub source_data: Vec<u8>,
    pub default_palette_id: u32,
}

impl DBObj for RenderSurface {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::from_bits_retain(
            DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
        )
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::RenderSurface
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

impl IUnpackable for RenderSurface {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(
            reader,
            DBObjHeaderFlags::from_bits_retain(
                DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
            ),
        );
        self.width = reader.read_i32();
        self.height = reader.read_i32();
        self.format = PixelFormat::from(reader.read_u32());
        let len = reader.read_i32().max(0) as usize;
        self.source_data = reader.read_bytes(len);
        if self.format == PixelFormat::PFID_INDEX16 || self.format == PixelFormat::PFID_P8 {
            self.default_palette_id = reader.read_u32();
        }
        true
    }
}
impl IPackable for RenderSurface {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(
            writer,
            DBObjHeaderFlags::from_bits_retain(
                DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
            ),
        );
        writer.write_i32(self.width);
        writer.write_i32(self.height);
        writer.write_u32(self.format.0);
        writer.write_i32(self.source_data.len() as i32);
        writer.write_bytes(&self.source_data, self.source_data.len());
        if self.format == PixelFormat::PFID_INDEX16 || self.format == PixelFormat::PFID_P8 {
            writer.write_u32(self.default_palette_id);
        }
        true
    }
}
impl IDBObj for RenderSurface {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &RENDER_SURFACE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::RenderSurface
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
