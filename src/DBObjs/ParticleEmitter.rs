use std::any::Any;

use crate::{
    DBObjs::GfxObj::GfxObj,
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
        EmitterType::EmitterType, ParticleType::ParticleType,
    },
    Lib::{
        Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj,
            IPackable::IPackable, IUnpackable::IUnpackable, Numerics::Vector3,
        },
    },
    Types::{
        DBObj::{DBObj, DBObjBase},
        QualifiedDataId::QualifiedDataId,
    },
};

pub const PARTICLE_EMITTER_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "ParticleEmitter",
    dat_file_type: DatFileType::Portal,
    db_obj_type: DBObjType::ParticleEmitter,
    header_flags: DBObjHeaderFlags::HasId,
    first_id: 0x32000000,
    last_id: 0x3200FFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParticleEmitter {
    pub base: DBObjBase,
    pub unknown: u32,
    pub emitter_type: EmitterType,
    pub particle_type: ParticleType,
    pub gfx_obj_id: QualifiedDataId<GfxObj>,
    pub hw_gfx_obj_id: QualifiedDataId<GfxObj>,
    pub birthrate: f64,
    pub max_particles: i32,
    pub initial_particles: i32,
    pub total_particles: i32,
    pub total_seconds: f64,
    pub lifespan: f64,
    pub lifespan_rand: f64,
    pub offset_dir: Vector3,
    pub min_offset: f32,
    pub max_offset: f32,
    pub a: Vector3,
    pub min_a: f32,
    pub max_a: f32,
    pub b: Vector3,
    pub min_b: f32,
    pub max_b: f32,
    pub c: Vector3,
    pub min_c: f32,
    pub max_c: f32,
    pub start_scale: f32,
    pub final_scale: f32,
    pub scale_rand: f32,
    pub start_trans: f32,
    pub final_trans: f32,
    pub trans_rand: f32,
    pub is_parent_local: bool,
}

impl DBObj for ParticleEmitter {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::HasId
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ParticleEmitter
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

impl IUnpackable for ParticleEmitter {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::HasId);
        self.unknown = reader.read_u32();
        self.emitter_type = reader.read_i32().into();
        self.particle_type = reader.read_i32().into();
        self.gfx_obj_id = reader.read_item::<QualifiedDataId<GfxObj>>();
        self.hw_gfx_obj_id = reader.read_item::<QualifiedDataId<GfxObj>>();
        self.birthrate = reader.read_double();
        self.max_particles = reader.read_i32();
        self.initial_particles = reader.read_i32();
        self.total_particles = reader.read_i32();
        self.total_seconds = reader.read_double();
        self.lifespan = reader.read_double();
        self.lifespan_rand = reader.read_double();
        self.offset_dir = reader.read_vector3();
        self.min_offset = reader.read_single();
        self.max_offset = reader.read_single();
        self.a = reader.read_vector3();
        self.min_a = reader.read_single();
        self.max_a = reader.read_single();
        self.b = reader.read_vector3();
        self.min_b = reader.read_single();
        self.max_b = reader.read_single();
        self.c = reader.read_vector3();
        self.min_c = reader.read_single();
        self.max_c = reader.read_single();
        self.start_scale = reader.read_single();
        self.final_scale = reader.read_single();
        self.scale_rand = reader.read_single();
        self.start_trans = reader.read_single();
        self.final_trans = reader.read_single();
        self.trans_rand = reader.read_single();
        self.is_parent_local = reader.read_bool(4);
        true
    }
}

impl IPackable for ParticleEmitter {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::HasId);
        writer.write_u32(self.unknown);
        writer.write_i32(self.emitter_type.into());
        writer.write_i32(self.particle_type.into());
        writer.write_item(&self.gfx_obj_id);
        writer.write_item(&self.hw_gfx_obj_id);
        writer.write_double(self.birthrate);
        writer.write_i32(self.max_particles);
        writer.write_i32(self.initial_particles);
        writer.write_i32(self.total_particles);
        writer.write_double(self.total_seconds);
        writer.write_double(self.lifespan);
        writer.write_double(self.lifespan_rand);
        writer.write_vector3(self.offset_dir);
        writer.write_single(self.min_offset);
        writer.write_single(self.max_offset);
        writer.write_vector3(self.a);
        writer.write_single(self.min_a);
        writer.write_single(self.max_a);
        writer.write_vector3(self.b);
        writer.write_single(self.min_b);
        writer.write_single(self.max_b);
        writer.write_vector3(self.c);
        writer.write_single(self.min_c);
        writer.write_single(self.max_c);
        writer.write_single(self.start_scale);
        writer.write_single(self.final_scale);
        writer.write_single(self.scale_rand);
        writer.write_single(self.start_trans);
        writer.write_single(self.final_trans);
        writer.write_single(self.trans_rand);
        writer.write_bool(self.is_parent_local, 4);
        true
    }
}

impl IDBObj for ParticleEmitter {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &PARTICLE_EMITTER_ATTR
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::ParticleEmitter
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
