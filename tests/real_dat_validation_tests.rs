use dat_reader_writer::{
    DBObjs::{
        ActionMap::ActionMap, Animation::Animation, BadDataTable::BadDataTable, CharGen::CharGen,
        ChatPoseTable::ChatPoseTable, Clothing::Clothing, ClothingTable::ClothingTable,
        CombatTable::CombatTable, ContractTable::ContractTable, DBProperties::DBProperties,
        DataIdMapper::DataIdMapper, DualDataIdMapper::DualDataIdMapper,
        DualEnumIDMap::DualEnumIDMap, EnumIDMap::EnumIDMap, EnvCell::EnvCell,
        Environment::Environment, ExperienceTable::ExperienceTable, Font::Font, GfxObj::GfxObj,
        GfxObjDegradeInfo::GfxObjDegradeInfo, Iteration::Iteration, LandBlock::LandBlock,
        LandBlockInfo::LandBlockInfo, LanguageInfo::LanguageInfo, LayoutDesc::LayoutDesc,
        MasterInputMap::MasterInputMap, MasterProperty::MasterProperty,
        MaterialInstance::MaterialInstance, MaterialModifier::MaterialModifier,
        MotionTable::MotionTable, NameFilterTable::NameFilterTable,
        ObjectHierarchy::ObjectHierarchy, Palette::Palette, PaletteSet::PaletteSet,
        ParticleEmitter::ParticleEmitter, ParticleEmitterInfo::ParticleEmitterInfo,
        PhysicsScript::PhysicsScript, PhysicsScriptTable::PhysicsScriptTable,
        QualityFilter::QualityFilter, Region::Region, RenderMaterial::RenderMaterial,
        RenderSurface::RenderSurface, RenderTexture::RenderTexture, Scene::Scene, Setup::Setup,
        SkillTable::SkillTable, SoundTable::SoundTable, SpellComponentTable::SpellComponentTable,
        SpellTable::SpellTable, Surface::Surface, SurfaceTexture::SurfaceTexture,
        TabooTable::TabooTable, VitalTable::VitalTable, Wave::Wave,
    },
    DatCollection::DatCollection,
    Options::DatAccessType::DatAccessType,
};

fn real_dat_dir() -> String {
    std::env::var("DAT_READER_WRITER_REAL_DAT_DIR")
        .expect("set DAT_READER_WRITER_REAL_DAT_DIR to a real DAT directory before running this ignored test")
}

fn validate_sample<T>(collection: &DatCollection, sample_count: usize)
where
    T: dat_reader_writer::Lib::IO::IDBObj::IDBObj + Default,
{
    let ids = collection.get_all_ids_of_type::<T>().unwrap();
    assert!(
        !ids.is_empty(),
        "no ids returned for {}",
        std::any::type_name::<T>()
    );
    for id in ids.into_iter().take(sample_count) {
        let value = collection.try_get::<T>(id).unwrap();
        assert!(
            value.is_some(),
            "failed to read {} id {id:#010X}",
            std::any::type_name::<T>()
        );
    }
}

#[test]
#[ignore = "requires DAT_READER_WRITER_REAL_DAT_DIR and local retail DAT files"]
fn validates_ported_types_against_real_dats() {
    let collection = DatCollection::from_directory(real_dat_dir(), DatAccessType::Read).unwrap();

    assert!(
        collection
            .try_get::<Iteration>(0xFFFF0001)
            .unwrap()
            .is_some()
    );
    assert!(collection.try_get::<CharGen>(0x0E000002).unwrap().is_some());
    assert!(
        collection
            .try_get::<VitalTable>(0x0E000003)
            .unwrap()
            .is_some()
    );
    assert!(
        collection
            .try_get::<SkillTable>(0x0E000004)
            .unwrap()
            .is_some()
    );
    assert!(
        collection
            .try_get::<ChatPoseTable>(0x0E000007)
            .unwrap()
            .is_some()
    );
    assert!(
        collection
            .try_get::<ObjectHierarchy>(0x0E00000D)
            .unwrap()
            .is_some()
    );
    assert!(
        collection
            .try_get::<SpellTable>(0x0E00000E)
            .unwrap()
            .is_some()
    );
    assert!(
        collection
            .try_get::<ExperienceTable>(0x0E000018)
            .unwrap()
            .is_some()
    );

    validate_sample::<Palette>(&collection, 5);
    validate_sample::<PaletteSet>(&collection, 2);
    validate_sample::<SurfaceTexture>(&collection, 5);
    validate_sample::<RenderSurface>(&collection, 5);
    validate_sample::<RenderTexture>(&collection, 3);
    validate_sample::<RenderMaterial>(&collection, 3);
    validate_sample::<MaterialModifier>(&collection, 3);
    validate_sample::<MaterialInstance>(&collection, 3);
    validate_sample::<EnumIDMap>(&collection, 1);
    validate_sample::<DataIdMapper>(&collection, 1);
    validate_sample::<DualEnumIDMap>(&collection, 1);
    validate_sample::<DualDataIdMapper>(&collection, 1);
    validate_sample::<ActionMap>(&collection, 1);
    validate_sample::<DBProperties>(&collection, 1);
    validate_sample::<Environment>(&collection, 1);
    validate_sample::<LandBlockInfo>(&collection, 1);
    validate_sample::<LandBlock>(&collection, 1);
    validate_sample::<EnvCell>(&collection, 1);
    validate_sample::<LayoutDesc>(&collection, 1);
    validate_sample::<MasterInputMap>(&collection, 1);
    validate_sample::<MasterProperty>(&collection, 1);
    validate_sample::<GfxObjDegradeInfo>(&collection, 3);
    validate_sample::<MotionTable>(&collection, 5);
    validate_sample::<Setup>(&collection, 5);
    validate_sample::<Animation>(&collection, 5);
    validate_sample::<Scene>(&collection, 3);
    validate_sample::<Region>(&collection, 2);
    validate_sample::<Surface>(&collection, 5);
    validate_sample::<GfxObj>(&collection, 5);
    validate_sample::<Wave>(&collection, 3);
    validate_sample::<ParticleEmitter>(&collection, 3);
    validate_sample::<ParticleEmitterInfo>(&collection, 2);
    validate_sample::<PhysicsScript>(&collection, 5);
    validate_sample::<SoundTable>(&collection, 3);
    validate_sample::<PhysicsScriptTable>(&collection, 3);
    validate_sample::<ClothingTable>(&collection, 3);
    validate_sample::<Clothing>(&collection, 2);
    validate_sample::<CombatTable>(&collection, 1);
    validate_sample::<BadDataTable>(&collection, 1);
    validate_sample::<ContractTable>(&collection, 1);
    validate_sample::<SpellComponentTable>(&collection, 1);
    validate_sample::<TabooTable>(&collection, 1);
    validate_sample::<QualityFilter>(&collection, 1);
    validate_sample::<Font>(&collection, 3);
    validate_sample::<LanguageInfo>(&collection, 1);
    validate_sample::<NameFilterTable>(&collection, 1);
}
