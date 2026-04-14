use crate::{
    Generated::Enums::{DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{DBObjAttributeCache, IO::IDBObj::IDBObj},
};

pub type DbObjFactory = fn() -> Box<dyn IDBObj>;

pub fn create_instance<T>() -> T
where
    T: IDBObj + Default,
{
    T::default()
}

fn create_default_boxed<T>() -> Box<dyn IDBObj>
where
    T: IDBObj + Default + 'static,
{
    Box::new(T::default())
}

pub fn get_factory(db_obj_type: DBObjType) -> Option<DbObjFactory> {
    match db_obj_type {
        DBObjType::Iteration => Some(create_default_boxed::<crate::DBObjs::Iteration::Iteration>),
        DBObjType::GfxObj => Some(create_default_boxed::<crate::DBObjs::GfxObj::GfxObj>),
        DBObjType::Setup => Some(create_default_boxed::<crate::DBObjs::Setup::Setup>),
        DBObjType::Animation => Some(create_default_boxed::<crate::DBObjs::Animation::Animation>),
        DBObjType::Palette => Some(create_default_boxed::<crate::DBObjs::Palette::Palette>),
        DBObjType::SurfaceTexture => {
            Some(create_default_boxed::<crate::DBObjs::SurfaceTexture::SurfaceTexture>)
        }
        DBObjType::RenderSurface => {
            Some(create_default_boxed::<crate::DBObjs::RenderSurface::RenderSurface>)
        }
        DBObjType::Surface => Some(create_default_boxed::<crate::DBObjs::Surface::Surface>),
        DBObjType::MotionTable => {
            Some(create_default_boxed::<crate::DBObjs::MotionTable::MotionTable>)
        }
        DBObjType::Wave => Some(create_default_boxed::<crate::DBObjs::Wave::Wave>),
        DBObjType::CharGen => Some(create_default_boxed::<crate::DBObjs::CharGen::CharGen>),
        DBObjType::PalSet => Some(create_default_boxed::<crate::DBObjs::PalSet::PalSet>),
        DBObjType::ClothingTable => {
            Some(create_default_boxed::<crate::DBObjs::ClothingTable::ClothingTable>)
        }
        DBObjType::Scene => Some(create_default_boxed::<crate::DBObjs::Scene::Scene>),
        DBObjType::Region => Some(create_default_boxed::<crate::DBObjs::Region::Region>),
        DBObjType::SoundTable => {
            Some(create_default_boxed::<crate::DBObjs::SoundTable::SoundTable>)
        }
        DBObjType::CombatTable => {
            Some(create_default_boxed::<crate::DBObjs::CombatTable::CombatTable>)
        }
        DBObjType::LanguageString => {
            Some(create_default_boxed::<crate::DBObjs::LanguageString::LanguageString>)
        }
        DBObjType::ParticleEmitter => {
            Some(create_default_boxed::<crate::DBObjs::ParticleEmitter::ParticleEmitter>)
        }
        DBObjType::PhysicsScript => {
            Some(create_default_boxed::<crate::DBObjs::PhysicsScript::PhysicsScript>)
        }
        DBObjType::PhysicsScriptTable => {
            Some(create_default_boxed::<crate::DBObjs::PhysicsScriptTable::PhysicsScriptTable>)
        }
        DBObjType::StringTable => {
            Some(create_default_boxed::<crate::DBObjs::StringTable::StringTable>)
        }
        DBObjType::VitalTable => {
            Some(create_default_boxed::<crate::DBObjs::VitalTable::VitalTable>)
        }
        DBObjType::SkillTable => {
            Some(create_default_boxed::<crate::DBObjs::SkillTable::SkillTable>)
        }
        DBObjType::ExperienceTable => {
            Some(create_default_boxed::<crate::DBObjs::ExperienceTable::ExperienceTable>)
        }
        DBObjType::Font => Some(create_default_boxed::<crate::DBObjs::Font::Font>),
        DBObjType::LanguageInfo => {
            Some(create_default_boxed::<crate::DBObjs::LanguageInfo::LanguageInfo>)
        }
        DBObjType::NameFilterTable => {
            Some(create_default_boxed::<crate::DBObjs::NameFilterTable::NameFilterTable>)
        }
        DBObjType::EnumMapper => {
            Some(create_default_boxed::<crate::DBObjs::EnumMapper::EnumMapper>)
        }
        DBObjType::EnumIDMap => Some(create_default_boxed::<crate::DBObjs::EnumIDMap::EnumIDMap>),
        DBObjType::DataIdMapper => {
            Some(create_default_boxed::<crate::DBObjs::DataIdMapper::DataIdMapper>)
        }
        DBObjType::DualEnumIDMap => {
            Some(create_default_boxed::<crate::DBObjs::DualEnumIDMap::DualEnumIDMap>)
        }
        DBObjType::DualDataIdMapper => {
            Some(create_default_boxed::<crate::DBObjs::DualDataIdMapper::DualDataIdMapper>)
        }
        DBObjType::RenderTexture => {
            Some(create_default_boxed::<crate::DBObjs::RenderTexture::RenderTexture>)
        }
        DBObjType::RenderMaterial => {
            Some(create_default_boxed::<crate::DBObjs::RenderMaterial::RenderMaterial>)
        }
        DBObjType::MaterialModifier => {
            Some(create_default_boxed::<crate::DBObjs::MaterialModifier::MaterialModifier>)
        }
        DBObjType::MaterialInstance => {
            Some(create_default_boxed::<crate::DBObjs::MaterialInstance::MaterialInstance>)
        }
        DBObjType::GfxObjDegradeInfo => {
            Some(create_default_boxed::<crate::DBObjs::GfxObjDegradeInfo::GfxObjDegradeInfo>)
        }
        DBObjType::ActionMap => Some(create_default_boxed::<crate::DBObjs::ActionMap::ActionMap>),
        DBObjType::DBProperties => {
            Some(create_default_boxed::<crate::DBObjs::DBProperties::DBProperties>)
        }
        DBObjType::SpellTable => {
            Some(create_default_boxed::<crate::DBObjs::SpellTable::SpellTable>)
        }
        DBObjType::SpellComponentTable => {
            Some(create_default_boxed::<crate::DBObjs::SpellComponentTable::SpellComponentTable>)
        }
        DBObjType::QualityFilter => {
            Some(create_default_boxed::<crate::DBObjs::QualityFilter::QualityFilter>)
        }
        DBObjType::BadDataTable => {
            Some(create_default_boxed::<crate::DBObjs::BadDataTable::BadDataTable>)
        }
        DBObjType::ChatPoseTable => {
            Some(create_default_boxed::<crate::DBObjs::ChatPoseTable::ChatPoseTable>)
        }
        DBObjType::ContractTable => {
            Some(create_default_boxed::<crate::DBObjs::ContractTable::ContractTable>)
        }
        DBObjType::LandBlock => Some(create_default_boxed::<crate::DBObjs::LandBlock::LandBlock>),
        DBObjType::EnvCell => Some(create_default_boxed::<crate::DBObjs::EnvCell::EnvCell>),
        DBObjType::Environment => {
            Some(create_default_boxed::<crate::DBObjs::Environment::Environment>)
        }
        DBObjType::LandBlockInfo => {
            Some(create_default_boxed::<crate::DBObjs::LandBlockInfo::LandBlockInfo>)
        }
        DBObjType::LayoutDesc => {
            Some(create_default_boxed::<crate::DBObjs::LayoutDesc::LayoutDesc>)
        }
        DBObjType::MasterInputMap => {
            Some(create_default_boxed::<crate::DBObjs::MasterInputMap::MasterInputMap>)
        }
        DBObjType::MasterProperty => {
            Some(create_default_boxed::<crate::DBObjs::MasterProperty::MasterProperty>)
        }
        DBObjType::ObjectHierarchy => {
            Some(create_default_boxed::<crate::DBObjs::ObjectHierarchy::ObjectHierarchy>)
        }
        DBObjType::TabooTable => {
            Some(create_default_boxed::<crate::DBObjs::TabooTable::TabooTable>)
        }
        _ => None,
    }
}

pub fn create_boxed(db_obj_type: DBObjType) -> Option<Box<dyn IDBObj>> {
    get_factory(db_obj_type).map(|factory| factory())
}

pub fn create_boxed_from_id(dat_file_type: DatFileType, id: u32) -> Option<Box<dyn IDBObj>> {
    let attr = DBObjAttributeCache::type_from_id(dat_file_type, id)?;
    create_boxed(attr.db_obj_type)
}
