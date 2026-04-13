use std::{any::Any, collections::BTreeMap};

use crate::{
    DBObjs::Surface::Surface,
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        GfxObjFlags::GfxObjFlags,
    },
    Lib::{
        Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj,
            IPackable::IPackable, IUnpackable::IUnpackable, Numerics::Vector3,
        },
    },
    Types::{
        BSPTrees::{DrawingBSPTree, PhysicsBSPTree},
        DBObj::{DBObj, DBObjBase},
        Polygon::Polygon,
        QualifiedDataId::QualifiedDataId,
        VertexArray::VertexArray,
    },
};

pub const GFX_OBJ_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "GfxObj",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::GfxObj,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x01000000,
    last_id: 0x0100FFFF,
    mask_id: 0x01000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GfxObj {
    pub base: DBObjBase,
    pub flags: GfxObjFlags,
    pub surfaces: Vec<QualifiedDataId<Surface>>,
    pub vertex_array: VertexArray,
    pub physics_polygons: BTreeMap<u16, Polygon>,
    pub physics_bsp: PhysicsBSPTree,
    pub sort_center: Vector3,
    pub polygons: BTreeMap<u16, Polygon>,
    pub drawing_bsp: DrawingBSPTree,
    pub did_degrade: u32,
}

impl DBObj for GfxObj {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::GfxObj
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

impl IUnpackable for GfxObj {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.flags = GfxObjFlags::from_bits_truncate(reader.read_u32());
        let surface_count = reader.read_compressed_uint() as usize;
        self.surfaces.clear();
        for _ in 0..surface_count {
            self.surfaces
                .push(reader.read_item::<QualifiedDataId<Surface>>());
        }
        self.vertex_array = reader.read_item::<VertexArray>();
        if self.flags.contains(GfxObjFlags::HasPhysics) {
            let count = reader.read_compressed_uint() as usize;
            self.physics_polygons.clear();
            for _ in 0..count {
                self.physics_polygons
                    .insert(reader.read_u16(), reader.read_item::<Polygon>());
            }
            self.physics_bsp = reader.read_item::<PhysicsBSPTree>();
        }
        self.sort_center = reader.read_vector3();
        if self.flags.contains(GfxObjFlags::HasDrawing) {
            let count = reader.read_compressed_uint() as usize;
            self.polygons.clear();
            for _ in 0..count {
                self.polygons
                    .insert(reader.read_u16(), reader.read_item::<Polygon>());
            }
            self.drawing_bsp = reader.read_item::<DrawingBSPTree>();
        }
        if self.flags.contains(GfxObjFlags::HasDIDDegrade) {
            self.did_degrade = reader.read_u32();
        }
        true
    }
}

impl IPackable for GfxObj {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.flags.bits());
        writer.write_compressed_uint(self.surfaces.len() as u32);
        for surface in &self.surfaces {
            writer.write_item(surface);
        }
        writer.write_item(&self.vertex_array);
        if self.flags.contains(GfxObjFlags::HasPhysics) {
            writer.write_compressed_uint(self.physics_polygons.len() as u32);
            for (key, polygon) in &self.physics_polygons {
                writer.write_u16(*key);
                writer.write_item(polygon);
            }
            writer.write_item(&self.physics_bsp);
        }
        writer.write_vector3(self.sort_center);
        if self.flags.contains(GfxObjFlags::HasDrawing) {
            writer.write_compressed_uint(self.polygons.len() as u32);
            for (key, polygon) in &self.polygons {
                writer.write_u16(*key);
                writer.write_item(polygon);
            }
            writer.write_item(&self.drawing_bsp);
        }
        if self.flags.contains(GfxObjFlags::HasDIDDegrade) {
            writer.write_u32(self.did_degrade);
        }
        true
    }
}

impl IDBObj for GfxObj {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &GFX_OBJ_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::GfxObj
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
