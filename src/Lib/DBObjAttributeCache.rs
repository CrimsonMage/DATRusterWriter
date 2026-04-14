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
    &crate::DBObjs::VitalTable::VITAL_TABLE_ATTR,
    &crate::DBObjs::SkillTable::SKILL_TABLE_ATTR,
    &crate::DBObjs::ExperienceTable::EXPERIENCE_TABLE_ATTR,
    &crate::DBObjs::Font::FONT_ATTR,
    &crate::DBObjs::LanguageInfo::LANGUAGE_INFO_ATTR,
    &crate::DBObjs::NameFilterTable::NAME_FILTER_TABLE_ATTR,
    &crate::DBObjs::EnumMapper::ENUM_MAPPER_ATTR,
    &crate::DBObjs::EnumIDMap::ENUM_ID_MAP_ATTR,
    &crate::DBObjs::DualEnumIDMap::DUAL_ENUM_ID_MAP_ATTR,
    &crate::DBObjs::RenderTexture::RENDER_TEXTURE_ATTR,
    &crate::DBObjs::RenderMaterial::RENDER_MATERIAL_ATTR,
    &crate::DBObjs::MaterialModifier::MATERIAL_MODIFIER_ATTR,
    &crate::DBObjs::MaterialInstance::MATERIAL_INSTANCE_ATTR,
    &crate::DBObjs::GfxObjDegradeInfo::GFX_OBJ_DEGRADE_INFO_ATTR,
    &crate::DBObjs::ActionMap::ACTION_MAP_ATTR,
    &crate::DBObjs::SpellTable::SPELL_TABLE_ATTR,
    &crate::DBObjs::SpellComponentTable::SPELL_COMPONENT_TABLE_ATTR,
    &crate::DBObjs::QualityFilter::QUALITY_FILTER_ATTR,
    &crate::DBObjs::BadDataTable::BAD_DATA_TABLE_ATTR,
    &crate::DBObjs::ChatPoseTable::CHAT_POSE_TABLE_ATTR,
    &crate::DBObjs::ContractTable::CONTRACT_TABLE_ATTR,
    &crate::DBObjs::LandBlock::LAND_BLOCK_ATTR,
    &crate::DBObjs::EnvCell::ENV_CELL_ATTR,
    &crate::DBObjs::Environment::ENVIRONMENT_ATTR,
    &crate::DBObjs::LandBlockInfo::LAND_BLOCK_INFO_ATTR,
    &crate::DBObjs::MasterInputMap::MASTER_INPUT_MAP_ATTR,
    &crate::DBObjs::MasterProperty::MASTER_PROPERTY_ATTR,
    &crate::DBObjs::ObjectHierarchy::OBJECT_HIERARCHY_ATTR,
    &crate::DBObjs::TabooTable::TABOO_TABLE_ATTR,
];

fn dat_type_matches(attr: &DBObjTypeAttribute, dat_type: DatFileType) -> bool {
    attr.dat_file_type == DatFileType::Undefined || attr.dat_file_type == dat_type
}

fn attrs_for_dat_type(dat_type: DatFileType) -> impl Iterator<Item = &'static DBObjTypeAttribute> {
    PORTED_ATTRIBUTES
        .iter()
        .copied()
        .filter(move |attr| dat_type_matches(attr, dat_type))
}

fn exact_id_matches(attr: &DBObjTypeAttribute, id: u32) -> bool {
    attr.is_singular() && id == attr.first_id
}

fn range_matches(attr: &DBObjTypeAttribute, id: u32) -> bool {
    attr.has_range_data() && id >= attr.first_id && id <= attr.last_id
}

fn mask_matches(attr: &DBObjTypeAttribute, id: u32) -> bool {
    attr.has_mask() && (id & 0x0000_FFFF) == attr.mask_id
}

pub fn all_ported_attributes() -> &'static [&'static DBObjTypeAttribute] {
    PORTED_ATTRIBUTES
}

pub fn type_from_id(dat_type: DatFileType, id: u32) -> Option<&'static DBObjTypeAttribute> {
    if id == 0xFFFF0001 {
        return Some(&crate::DBObjs::Iteration::ITERATION_ATTR);
    }

    match dat_type {
        DatFileType::Cell => {
            if (id & 0xFF00_0000) == 0x0D00_0000 {
                return Some(&crate::DBObjs::Environment::ENVIRONMENT_ATTR);
            }
            if (id & 0x0000_FFFF) == 0x0000_FFFE {
                return Some(&crate::DBObjs::LandBlockInfo::LAND_BLOCK_INFO_ATTR);
            }
            if (id & 0x0000_FFFF) == 0x0000_FFFF {
                return Some(&crate::DBObjs::LandBlock::LAND_BLOCK_ATTR);
            }
            Some(&crate::DBObjs::EnvCell::ENV_CELL_ATTR)
        }
        DatFileType::Portal => attrs_for_dat_type(dat_type)
            .find(|attr| exact_id_matches(attr, id) || range_matches(attr, id)),
        DatFileType::Local => attrs_for_dat_type(dat_type).find(|attr| {
            exact_id_matches(attr, id) || mask_matches(attr, id) || range_matches(attr, id)
        }),
        DatFileType::Undefined => attrs_for_dat_type(dat_type).find(|attr| {
            exact_id_matches(attr, id) || mask_matches(attr, id) || range_matches(attr, id)
        }),
    }
}

pub fn db_obj_type_from_id(dat_type: DatFileType, id: u32) -> DBObjType {
    type_from_id(dat_type, id)
        .map(|attr| attr.db_obj_type)
        .unwrap_or(DBObjType::Unknown)
}
