use crate::{
    Generated::Enums::{DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
};

pub fn type_from_id(dat_type: DatFileType, id: u32) -> Option<&'static DBObjTypeAttribute> {
    if id == 0xFFFF0001 {
        return Some(&crate::DBObjs::Iteration::ITERATION_ATTR);
    }

    match dat_type {
        DatFileType::Portal => match id {
            0x01000000..=0x0100FFFF => Some(&crate::DBObjs::GfxObj::GFX_OBJ_ATTR),
            0x04000000..=0x0400FFFF => Some(&crate::DBObjs::Palette::PALETTE_ATTR),
            0x05000000..=0x05FFFFFF => Some(&crate::DBObjs::SurfaceTexture::SURFACE_TEXTURE_ATTR),
            0x06000000..=0x07FFFFFF => Some(&crate::DBObjs::RenderSurface::RENDER_SURFACE_ATTR),
            0x08000000..=0x0800FFFF => Some(&crate::DBObjs::Surface::SURFACE_ATTR),
            0x09000000..=0x0900FFFF => Some(&crate::DBObjs::MotionTable::MOTION_TABLE_ATTR),
            0x0A000000..=0x0A00FFFF => Some(&crate::DBObjs::Wave::WAVE_ATTR),
            0x12000000..=0x1200FFFF => Some(&crate::DBObjs::Scene::SCENE_ATTR),
            0x13000000..=0x1300FFFF => Some(&crate::DBObjs::Region::REGION_ATTR),
            0x32000000..=0x3200FFFF => Some(&crate::DBObjs::ParticleEmitter::PARTICLE_EMITTER_ATTR),
            0x33000000..=0x3300FFFF => Some(&crate::DBObjs::PhysicsScript::PHYSICS_SCRIPT_ATTR),
            _ => None,
        },
        _ => None,
    }
}

pub fn db_obj_type_from_id(dat_type: DatFileType, id: u32) -> DBObjType {
    type_from_id(dat_type, id)
        .map(|attr| attr.db_obj_type)
        .unwrap_or(DBObjType::Unknown)
}
