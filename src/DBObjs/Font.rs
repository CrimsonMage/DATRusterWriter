use std::any::Any;

use crate::{
    DBObjs::Surface::Surface,
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
        FontCharDesc::FontCharDesc,
        QualifiedDataId::QualifiedDataId,
    },
};

pub const FONT_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "Font",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::Font,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x40000000,
    last_id: 0x40000FFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Font {
    pub base: DBObjBase,
    pub max_char_height: u32,
    pub max_char_width: u32,
    pub char_descs: Vec<FontCharDesc>,
    pub num_horizontal_border_pixels: u32,
    pub num_vertical_border_pixels: u32,
    pub baseline_offset: u32,
    pub foreground_surface_data_id: QualifiedDataId<Surface>,
    pub background_surface_data_id: QualifiedDataId<Surface>,
}

impl DBObj for Font {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::Font
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

impl IUnpackable for Font {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.max_char_height = reader.read_u32();
        self.max_char_width = reader.read_u32();

        let count = reader.read_u32() as usize;
        self.char_descs.clear();
        self.char_descs.reserve(count);
        for _ in 0..count {
            self.char_descs.push(reader.read_item::<FontCharDesc>());
        }

        self.num_horizontal_border_pixels = reader.read_u32();
        self.num_vertical_border_pixels = reader.read_u32();
        self.baseline_offset = reader.read_u32();
        self.foreground_surface_data_id = reader.read_item::<QualifiedDataId<Surface>>();
        self.background_surface_data_id = reader.read_item::<QualifiedDataId<Surface>>();
        true
    }
}

impl IPackable for Font {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.max_char_height);
        writer.write_u32(self.max_char_width);
        writer.write_u32(self.char_descs.len() as u32);
        for char_desc in &self.char_descs {
            writer.write_item(char_desc);
        }
        writer.write_u32(self.num_horizontal_border_pixels);
        writer.write_u32(self.num_vertical_border_pixels);
        writer.write_u32(self.baseline_offset);
        writer.write_item(&self.foreground_surface_data_id);
        writer.write_item(&self.background_surface_data_id);
        true
    }
}

impl IDBObj for Font {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &FONT_ATTR
    }

    fn db_obj_type(&self) -> DBObjType {
        DBObjType::Font
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
