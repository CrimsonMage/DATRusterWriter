use crate::{
    Generated::Enums::{DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
};

const PORTED_ATTRIBUTES: &[&DBObjTypeAttribute] = &[
    &crate::DBObjs::Iteration::ITERATION_ATTR,
    &crate::DBObjs::GfxObj::GFX_OBJ_ATTR,
    &crate::DBObjs::Setup::SETUP_ATTR,
    &crate::DBObjs::Animation::ANIMATION_ATTR,
    &crate::DBObjs::Palette::PALETTE_ATTR,
    &crate::DBObjs::SurfaceTexture::SURFACE_TEXTURE_ATTR,
    &crate::DBObjs::RenderSurface::RENDER_SURFACE_ATTR,
    &crate::DBObjs::Surface::SURFACE_ATTR,
    &crate::DBObjs::MotionTable::MOTION_TABLE_ATTR,
    &crate::DBObjs::Wave::WAVE_ATTR,
    &crate::DBObjs::CharGen::CHAR_GEN_ATTR,
    &crate::DBObjs::PalSet::PAL_SET_ATTR,
    &crate::DBObjs::ClothingTable::CLOTHING_TABLE_ATTR,
    &crate::DBObjs::Scene::SCENE_ATTR,
    &crate::DBObjs::Region::REGION_ATTR,
    &crate::DBObjs::SoundTable::SOUND_TABLE_ATTR,
    &crate::DBObjs::CombatTable::COMBAT_TABLE_ATTR,
    &crate::DBObjs::LanguageString::LANGUAGE_STRING_ATTR,
    &crate::DBObjs::ParticleEmitter::PARTICLE_EMITTER_ATTR,
    &crate::DBObjs::PhysicsScript::PHYSICS_SCRIPT_ATTR,
    &crate::DBObjs::PhysicsScriptTable::PHYSICS_SCRIPT_TABLE_ATTR,
    &crate::DBObjs::StringTable::STRING_TABLE_ATTR,
];

fn dat_type_matches(attr: &DBObjTypeAttribute, dat_type: DatFileType) -> bool {
    attr.dat_file_type == DatFileType::Undefined || attr.dat_file_type == dat_type
}

fn attr_matches_id(attr: &DBObjTypeAttribute, id: u32) -> bool {
    if attr.is_singular() {
        return id == attr.first_id;
    }

    if attr.has_range_data() {
        return id >= attr.first_id && id <= attr.last_id;
    }

    if attr.has_mask() {
        return (id & 0x0000_FFFF) == attr.mask_id;
    }

    false
}

pub fn all_ported_attributes() -> &'static [&'static DBObjTypeAttribute] {
    PORTED_ATTRIBUTES
}

pub fn type_from_id(dat_type: DatFileType, id: u32) -> Option<&'static DBObjTypeAttribute> {
    PORTED_ATTRIBUTES
        .iter()
        .copied()
        .find(|attr| dat_type_matches(attr, dat_type) && attr_matches_id(attr, id))
}

pub fn db_obj_type_from_id(dat_type: DatFileType, id: u32) -> DBObjType {
    type_from_id(dat_type, id)
        .map(|attr| attr.db_obj_type)
        .unwrap_or(DBObjType::Unknown)
}
