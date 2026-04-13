use std::any::Any;

use crate::{
    DBObjs::{Palette::Palette, SurfaceTexture::SurfaceTexture},
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        SurfaceType::SurfaceType,
    },
    Lib::{
        Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj,
            IPackable::IPackable, IUnpackable::IUnpackable,
        },
    },
    Types::{
        ColorARGB::ColorARGB,
        DBObj::{DBObj, DBObjBase},
        QualifiedDataId::QualifiedDataId,
    },
};

pub const SURFACE_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "Surface",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::Surface,
    header_flags: DBObjHeaderFlags::None,
    first_id: 0x08000000,
    last_id: 0x0800FFFF,
    mask_id: 0x08000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Surface {
    pub base: DBObjBase,
    pub surface_type: SurfaceType,
    pub orig_texture_id: QualifiedDataId<SurfaceTexture>,
    pub orig_palette_id: QualifiedDataId<Palette>,
    pub color_value: ColorARGB,
    pub translucency: f32,
    pub luminosity: f32,
    pub diffuse: f32,
}

impl DBObj for Surface {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::None
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::Surface
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

impl IUnpackable for Surface {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::None);
        self.surface_type = SurfaceType::from_bits_truncate(reader.read_u32());
        if self
            .surface_type
            .intersects(SurfaceType::Base1Image | SurfaceType::Base1ClipMap)
        {
            self.orig_texture_id = reader.read_item::<QualifiedDataId<SurfaceTexture>>();
            self.orig_palette_id = reader.read_item::<QualifiedDataId<Palette>>();
        } else {
            self.color_value = reader.read_item::<ColorARGB>();
        }
        self.translucency = reader.read_single();
        self.luminosity = reader.read_single();
        self.diffuse = reader.read_single();
        true
    }
}

impl IPackable for Surface {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::None);
        writer.write_u32(self.surface_type.bits());
        if self
            .surface_type
            .intersects(SurfaceType::Base1Image | SurfaceType::Base1ClipMap)
        {
            writer.write_item(&self.orig_texture_id);
            writer.write_item(&self.orig_palette_id);
        } else {
            writer.write_item(&self.color_value);
        }
        writer.write_single(self.translucency);
        writer.write_single(self.luminosity);
        writer.write_single(self.diffuse);
        true
    }
}

impl IDBObj for Surface {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &SURFACE_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::Surface
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
