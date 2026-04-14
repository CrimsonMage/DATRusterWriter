use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use futures::executor::block_on;

use dat_reader_writer::{
    DBObjs::{
        ActionMap::ActionMap, BadDataTable::BadDataTable, ChatPoseTable::ChatPoseTable,
        Clothing::Clothing, ContractTable::ContractTable, DBProperties::DBProperties,
        DataIdMapper::DataIdMapper, DualDataIdMapper::DualDataIdMapper,
        DualEnumIDMap::DualEnumIDMap, EnumIDMap::EnumIDMap, EnumMapper::EnumMapper,
        EnvCell::EnvCell, Environment::Environment, Font::Font,
        GfxObjDegradeInfo::GfxObjDegradeInfo, Iteration::Iteration, LandBlock::LandBlock,
        LandBlockInfo::LandBlockInfo, LanguageInfo::LanguageInfo, LanguageString::LanguageString,
        LayoutDesc::LayoutDesc, MasterInputMap::MasterInputMap, MasterProperty::MasterProperty,
        MaterialInstance::MaterialInstance, MaterialModifier::MaterialModifier,
        NameFilterTable::NameFilterTable, ObjectHierarchy::ObjectHierarchy, Palette::Palette,
        PaletteSet::PaletteSet, ParticleEmitterInfo::ParticleEmitterInfo,
        QualityFilter::QualityFilter, RenderMaterial::RenderMaterial, RenderTexture::RenderTexture,
        SpellComponentTable::SpellComponentTable, SpellTable::SpellTable, StringTable::StringTable,
        TabooTable::TabooTable,
    },
    DatCollection::DatCollection,
    DatDatabase::DatDatabase,
    Generated::Enums::{
        DBObjType::DBObjType, DatFileType::DatFileType, EnvCellFlags::EnvCellFlags,
        EquipmentSet::EquipmentSet, IncorporationFlags::IncorporationFlags, ItemType::ItemType,
        MagicSchool::MagicSchool, NumberingType::NumberingType, PlayScript::PlayScript,
        PortalFlags::PortalFlags, SpellCategory::SpellCategory, SpellIndex::SpellIndex,
        SpellType::SpellType, ToggleType::ToggleType, UIStateId::UIStateId,
    },
    Lib::{
        DBObjAttributeCache,
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, DatHeader::DatHeader,
            IPackable::IPackable, IUnpackable::IUnpackable,
        },
    },
    Options::{DatAccessType::DatAccessType, DatDatabaseOptions::DatDatabaseOptions},
    Types::{
        AC1LegacyPStringBase::AC1LegacyPStringBase,
        ActionMapValue::ActionMapValue,
        AnimationDoneHook::AnimationDoneHook,
        AnimationHook::AnimationHook,
        AttackCone::AttackCone,
        AttackHook::AttackHook,
        AutoGrowHashTable::AutoGrowHashTable,
        BSPTrees::{CellBSPNode, CellBSPTree},
        BannedPatterns::BannedPatterns,
        BaseProperty::{BaseProperty, BasePropertyHeader},
        BasePropertyDesc::BasePropertyDesc,
        Bitfield32BaseProperty::Bitfield32BaseProperty,
        Bitfield64BaseProperty::Bitfield64BaseProperty,
        BoolBaseProperty::BoolBaseProperty,
        BuildingInfo::BuildingInfo,
        BuildingPortal::BuildingPortal,
        CInputMap::CInputMap,
        CallPESHook::CallPESHook,
        CellPortal::CellPortal,
        CellStruct::CellStruct,
        ChatEmoteData::ChatEmoteData,
        ColorARGB::ColorARGB,
        ColorBaseProperty::ColorBaseProperty,
        Contract::Contract,
        CreateBlockingParticleHook::CreateBlockingParticleHook,
        CreateParticleHook::CreateParticleHook,
        DBObj::DBObjBase,
        DataIdBaseProperty::DataIdBaseProperty,
        DefaultScriptHook::DefaultScriptHook,
        DefaultScriptPartHook::DefaultScriptPartHook,
        DestroyParticleHook::DestroyParticleHook,
        DeviceKeyMapEntry::DeviceKeyMapEntry,
        DiffuseHook::DiffuseHook,
        DiffusePartHook::DiffusePartHook,
        ElementDesc::ElementDesc,
        EnumBaseProperty::EnumBaseProperty,
        EnumMapperData::EnumMapperData,
        EtherealHook::EtherealHook,
        FloatBaseProperty::FloatBaseProperty,
        FontCharDesc::FontCharDesc,
        Frame::Frame,
        HashTable::HashTable,
        InputsConflictsValue::InputsConflictsValue,
        InstanceIdBaseProperty::InstanceIdBaseProperty,
        IntegerBaseProperty::IntegerBaseProperty,
        LM_UVRotate::LM_UVRotate,
        LM_UVScale::LM_UVScale,
        LM_UVTransform::LM_UVTransform,
        LM_UVTranslate::LM_UVTranslate,
        LayerModifier::LayerModifier,
        LayerStage::LayerStage,
        LuminousHook::LuminousHook,
        LuminousPartHook::LuminousPartHook,
        MaterialLayer::MaterialLayer,
        MaterialProperty::MaterialProperty,
        NameFilterLanguageData::NameFilterLanguageData,
        NoDrawHook::NoDrawHook,
        ObfuscatedPStringBase::ObfuscatedPStringBase,
        ObjHierarchyNode::ObjHierarchyNode,
        PHashTable::PHashTable,
        PStringBase::PStringBase,
        PackedQualifiedDataId::PackedQualifiedDataId,
        PortalPoly::PortalPoly,
        Position::Position,
        QualifiedControl::QualifiedControl,
        QualifiedDataId::QualifiedDataId,
        ReplaceObjectHook::ReplaceObjectHook,
        ScaleHook::ScaleHook,
        SetLightHook::SetLightHook,
        SetOmegaHook::SetOmegaHook,
        ShaderResourceEntry::ShaderResourceEntry,
        SoundHook::SoundHook,
        SoundTableHook::SoundTableHook,
        SoundTweakedHook::SoundTweakedHook,
        SpellBase::SpellBase,
        SpellComponentBase::SpellComponentBase,
        SpellSet::SpellSet,
        SpellSetTiers::SpellSetTiers,
        Stab::Stab,
        StateDesc::StateDesc,
        StopParticleHook::StopParticleHook,
        StringTableString::StringTableString,
        SubPalette::SubPalette,
        TabooTableEntry::TabooTableEntry,
        TerrainInfo::TerrainInfo,
        TextureMapChange::TextureMapChange,
        TextureVelocityHook::TextureVelocityHook,
        TextureVelocityPartHook::TextureVelocityPartHook,
        TransparentHook::TransparentHook,
        TransparentPartHook::TransparentPartHook,
        UserBindingData::UserBindingData,
        VectorBaseProperty::VectorBaseProperty,
        Waveform::Waveform,
    },
};
use uuid::Uuid;

static UNIQUE_TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = UNIQUE_TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("dat_reader_writer_typed_{stamp}_{counter}"));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn unique_temp_file() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = UNIQUE_TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir();
    dir.join(format!("dat_reader_writer_typed_{stamp}_{counter}.dat"))
}

fn build_single_block_dat(dat_file_type: DatFileType, file_id: u32, payload: &[u8]) -> Vec<u8> {
    let block_size = 1024usize;
    let root_offset = 1024usize;
    let file_offset = 2048usize;

    let mut header = DatHeader::new(
        dat_file_type,
        0,
        block_size as i32,
        Some("test".to_string()),
        1,
        1,
        Uuid::nil(),
        1,
    );
    header.root_block = root_offset as i32;
    header.file_size = (file_offset + block_size) as i32;

    let mut root_node =
        dat_reader_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::new(root_offset as i32);
    root_node.file_count = 1;
    root_node.files[0] = dat_reader_writer::Lib::IO::DatBTree::DatBTreeFile::DatBTreeFile {
        version: 2,
        id: file_id,
        offset: file_offset as i32,
        size: payload.len() as u32,
        iteration: 1,
        ..Default::default()
    };

    let mut bytes = vec![0u8; file_offset + block_size];
    assert!(header.pack(&mut DatBinWriter::new(&mut bytes[..DatHeader::SIZE])));

    let mut node_bytes =
        vec![0u8; dat_reader_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::SIZE];
    assert!(root_node.pack(&mut DatBinWriter::new(&mut node_bytes)));
    bytes[root_offset + 4..root_offset + 4 + node_bytes.len()].copy_from_slice(&node_bytes);
    bytes[file_offset + 4..file_offset + 4 + payload.len()].copy_from_slice(payload);
    bytes
}

fn build_multi_block_dat(dat_file_type: DatFileType, entries: &[(u32, Vec<u8>)]) -> Vec<u8> {
    let block_size = 1024usize;
    let root_offset = 1024usize;
    let first_file_offset = 2048usize;

    let mut header = DatHeader::new(
        dat_file_type,
        0,
        block_size as i32,
        Some("test".to_string()),
        1,
        1,
        Uuid::nil(),
        1,
    );
    header.root_block = root_offset as i32;
    let total_len =
        (root_offset + 4 + node_bytes_len()).max(first_file_offset + (entries.len() * block_size));
    header.file_size = total_len as i32;

    let mut root_node =
        dat_reader_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::new(root_offset as i32);
    root_node.file_count = entries.len();
    for (index, (file_id, payload)) in entries.iter().enumerate() {
        root_node.files[index] = dat_reader_writer::Lib::IO::DatBTree::DatBTreeFile::DatBTreeFile {
            version: 2,
            id: *file_id,
            offset: (first_file_offset + (index * block_size)) as i32,
            size: payload.len() as u32,
            iteration: 1,
            ..Default::default()
        };
    }

    let mut bytes = vec![0u8; total_len];
    assert!(header.pack(&mut DatBinWriter::new(&mut bytes[..DatHeader::SIZE])));

    let mut node_bytes =
        vec![0u8; dat_reader_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::SIZE];
    assert!(root_node.pack(&mut DatBinWriter::new(&mut node_bytes)));
    bytes[root_offset + 4..root_offset + 4 + node_bytes.len()].copy_from_slice(&node_bytes);

    for (index, (_, payload)) in entries.iter().enumerate() {
        let file_offset = first_file_offset + (index * block_size);
        bytes[file_offset + 4..file_offset + 4 + payload.len()].copy_from_slice(payload);
    }

    bytes
}

fn node_bytes_len() -> usize {
    dat_reader_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::SIZE
}

#[test]
fn db_obj_attribute_cache_resolves_iteration() {
    let attr = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0xFFFF0001).unwrap();
    assert_eq!(DBObjType::Iteration, attr.db_obj_type);
}

#[test]
fn db_obj_attribute_cache_resolves_ported_range_and_singular_types() {
    let palette = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x04000010).unwrap();
    let clothing = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x10000040).unwrap();
    let char_gen = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E000002).unwrap();
    let language_string =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x31000010).unwrap();
    let string_table = DBObjAttributeCache::type_from_id(DatFileType::Local, 0x23000001).unwrap();
    let font = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x40000010).unwrap();
    let language_info = DBObjAttributeCache::type_from_id(DatFileType::Local, 0x41000010).unwrap();
    let name_filter = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E000020).unwrap();
    let enum_mapper = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x22000010).unwrap();
    let enum_id_map = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x25000010).unwrap();
    let dual_enum_id_map =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x27000010).unwrap();
    let render_texture =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x15000010).unwrap();
    let render_material =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x16000010).unwrap();
    let material_modifier =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x17000010).unwrap();
    let material_instance =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x18000010).unwrap();
    let gfx_obj_degrade_info =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x11000010).unwrap();
    let action_map = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x26000010).unwrap();
    let spell_component_table =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E00000F).unwrap();
    let spell_table = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E00000E).unwrap();
    let quality_filter =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E010010).unwrap();
    let bad_data_table =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E00001A).unwrap();
    let chat_pose_table =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E000007).unwrap();
    let contract_table =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E00001D).unwrap();
    let db_properties = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x78000010).unwrap();
    let environment = DBObjAttributeCache::type_from_id(DatFileType::Cell, 0x0D000123).unwrap();
    let land_block_info = DBObjAttributeCache::type_from_id(DatFileType::Cell, 0x0001FFFE).unwrap();
    let land_block = DBObjAttributeCache::type_from_id(DatFileType::Cell, 0x0001FFFF).unwrap();
    let env_cell = DBObjAttributeCache::type_from_id(DatFileType::Cell, 0x00010123).unwrap();
    let layout_desc = DBObjAttributeCache::type_from_id(DatFileType::Local, 0x21000010).unwrap();
    let master_input_map =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x14000010).unwrap();
    let master_property =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x39000010).unwrap();
    let object_hierarchy =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E00000D).unwrap();
    let taboo_table = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E00001E).unwrap();

    assert_eq!(DBObjType::Palette, palette.db_obj_type);
    assert_eq!(DBObjType::ClothingTable, clothing.db_obj_type);
    assert_eq!(DBObjType::CharGen, char_gen.db_obj_type);
    assert_eq!(DBObjType::LanguageString, language_string.db_obj_type);
    assert_eq!(DBObjType::StringTable, string_table.db_obj_type);
    assert_eq!(DBObjType::Font, font.db_obj_type);
    assert_eq!(DBObjType::LanguageInfo, language_info.db_obj_type);
    assert_eq!(DBObjType::NameFilterTable, name_filter.db_obj_type);
    assert_eq!(DBObjType::EnumMapper, enum_mapper.db_obj_type);
    assert_eq!(DBObjType::EnumIDMap, enum_id_map.db_obj_type);
    assert_eq!(DBObjType::DualEnumIDMap, dual_enum_id_map.db_obj_type);
    assert_eq!(DBObjType::RenderTexture, render_texture.db_obj_type);
    assert_eq!(DBObjType::RenderMaterial, render_material.db_obj_type);
    assert_eq!(DBObjType::MaterialModifier, material_modifier.db_obj_type);
    assert_eq!(DBObjType::MaterialInstance, material_instance.db_obj_type);
    assert_eq!(
        DBObjType::GfxObjDegradeInfo,
        gfx_obj_degrade_info.db_obj_type
    );
    assert_eq!(DBObjType::ActionMap, action_map.db_obj_type);
    assert_eq!(DBObjType::SpellTable, spell_table.db_obj_type);
    assert_eq!(
        DBObjType::SpellComponentTable,
        spell_component_table.db_obj_type
    );
    assert_eq!(DBObjType::QualityFilter, quality_filter.db_obj_type);
    assert_eq!(DBObjType::BadDataTable, bad_data_table.db_obj_type);
    assert_eq!(DBObjType::ChatPoseTable, chat_pose_table.db_obj_type);
    assert_eq!(DBObjType::ContractTable, contract_table.db_obj_type);
    assert_eq!(DBObjType::DBProperties, db_properties.db_obj_type);
    assert_eq!(DBObjType::Environment, environment.db_obj_type);
    assert_eq!(DBObjType::LandBlockInfo, land_block_info.db_obj_type);
    assert_eq!(DBObjType::LandBlock, land_block.db_obj_type);
    assert_eq!(DBObjType::EnvCell, env_cell.db_obj_type);
    assert_eq!(DBObjType::LayoutDesc, layout_desc.db_obj_type);
    assert_eq!(DBObjType::MasterInputMap, master_input_map.db_obj_type);
    assert_eq!(DBObjType::MasterProperty, master_property.db_obj_type);
    assert_eq!(DBObjType::ObjectHierarchy, object_hierarchy.db_obj_type);
    assert_eq!(DBObjType::TabooTable, taboo_table.db_obj_type);
}

#[test]
fn db_obj_attribute_cache_tracks_current_ported_dbobjs() {
    let attrs = DBObjAttributeCache::all_ported_attributes();
    assert!(attrs.iter().any(|attr| attr.rust_type_name == "Clothing"));
    assert!(attrs.iter().any(|attr| attr.rust_type_name == "PaletteSet"));
    assert!(
        attrs
            .iter()
            .any(|attr| attr.rust_type_name == "ParticleEmitterInfo")
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::Iteration)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::Palette)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::ClothingTable)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::LanguageString)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::StringTable)
    );
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::Font));
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::LanguageInfo)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::NameFilterTable)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::EnumMapper)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::EnumIDMap)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::DualEnumIDMap)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::RenderTexture)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::RenderMaterial)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::MaterialModifier)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::MaterialInstance)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::GfxObjDegradeInfo)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::ActionMap)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::SpellTable)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::SpellComponentTable)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::QualityFilter)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::BadDataTable)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::ChatPoseTable)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::ContractTable)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::DBProperties)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::EnvCell)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::Environment)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::LandBlock)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::LandBlockInfo)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::LayoutDesc)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::MasterInputMap)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::MasterProperty)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::ObjectHierarchy)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::TabooTable)
    );
}

#[test]
fn db_obj_attribute_cache_exposes_exact_mask_and_range_groups() {
    let exact = DBObjAttributeCache::all_exact_id_attributes();
    let masked = DBObjAttributeCache::all_masked_attributes();
    let ranges = DBObjAttributeCache::all_range_attributes();

    assert!(exact.iter().any(|attr| attr.db_obj_type == DBObjType::Iteration));
    assert!(exact.iter().any(|attr| attr.db_obj_type == DBObjType::CharGen));
    assert!(masked.iter().any(|attr| attr.db_obj_type == DBObjType::Palette));
    assert!(masked.iter().any(|attr| attr.db_obj_type == DBObjType::Animation));
    assert!(ranges.iter().any(|attr| attr.db_obj_type == DBObjType::StringTable));
    assert!(ranges.iter().any(|attr| attr.db_obj_type == DBObjType::LayoutDesc));
    assert!(ranges.iter().any(|attr| attr.db_obj_type == DBObjType::MasterProperty));
}

#[test]
fn db_obj_attribute_cache_maps_types_to_dbobj_types_and_masks() {
    assert_eq!(
        DBObjType::Palette,
        DBObjAttributeCache::db_obj_type_from_type::<Palette>()
    );
    assert_eq!(
        DBObjType::StringTable,
        DBObjAttributeCache::db_obj_type_from_type::<StringTable>()
    );
    assert_eq!(0x0400_0000, DBObjAttributeCache::mask_from_type::<Palette>());
    assert_eq!(0x0000_0000, DBObjAttributeCache::mask_from_type::<StringTable>());
}

#[test]
fn object_factory_creates_boxed_dbobjs_from_type_and_id() {
    let from_type = dat_reader_writer::Lib::IO::ObjectFactory::create_boxed(DBObjType::Iteration)
        .expect("factory should create Iteration");
    assert_eq!(DBObjType::Iteration, from_type.db_obj_type());
    assert_eq!(
        dat_reader_writer::Generated::Enums::DBObjHeaderFlags::DBObjHeaderFlags::None,
        from_type.header_flags()
    );
    assert!(from_type.as_any().is::<Iteration>());

    let mut from_portal_id = dat_reader_writer::Lib::IO::ObjectFactory::create_boxed_from_id(
        DatFileType::Portal,
        0x1500_0010,
    )
    .expect("factory should resolve RenderTexture");
    assert_eq!(DBObjType::RenderTexture, from_portal_id.db_obj_type());
    assert_eq!(
        dat_reader_writer::Generated::Enums::DBObjHeaderFlags::DBObjHeaderFlags::from_bits_retain(
            dat_reader_writer::Generated::Enums::DBObjHeaderFlags::DBObjHeaderFlags::HasId.bits()
                | dat_reader_writer::Generated::Enums::DBObjHeaderFlags::DBObjHeaderFlags::HasDataCategory.bits()
        ),
        from_portal_id.header_flags()
    );
    from_portal_id.set_data_category(7);
    assert_eq!(7, from_portal_id.data_category());
    assert!(from_portal_id.as_any().is::<RenderTexture>());

    let from_local_id = dat_reader_writer::Lib::IO::ObjectFactory::create_boxed_from_id(
        DatFileType::Local,
        0x2100_0010,
    )
    .expect("factory should resolve LayoutDesc");
    assert_eq!(DBObjType::LayoutDesc, from_local_id.db_obj_type());
    assert!(from_local_id.as_any().is::<LayoutDesc>());

    let from_cell_id = dat_reader_writer::Lib::IO::ObjectFactory::create_boxed_from_id(
        DatFileType::Cell,
        0x0001_FFFF,
    )
    .expect("factory should resolve LandBlock");
    assert_eq!(DBObjType::LandBlock, from_cell_id.db_obj_type());
    assert!(from_cell_id.as_any().is::<LandBlock>());
}

#[test]
fn dbobj_base_pack_unpack_matches_header_flag_behavior() {
    use dat_reader_writer::Generated::Enums::DBObjHeaderFlags::DBObjHeaderFlags;

    let flags = DBObjHeaderFlags::from_bits_retain(
        DBObjHeaderFlags::HasId.bits() | DBObjHeaderFlags::HasDataCategory.bits(),
    );
    let value = DBObjBase {
        id: 0x1234_5678,
        data_category: 0x90AB_CDEF,
        header_flags: flags,
    };

    let mut bytes = vec![0u8; 16];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(value.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = DBObjBase::with_header_flags(flags);
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(0x1234_5678, unpacked.id);
    assert_eq!(0x90AB_CDEF, unpacked.data_category);
    assert_eq!(flags, unpacked.header_flags);

    let none = DBObjBase::with_header_flags(DBObjHeaderFlags::None);
    let mut none_bytes = vec![0u8; 8];
    let mut none_writer = DatBinWriter::new(&mut none_bytes);
    assert!(none.pack(&mut none_writer));
    assert_eq!(0, none_writer.offset());
}

#[test]
fn foundational_enum_and_flag_surfaces_are_stable() {
    use dat_reader_writer::{
        Generated::Enums::{
            AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType,
            DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType,
            GfxObjFlags::GfxObjFlags, MotionCommand::MotionCommand, PartsMask::PartsMask,
            PixelFormat::PixelFormat, SurfaceType::SurfaceType, TextureType::TextureType,
        },
        Lib::IO::DatBTree::DatBTreeFileFlags::DatBTreeFileFlags,
    };

    assert_eq!(5, DBObjType::Palette as i32);
    assert_eq!(38, DBObjType::MasterProperty as i32);
    assert_eq!(0x01, DBObjHeaderFlags::HasId.bits());
    assert_eq!(0x02, DBObjHeaderFlags::HasDataCategory.bits());
    assert_eq!(
        DBObjHeaderFlags::HasId | DBObjHeaderFlags::HasDataCategory,
        DBObjHeaderFlags::from_bits_retain(0x03)
    );
    assert_eq!(TextureType::TEXTURE2D, TextureType::from(2));
    assert_eq!(PixelFormat::PFID_P8, PixelFormat::from(41));
    assert_eq!(MotionCommand(0x4100_0003), MotionCommand(0x4100_0003));
    assert_eq!(AnimationHookDir::BOTH, AnimationHookDir::from(0));
    assert_eq!(AnimationHookDir::FORWARD, AnimationHookDir::from(1));
    assert_eq!(AnimationHookType::NO_OP, AnimationHookType::from(0));
    assert_eq!(AnimationHookType::SOUND, AnimationHookType::from(1));
    assert!(GfxObjFlags::HasDrawing.bits() != 0);
    assert!(PartsMask::HasSceneInfo.bits() != 0);
    assert!(SurfaceType::Diffuse.bits() != 0);
    assert_eq!(0, DatBTreeFileFlags::None.bits());
}

#[test]
fn foundational_wire_types_roundtrip_cleanly() {
    use dat_reader_writer::{
        DBObjs::Palette::Palette,
        Generated::Enums::DBObjHeaderFlags::DBObjHeaderFlags,
        Lib::IO::DatBTree::{
            DatBTreeFile::DatBTreeFile, DatBTreeFileFlags::DatBTreeFileFlags,
        },
        Types::{ColorARGB::ColorARGB, PStringBase::PStringBase, QualifiedDataId::QualifiedDataId},
    };

    let file = DatBTreeFile {
        flags: DatBTreeFileFlags::IsCompressed,
        version: 7,
        id: 0x0400_1234,
        offset: 0x2000,
        size: 0x300,
        raw_date: 0x1122_3344,
        iteration: 9,
    };
    let mut file_bytes = vec![0u8; DatBTreeFile::SIZE];
    assert!(file.pack(&mut DatBinWriter::new(&mut file_bytes)));
    let mut unpacked_file = DatBTreeFile::default();
    assert!(unpacked_file.unpack(&mut DatBinReader::new(&file_bytes)));
    assert_eq!(file, unpacked_file);

    let color = ColorARGB {
        blue: 0x11,
        green: 0x22,
        red: 0x33,
        alpha: 0x44,
    };
    let mut color_bytes = [0u8; 4];
    assert!(color.pack(&mut DatBinWriter::new(&mut color_bytes)));
    let mut unpacked_color = ColorARGB::default();
    assert!(unpacked_color.unpack(&mut DatBinReader::new(&color_bytes)));
    assert_eq!(color, unpacked_color);

    let byte_string = PStringBase::<u8>::from("Portal");
    let wide_string = PStringBase::<u16>::from("Dereth");
    let mut byte_string_bytes = vec![0u8; 64];
    let mut wide_string_bytes = vec![0u8; 64];
    let byte_used = {
        let mut writer = DatBinWriter::new(&mut byte_string_bytes);
        assert!(byte_string.pack(&mut writer));
        writer.offset()
    };
    let wide_used = {
        let mut writer = DatBinWriter::new(&mut wide_string_bytes);
        assert!(wide_string.pack(&mut writer));
        writer.offset()
    };
    let mut unpacked_byte_string = PStringBase::<u8>::default();
    let mut unpacked_wide_string = PStringBase::<u16>::default();
    assert!(unpacked_byte_string.unpack(&mut DatBinReader::new(
        &byte_string_bytes[..byte_used]
    )));
    assert!(unpacked_wide_string.unpack(&mut DatBinReader::new(
        &wide_string_bytes[..wide_used]
    )));
    assert_eq!(byte_string, unpacked_byte_string);
    assert_eq!(wide_string, unpacked_wide_string);

    let qualified = QualifiedDataId::<Palette>::new(0x0400_4321);
    let mut qualified_bytes = [0u8; 4];
    assert!(qualified.pack(&mut DatBinWriter::new(&mut qualified_bytes)));
    let mut unpacked_qualified = QualifiedDataId::<Palette>::default();
    assert!(unpacked_qualified.unpack(&mut DatBinReader::new(&qualified_bytes)));
    assert_eq!(qualified.data_id, unpacked_qualified.data_id);

    let flags = DBObjHeaderFlags::HasId | DBObjHeaderFlags::HasDataCategory;
    assert_eq!(0x03, flags.bits());
}

#[test]
fn dat_database_can_read_typed_iteration() {
    let mut payload = [0u8; 12];
    let mut writer = DatBinWriter::new(&mut payload);
    writer.write_i32(1);
    writer.write_i32(-1);
    writer.write_i32(1);

    let bytes = build_single_block_dat(DatFileType::Portal, 0xFFFF0001, &payload);
    let path = unique_temp_file();
    fs::write(&path, bytes).unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    assert_eq!(DBObjType::Iteration, db.type_from_id(0xFFFF0001));

    let iteration = db.try_get::<Iteration>(0xFFFF0001).unwrap().unwrap();
    assert_eq!(1, iteration.current_iteration);
    assert_eq!(Some(&-1), iteration.iterations.get(&1));
}

#[test]
fn dat_database_async_reads_typed_entry_and_cache() {
    let palette = Palette {
        base: DBObjBase {
            id: 0x0400_00AA,
            ..Default::default()
        },
        colors: vec![ColorARGB {
            blue: 0x12,
            green: 0x34,
            red: 0x56,
            alpha: 0x78,
        }],
    };

    let mut payload = vec![0u8; 128];
    let used = {
        let mut writer = DatBinWriter::new(&mut payload);
        assert!(palette.pack(&mut writer));
        writer.offset()
    };

    let bytes = build_single_block_dat(DatFileType::Portal, 0x0400_00AA, &payload[..used]);
    let path = unique_temp_file();
    fs::write(&path, bytes).unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let async_palette = block_on(db.get_async::<Palette>(0x0400_00AA))
        .unwrap()
        .unwrap();
    assert_eq!(0x12, async_palette.colors[0].blue);
    assert_eq!(0x78, async_palette.colors[0].alpha);

    let cached_palette = block_on(db.get_cached_async::<Palette>(0x0400_00AA))
        .unwrap()
        .unwrap();
    assert_eq!(0x12, cached_palette.colors[0].blue);
    assert_eq!(0x78, cached_palette.colors[0].alpha);
}

#[test]
fn dat_database_can_enumerate_ids_of_type() {
    let mut payload = [0u8; 8];
    let mut writer = DatBinWriter::new(&mut payload);
    writer.write_u32(0x00000000);
    writer.write_i32(0);

    let bytes = build_single_block_dat(DatFileType::Portal, 0x04000010, &payload);
    let path = unique_temp_file();
    fs::write(&path, bytes).unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let ids = db.get_all_ids_of_type::<Palette>().unwrap();
    assert_eq!(vec![0x04000010], ids);
}

#[test]
fn dat_database_can_read_language_string_and_local_string_table() {
    let language_string = LanguageString {
        value: PStringBase::from("Sho names"),
        ..Default::default()
    };
    let mut language_payload = vec![0u8; 128];
    let mut writer = DatBinWriter::new(&mut language_payload);
    assert!(language_string.pack(&mut writer));
    let language_used = writer.offset();

    let mut strings = HashTable::<u32, StringTableString>::default();
    strings.insert(
        0x52BA517,
        StringTableString {
            data_id: QualifiedDataId::new(0),
            strings: vec![PStringBase::from("first"), PStringBase::from("second")],
            variables: vec![11, 22],
            is_var_name_table_worth_packing: true,
        },
    );
    let string_table = StringTable {
        language: 1,
        strings,
        ..Default::default()
    };
    let mut table_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut table_payload);
    assert!(string_table.pack(&mut writer));
    let table_used = writer.offset();

    let portal_path = unique_temp_file();
    let local_path = unique_temp_file();
    fs::write(
        &portal_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x31000010,
            &language_payload[..language_used],
        ),
    )
    .unwrap();
    fs::write(
        &local_path,
        build_single_block_dat(DatFileType::Local, 0x23000001, &table_payload[..table_used]),
    )
    .unwrap();

    let portal_db = DatDatabase::new(DatDatabaseOptions {
        file_path: portal_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let local_db = DatDatabase::new(DatDatabaseOptions {
        file_path: local_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_language = portal_db
        .try_get::<LanguageString>(0x31000010)
        .unwrap()
        .unwrap();
    assert_eq!("Sho names", read_language.value.value);

    let read_table = local_db
        .try_get::<StringTable>(0x23000001)
        .unwrap()
        .unwrap();
    assert_eq!(1, read_table.language);
    assert_eq!(2, read_table.strings.get(&0x52BA517).unwrap().strings.len());
    assert_eq!(
        "first",
        read_table.strings.get(&0x52BA517).unwrap().strings[0].value
    );
    assert!(
        read_table
            .strings
            .get(&0x52BA517)
            .unwrap()
            .is_var_name_table_worth_packing
    );
}

#[test]
fn db_obj_attribute_cache_resolves_new_gameplay_tables() {
    let vital = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E000003).unwrap();
    let skill = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E000004).unwrap();

    assert_eq!(DBObjType::VitalTable, vital.db_obj_type);
    assert_eq!(DBObjType::SkillTable, skill.db_obj_type);
}

#[test]
fn db_obj_attribute_cache_tracks_new_gameplay_table_ports() {
    let attrs = DBObjAttributeCache::all_ported_attributes();
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::VitalTable)
    );
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::SkillTable)
    );
}

#[test]
fn dat_database_can_read_typed_vital_table() {
    use dat_reader_writer::{
        DBObjs::VitalTable::VitalTable, Generated::Enums::AttributeId::AttributeId,
        Types::SkillFormula::SkillFormula,
    };

    let vital = VitalTable {
        health: SkillFormula {
            additive_bonus: 1,
            attribute1_multiplier: 2,
            attribute2_multiplier: 3,
            divisor: 4,
            attribute1: AttributeId::STRENGTH,
            attribute2: AttributeId::ENDURANCE,
        },
        stamina: SkillFormula::default(),
        mana: SkillFormula::default(),
        ..Default::default()
    };

    let mut payload = vec![0u8; 128];
    let mut writer = DatBinWriter::new(&mut payload);
    assert!(vital.pack(&mut writer));
    let used = writer.offset();

    let bytes = build_single_block_dat(DatFileType::Portal, 0x0E000003, &payload[..used]);
    let path = unique_temp_file();
    fs::write(&path, bytes).unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_vital = db.try_get::<VitalTable>(0x0E000003).unwrap().unwrap();
    assert_eq!(1, read_vital.health.additive_bonus);
    assert_eq!(AttributeId::STRENGTH, read_vital.health.attribute1);
    assert_eq!(4, read_vital.health.divisor);
}

#[test]
fn db_obj_attribute_cache_resolves_experience_table() {
    let experience = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E000018).unwrap();
    assert_eq!(DBObjType::ExperienceTable, experience.db_obj_type);
}

#[test]
fn dat_database_can_read_typed_experience_table() {
    use dat_reader_writer::DBObjs::ExperienceTable::ExperienceTable;

    let experience = ExperienceTable {
        attributes: vec![0, 10, 20],
        vitals: vec![0, 30],
        trained_skills: vec![0, 40],
        specialized_skills: vec![0, 50],
        levels: vec![0, 100, 200],
        skill_credits: vec![0, 6, 8],
        ..Default::default()
    };

    let mut payload = vec![0u8; 128];
    let mut writer = DatBinWriter::new(&mut payload);
    assert!(experience.pack(&mut writer));
    let used = writer.offset();

    let bytes = build_single_block_dat(DatFileType::Portal, 0x0E000018, &payload[..used]);
    let path = unique_temp_file();
    fs::write(&path, bytes).unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_experience = db.try_get::<ExperienceTable>(0x0E000018).unwrap().unwrap();
    assert_eq!(vec![0, 10, 20], read_experience.attributes);
    assert_eq!(vec![0, 100, 200], read_experience.levels);
    assert_eq!(vec![0, 6, 8], read_experience.skill_credits);
}

#[test]
fn dat_database_can_read_font_language_info_and_name_filter_table() {
    let font = Font {
        max_char_height: 12,
        max_char_width: 8,
        char_descs: vec![FontCharDesc {
            unicode: 'A' as u16,
            offset_x: 1,
            offset_y: 2,
            width: 7,
            height: 9,
            horizontal_offset_before: -1,
            horizontal_offset_after: 2,
            vertical_offset_before: 3,
        }],
        num_horizontal_border_pixels: 1,
        num_vertical_border_pixels: 2,
        baseline_offset: 3,
        foreground_surface_data_id: QualifiedDataId::new(0x08000011),
        background_surface_data_id: QualifiedDataId::new(0x08000012),
        ..Default::default()
    };

    let mut font_payload = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut font_payload);
    assert!(font.pack(&mut writer));
    let font_used = writer.offset();

    let language_info = LanguageInfo {
        version: 2,
        base_value: 10,
        num_decimal_digits: 3,
        leading_zero: true,
        grouping_size: 3,
        numerals: PStringBase::from("0123456789"),
        decimal_separator: PStringBase::from("."),
        grouping_separator: PStringBase::from(","),
        negative_number_format: PStringBase::from("-n"),
        is_zero_singular: true,
        is_one_singular: true,
        is_negative_one_singular: false,
        is_two_or_more_singular: false,
        is_negative_two_or_less_singular: false,
        treasure_prefix_letters: PStringBase::from("abc"),
        treasure_middle_letters: PStringBase::from("def"),
        treasure_suffix_letters: PStringBase::from("ghi"),
        male_player_letters: PStringBase::from("klm"),
        female_player_letters: PStringBase::from("nop"),
        ime_enabled_setting: 1,
        symbol_color: 2,
        symbol_color_text: 3,
        symbol_height: 4,
        symbol_translucence: 5,
        symbol_placement: 6,
        cand_color_base: 7,
        cand_color_border: 8,
        cand_color_text: 9,
        comp_color_input: 10,
        comp_color_target_conv: 11,
        comp_color_converted: 12,
        comp_color_target_not_conv: 13,
        comp_color_input_err: 14,
        comp_translucence: 15,
        comp_color_text: 16,
        other_ime: 17,
        word_wrap_on_space: 18,
        additional_settings: PStringBase::from("settings"),
        additional_flags: 19,
        ..Default::default()
    };

    let mut language_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut language_payload);
    assert!(language_info.pack(&mut writer));
    let language_used = writer.offset();

    let mut language_data = HashTable::<u32, NameFilterLanguageData>::default();
    language_data.insert(
        1,
        NameFilterLanguageData {
            maximum_same_characters_in_a_row: 2,
            maximum_vowels_in_a_row: 3,
            first_n_characters_must_have_a_vowel: 4,
            vowel_containing_substring_length: 5,
            extra_allowed_characters: PStringBase::from("'-"),
            compound_letter_groups: vec![PStringBase::from("th"), PStringBase::from("sh")],
        },
    );
    let name_filter = NameFilterTable {
        language_data,
        ..Default::default()
    };

    let mut name_filter_payload = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut name_filter_payload);
    assert!(name_filter.pack(&mut writer));
    let name_filter_used = writer.offset();

    let font_path = unique_temp_file();
    let language_path = unique_temp_file();
    let name_filter_path = unique_temp_file();

    fs::write(
        &font_path,
        build_single_block_dat(DatFileType::Portal, 0x40000010, &font_payload[..font_used]),
    )
    .unwrap();
    fs::write(
        &language_path,
        build_single_block_dat(
            DatFileType::Local,
            0x41000010,
            &language_payload[..language_used],
        ),
    )
    .unwrap();
    fs::write(
        &name_filter_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x0E000020,
            &name_filter_payload[..name_filter_used],
        ),
    )
    .unwrap();

    let font_db = DatDatabase::new(DatDatabaseOptions {
        file_path: font_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let language_db = DatDatabase::new(DatDatabaseOptions {
        file_path: language_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let name_filter_db = DatDatabase::new(DatDatabaseOptions {
        file_path: name_filter_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_font = font_db.try_get::<Font>(0x40000010).unwrap().unwrap();
    assert_eq!(12, read_font.max_char_height);
    assert_eq!('A' as u16, read_font.char_descs[0].unicode);
    assert_eq!(0x08000011, read_font.foreground_surface_data_id.data_id);

    let read_language = language_db
        .try_get::<LanguageInfo>(0x41000010)
        .unwrap()
        .unwrap();
    assert_eq!(10, read_language.base_value);
    assert_eq!("0123456789", read_language.numerals.value);
    assert_eq!("settings", read_language.additional_settings.value);

    let read_name_filter = name_filter_db
        .try_get::<NameFilterTable>(0x0E000020)
        .unwrap()
        .unwrap();
    assert_eq!(
        2,
        read_name_filter
            .language_data
            .get(&1)
            .unwrap()
            .compound_letter_groups
            .len()
    );
    assert_eq!(
        "th",
        read_name_filter
            .language_data
            .get(&1)
            .unwrap()
            .compound_letter_groups[0]
            .value
    );
}

#[test]
fn dat_database_can_read_enum_mapper_family() {
    let mut mapper_strings = AutoGrowHashTable::<u32, PStringBase<u8>>::default();
    mapper_strings.insert(1, PStringBase::from("one"));
    let enum_mapper = EnumMapper {
        base_enum_map: 0x10,
        id_to_string_map: mapper_strings,
        ..Default::default()
    };

    let mut mapper_payload = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut mapper_payload);
    assert!(enum_mapper.pack(&mut writer));
    let mapper_used = writer.offset();

    let client_name_map = std::collections::BTreeMap::from([(2, String::from("client"))]);
    let server_name_map = std::collections::BTreeMap::from([(3, String::from("server"))]);
    let client_id_map = std::collections::BTreeMap::from([(4, 0x05000010)]);
    let server_id_map = std::collections::BTreeMap::from([(5, 0x05000020)]);

    let enum_id_map = EnumIDMap {
        client_id_numbering_type: NumberingType::NORMAL,
        client_enum_to_id: client_id_map.clone(),
        client_name_numbering_type: NumberingType::NORMAL,
        client_enum_to_name: client_name_map.clone(),
        server_id_numbering_type: NumberingType::NORMAL,
        server_enum_to_id: server_id_map.clone(),
        server_name_numbering_type: NumberingType::NORMAL,
        server_enum_to_name: server_name_map.clone(),
        ..Default::default()
    };
    let dual_enum_id_map = DualEnumIDMap {
        client_id_numbering_type: NumberingType::NORMAL,
        client_enum_to_id: client_id_map,
        client_name_numbering_type: NumberingType::NORMAL,
        client_enum_to_name: client_name_map,
        server_id_numbering_type: NumberingType::NORMAL,
        server_enum_to_id: server_id_map,
        server_name_numbering_type: NumberingType::NORMAL,
        server_enum_to_name: server_name_map,
        ..Default::default()
    };
    let data_id_mapper = DataIdMapper {
        client_id_numbering_type: NumberingType::NORMAL,
        client_enum_to_id: std::collections::BTreeMap::from([(14, 0x05000030)]),
        client_name_numbering_type: NumberingType::NORMAL,
        client_enum_to_name: std::collections::BTreeMap::from([(15, String::from("did-client"))]),
        server_id_numbering_type: NumberingType::NORMAL,
        server_enum_to_id: std::collections::BTreeMap::from([(16, 0x05000040)]),
        server_name_numbering_type: NumberingType::NORMAL,
        server_enum_to_name: std::collections::BTreeMap::from([(17, String::from("did-server"))]),
        ..Default::default()
    };
    let dual_data_id_mapper = DualDataIdMapper {
        client_id_numbering_type: NumberingType::NORMAL,
        client_enum_to_id: std::collections::BTreeMap::from([(24, 0x05000050)]),
        client_name_numbering_type: NumberingType::NORMAL,
        client_enum_to_name: std::collections::BTreeMap::from([(
            25,
            String::from("dual-did-client"),
        )]),
        server_id_numbering_type: NumberingType::NORMAL,
        server_enum_to_id: std::collections::BTreeMap::from([(26, 0x05000060)]),
        server_name_numbering_type: NumberingType::NORMAL,
        server_enum_to_name: std::collections::BTreeMap::from([(
            27,
            String::from("dual-did-server"),
        )]),
        ..Default::default()
    };

    let mut enum_id_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut enum_id_payload);
    assert!(enum_id_map.pack(&mut writer));
    let enum_id_used = writer.offset();

    let mut dual_enum_id_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut dual_enum_id_payload);
    assert!(dual_enum_id_map.pack(&mut writer));
    let dual_enum_id_used = writer.offset();

    let mut data_id_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut data_id_payload);
    assert!(data_id_mapper.pack(&mut writer));
    let data_id_used = writer.offset();

    let mut dual_data_id_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut dual_data_id_payload);
    assert!(dual_data_id_mapper.pack(&mut writer));
    let dual_data_id_used = writer.offset();

    let mapper_path = unique_temp_file();
    let enum_id_path = unique_temp_file();
    let dual_enum_id_path = unique_temp_file();
    let data_id_path = unique_temp_file();
    let dual_data_id_path = unique_temp_file();

    fs::write(
        &mapper_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x22000010,
            &mapper_payload[..mapper_used],
        ),
    )
    .unwrap();
    fs::write(
        &enum_id_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x25000010,
            &enum_id_payload[..enum_id_used],
        ),
    )
    .unwrap();
    fs::write(
        &data_id_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x25000011,
            &data_id_payload[..data_id_used],
        ),
    )
    .unwrap();
    fs::write(
        &dual_enum_id_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x27000010,
            &dual_enum_id_payload[..dual_enum_id_used],
        ),
    )
    .unwrap();
    fs::write(
        &dual_data_id_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x27000011,
            &dual_data_id_payload[..dual_data_id_used],
        ),
    )
    .unwrap();

    let mapper_db = DatDatabase::new(DatDatabaseOptions {
        file_path: mapper_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let enum_id_db = DatDatabase::new(DatDatabaseOptions {
        file_path: enum_id_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let dual_enum_id_db = DatDatabase::new(DatDatabaseOptions {
        file_path: dual_enum_id_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let data_id_db = DatDatabase::new(DatDatabaseOptions {
        file_path: data_id_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let dual_data_id_db = DatDatabase::new(DatDatabaseOptions {
        file_path: dual_data_id_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_mapper = mapper_db
        .try_get::<EnumMapper>(0x22000010)
        .unwrap()
        .unwrap();
    assert_eq!(0x10, read_mapper.base_enum_map);
    assert_eq!("one", read_mapper.id_to_string_map.get(&1).unwrap().value);

    let read_enum_id = enum_id_db
        .try_get::<EnumIDMap>(0x25000010)
        .unwrap()
        .unwrap();
    assert_eq!(Some(&0x05000010), read_enum_id.client_enum_to_id.get(&4));
    assert_eq!("client", read_enum_id.client_enum_to_name.get(&2).unwrap());

    let read_dual_enum_id = dual_enum_id_db
        .try_get::<DualEnumIDMap>(0x27000010)
        .unwrap()
        .unwrap();
    assert_eq!(
        Some(&0x05000020),
        read_dual_enum_id.server_enum_to_id.get(&5)
    );
    assert_eq!(
        "server",
        read_dual_enum_id.server_enum_to_name.get(&3).unwrap()
    );

    let read_data_id = data_id_db
        .try_get::<DataIdMapper>(0x25000011)
        .unwrap()
        .unwrap();
    assert_eq!(Some(&0x05000030), read_data_id.client_enum_to_id.get(&14));
    assert_eq!(
        "did-client",
        read_data_id.client_enum_to_name.get(&15).unwrap()
    );
    assert_eq!(Some(&0x05000040), read_data_id.server_enum_to_id.get(&16));
    assert_eq!(
        "did-server",
        read_data_id.server_enum_to_name.get(&17).unwrap()
    );

    let read_dual_data_id = dual_data_id_db
        .try_get::<DualDataIdMapper>(0x27000011)
        .unwrap()
        .unwrap();
    assert_eq!(
        Some(&0x05000050),
        read_dual_data_id.client_enum_to_id.get(&24)
    );
    assert_eq!(
        "dual-did-server",
        read_dual_data_id.server_enum_to_name.get(&27).unwrap()
    );
}

#[test]
fn dat_database_can_read_render_material_family() {
    use dat_reader_writer::Generated::Enums::{RMDataType::RMDataType, TextureType::TextureType};

    let render_texture = RenderTexture {
        texture_type: TextureType::TEXTURE2D,
        source_levels: vec![
            QualifiedDataId::new(0x06000010),
            QualifiedDataId::new(0x06000011),
        ],
        ..Default::default()
    };

    let render_material = RenderMaterial::default();

    let material_modifier = MaterialModifier {
        material_properties: vec![MaterialProperty {
            name_id: 0x11223344,
            data_type: RMDataType::Texture,
            data_length: 16,
            data_length2: 32,
            data_length3: 48,
            data_length4: 64,
        }],
        ..Default::default()
    };

    let material_instance = MaterialInstance {
        material_id: 0x16000010,
        material_type: 7,
        modifier_refs: vec![0x17000010, 0x17000011],
        allow_stencil_shadows: true,
        want_discard_geometry: false,
        ..Default::default()
    };

    let mut render_texture_payload = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut render_texture_payload);
    assert!(render_texture.pack(&mut writer));
    let render_texture_used = writer.offset();

    let mut render_material_payload = vec![0u8; 64];
    let mut writer = DatBinWriter::new(&mut render_material_payload);
    assert!(render_material.pack(&mut writer));
    let render_material_used = writer.offset();

    let mut material_modifier_payload = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut material_modifier_payload);
    assert!(material_modifier.pack(&mut writer));
    let material_modifier_used = writer.offset();

    let mut material_instance_payload = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut material_instance_payload);
    assert!(material_instance.pack(&mut writer));
    let material_instance_used = writer.offset();

    let render_texture_path = unique_temp_file();
    let render_material_path = unique_temp_file();
    let material_modifier_path = unique_temp_file();
    let material_instance_path = unique_temp_file();

    fs::write(
        &render_texture_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x15000010,
            &render_texture_payload[..render_texture_used],
        ),
    )
    .unwrap();
    fs::write(
        &render_material_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x16000010,
            &render_material_payload[..render_material_used],
        ),
    )
    .unwrap();
    fs::write(
        &material_modifier_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x17000010,
            &material_modifier_payload[..material_modifier_used],
        ),
    )
    .unwrap();
    fs::write(
        &material_instance_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x18000010,
            &material_instance_payload[..material_instance_used],
        ),
    )
    .unwrap();

    let render_texture_db = DatDatabase::new(DatDatabaseOptions {
        file_path: render_texture_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let render_material_db = DatDatabase::new(DatDatabaseOptions {
        file_path: render_material_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let material_modifier_db = DatDatabase::new(DatDatabaseOptions {
        file_path: material_modifier_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let material_instance_db = DatDatabase::new(DatDatabaseOptions {
        file_path: material_instance_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_render_texture = render_texture_db
        .try_get::<RenderTexture>(0x15000010)
        .unwrap()
        .unwrap();
    assert_eq!(TextureType::TEXTURE2D, read_render_texture.texture_type);
    assert_eq!(2, read_render_texture.source_levels.len());
    assert_eq!(0x06000011, read_render_texture.source_levels[1].data_id);

    let read_render_material = render_material_db
        .try_get::<RenderMaterial>(0x16000010)
        .unwrap()
        .unwrap();
    assert_eq!(0, read_render_material.base.data_category);

    let read_material_modifier = material_modifier_db
        .try_get::<MaterialModifier>(0x17000010)
        .unwrap()
        .unwrap();
    assert_eq!(1, read_material_modifier.material_properties.len());
    assert_eq!(
        0x11223344,
        read_material_modifier.material_properties[0].name_id
    );

    let read_material_instance = material_instance_db
        .try_get::<MaterialInstance>(0x18000010)
        .unwrap()
        .unwrap();
    assert_eq!(0x16000010, read_material_instance.material_id);
    assert_eq!(
        vec![0x17000010, 0x17000011],
        read_material_instance.modifier_refs
    );
    assert!(read_material_instance.allow_stencil_shadows);
    assert!(!read_material_instance.want_discard_geometry);
}

#[test]
fn dat_database_can_read_action_map_and_master_property() {
    use dat_reader_writer::Generated::Enums::{
        BasePropertyType::BasePropertyType, PatchFlags::PatchFlags,
        PropertyCachingType::PropertyCachingType, PropertyDatFileType::PropertyDatFileType,
        PropertyGroupName::PropertyGroupName, PropertyInheritanceType::PropertyInheritanceType,
        PropertyPropagationType::PropertyPropagationType,
    };

    let mut child_map = std::collections::BTreeMap::new();
    child_map.insert(
        7,
        ActionMapValue {
            magic: 0,
            unknown: 0,
            toggle_type: ToggleType::Toggle,
            dummy_list_length: 0,
            user_binding: UserBindingData {
                action_class: 10,
                action_name: 20,
                action_description: 30,
            },
        },
    );

    let mut input_maps = std::collections::BTreeMap::new();
    input_maps.insert(5, child_map);

    let mut conflicting_maps = std::collections::BTreeMap::new();
    conflicting_maps.insert(
        9,
        InputsConflictsValue {
            input_map: 9,
            conflicting_input_maps: vec![5, 6],
        },
    );

    let action_map = ActionMap {
        input_maps,
        string_table_id: 0x23000001,
        conflicting_maps,
        ..Default::default()
    };

    let mut enum_mapper_strings = AutoGrowHashTable::<u32, PStringBase<u8>>::default();
    enum_mapper_strings.insert(1, PStringBase::from("Visible"));

    let mut properties = std::collections::BTreeMap::new();
    properties.insert(
        0x10,
        BasePropertyDesc {
            name: 0x10,
            property_type: BasePropertyType::Integer,
            group: PropertyGroupName::from(1),
            provider: 2,
            data: 3,
            patch_flags: PatchFlags::default(),
            default_value: Some(BaseProperty::Integer {
                header: Default::default(),
                value: 42,
            }),
            max_value: Some(BaseProperty::Integer {
                header: Default::default(),
                value: 99,
            }),
            min_value: Some(BaseProperty::Integer {
                header: Default::default(),
                value: -5,
            }),
            prediction_timeout: 1.5,
            inheritance_type: PropertyInheritanceType::from(1),
            dat_file_type: PropertyDatFileType::from(1),
            propagation_type: PropertyPropagationType::from(1),
            caching_type: PropertyCachingType::from(1),
            required: true,
            read_only: false,
            no_checkpoint: false,
            recorded: true,
            do_not_replay: false,
            absolute_time_stamp: false,
            groupable: true,
            propagate_to_children: true,
            available_properties: std::collections::BTreeMap::from([(1, 2)]),
        },
    );

    let master_property = MasterProperty {
        enum_mapper: EnumMapperData {
            base_enum_map: 11,
            unknown: 22,
            id_to_string_map: enum_mapper_strings,
        },
        properties,
        ..Default::default()
    };

    let mut action_payload = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut action_payload);
    assert!(action_map.pack(&mut writer));
    let action_used = writer.offset();

    let mut master_payload = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut master_payload);
    assert!(master_property.pack(&mut writer));
    let master_used = writer.offset();

    let action_path = unique_temp_file();
    let master_path = unique_temp_file();

    fs::write(
        &action_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x26000010,
            &action_payload[..action_used],
        ),
    )
    .unwrap();
    fs::write(
        &master_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x39000010,
            &master_payload[..master_used],
        ),
    )
    .unwrap();

    let action_db = DatDatabase::new(DatDatabaseOptions {
        file_path: action_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let master_db = DatDatabase::new(DatDatabaseOptions {
        file_path: master_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_action_map = action_db.try_get::<ActionMap>(0x26000010).unwrap().unwrap();
    assert_eq!(0x23000001, read_action_map.string_table_id);
    let read_action = read_action_map.input_maps.get(&5).unwrap().get(&7).unwrap();
    assert_eq!(ToggleType::Toggle, read_action.toggle_type);
    assert_eq!(20, read_action.user_binding.action_name);
    let read_conflict = read_action_map.conflicting_maps.get(&9).unwrap();
    assert_eq!(vec![5, 6], read_conflict.conflicting_input_maps);

    let read_master = master_db
        .try_get::<MasterProperty>(0x39000010)
        .unwrap()
        .unwrap();
    assert_eq!(11, read_master.enum_mapper.base_enum_map);
    assert_eq!(
        "Visible",
        read_master
            .enum_mapper
            .id_to_string_map
            .get(&1)
            .unwrap()
            .value
    );
    let read_property = read_master.properties.get(&0x10).unwrap();
    assert_eq!(BasePropertyType::Integer, read_property.property_type);
    assert_eq!(1.5, read_property.prediction_timeout);
    match read_property.default_value.as_ref().unwrap() {
        BaseProperty::Integer { value, .. } => assert_eq!(42, *value),
        other => panic!("unexpected default property variant: {other:?}"),
    }
}

#[test]
fn dat_database_can_write_master_property_and_read_it_back() {
    use dat_reader_writer::Generated::Enums::{
        BasePropertyType::BasePropertyType, PatchFlags::PatchFlags,
        PropertyCachingType::PropertyCachingType, PropertyDatFileType::PropertyDatFileType,
        PropertyGroupName::PropertyGroupName, PropertyInheritanceType::PropertyInheritanceType,
        PropertyPropagationType::PropertyPropagationType,
    };

    let path = unique_temp_file();
    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        access_type: DatAccessType::ReadWrite,
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    db.block_allocator
        .init_new(DatFileType::Portal, 0, 1024, 4)
        .unwrap();

    let mut mapper_strings = AutoGrowHashTable::<u32, PStringBase<u8>>::default();
    mapper_strings.insert(1, PStringBase::from("test"));

    let master_property = MasterProperty {
        base: DBObjBase {
            id: 0x3900_0001,
            ..Default::default()
        },
        enum_mapper: EnumMapperData {
            base_enum_map: 0x1234_5678,
            unknown: 0x1234_5678,
            id_to_string_map: mapper_strings,
        },
        properties: std::collections::BTreeMap::from([(
            1,
            BasePropertyDesc {
                name: 1,
                property_type: BasePropertyType::Integer,
                group: PropertyGroupName::from(2),
                provider: 3,
                data: 0x1234_5678,
                patch_flags: PatchFlags::default(),
                default_value: Some(BaseProperty::Integer {
                    header: BasePropertyHeader::default(),
                    value: 77,
                }),
                max_value: Some(BaseProperty::Integer {
                    header: BasePropertyHeader::default(),
                    value: 99,
                }),
                min_value: Some(BaseProperty::Integer {
                    header: BasePropertyHeader::default(),
                    value: -5,
                }),
                prediction_timeout: 1.5,
                inheritance_type: PropertyInheritanceType::from(1),
                dat_file_type: PropertyDatFileType::from(1),
                propagation_type: PropertyPropagationType::from(1),
                caching_type: PropertyCachingType::from(1),
                ..Default::default()
            },
        )]),
    };

    assert!(db.try_write_file(&master_property).unwrap());
    let read_master = db.try_get::<MasterProperty>(0x3900_0001).unwrap().unwrap();
    assert_eq!(0x3900_0001, read_master.base.id);
    assert_eq!(0x1234_5678, read_master.enum_mapper.base_enum_map);
    assert_eq!(
        "test",
        read_master
            .enum_mapper
            .id_to_string_map
            .get(&1)
            .unwrap()
            .value
    );
    assert_eq!(1, read_master.properties.len());
    assert_eq!(0x1234_5678, read_master.properties.get(&1).unwrap().data);

    drop(db);

    let reopened = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let read_reopened = reopened.try_get::<MasterProperty>(0x3900_0001).unwrap().unwrap();
    assert_eq!(BasePropertyType::Integer, read_reopened.properties.get(&1).unwrap().property_type);
    match read_reopened
        .properties
        .get(&1)
        .unwrap()
        .default_value
        .as_ref()
        .unwrap()
    {
        BaseProperty::Integer { value, .. } => assert_eq!(77, *value),
        other => panic!("unexpected reopened default property variant: {other:?}"),
    }
}

#[test]
fn dat_database_can_write_and_read_compressed_file_bytes() {
    let path = unique_temp_file();
    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        access_type: DatAccessType::ReadWrite,
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    db.block_allocator
        .init_new(DatFileType::Portal, 0, 1024, 4)
        .unwrap();

    let payload = b"PortalPortalPortalPortalPortalPortalPortalPortalPortalPortal".repeat(32);
    assert!(db
        .try_write_compressed_bytes(0x0400_0010, &payload, payload.len(), 2)
        .unwrap());

    let entry = db.try_get_file_entry(0x0400_0010).unwrap().unwrap();
    assert!(entry.flags.contains(
        dat_reader_writer::Lib::IO::DatBTree::DatBTreeFileFlags::DatBTreeFileFlags::IsCompressed
    ));
    assert_eq!(2, entry.iteration);

    let compressed = db.try_get_file_bytes(0x0400_0010, false).unwrap().unwrap();
    assert_eq!(payload.len() as u32, u32::from_le_bytes(compressed[0..4].try_into().unwrap()));

    let decompressed = db.try_get_file_bytes(0x0400_0010, true).unwrap().unwrap();
    assert_eq!(payload, decompressed);
}

#[test]
fn dat_database_can_write_with_template_metadata() {
    use dat_reader_writer::Lib::IO::DatBTree::{
        DatBTreeFile::DatBTreeFile, DatBTreeFileFlags::DatBTreeFileFlags,
    };

    let path = unique_temp_file();
    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        access_type: DatAccessType::ReadWrite,
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    db.block_allocator
        .init_new(DatFileType::Portal, 0, 1024, 4)
        .unwrap();

    let palette = Palette {
        base: DBObjBase {
            id: 0x0400_0042,
            ..Default::default()
        },
        colors: vec![ColorARGB {
            blue: 1,
            green: 2,
            red: 3,
            alpha: 4,
        }],
    };

    let template = DatBTreeFile {
        flags: DatBTreeFileFlags::None,
        version: 7,
        iteration: 9,
        ..Default::default()
    };

    assert!(db
        .try_write_file_with_template(&palette, template)
        .unwrap());

    let entry = db.try_get_file_entry(0x0400_0042).unwrap().unwrap();
    assert_eq!(7, entry.version);
    assert_eq!(9, entry.iteration);
    assert!(!entry.flags.contains(DatBTreeFileFlags::IsCompressed));

    let compressed_payload = b"TextureTextureTextureTextureTextureTexture".repeat(16);
    let compressed_template = DatBTreeFile {
        flags: DatBTreeFileFlags::None,
        version: 5,
        iteration: 11,
        ..Default::default()
    };

    assert!(db
        .try_write_compressed_bytes_with_template(
            0x0500_0042,
            &compressed_payload,
            compressed_payload.len(),
            compressed_template,
        )
        .unwrap());

    let compressed_entry = db.try_get_file_entry(0x0500_0042).unwrap().unwrap();
    assert_eq!(5, compressed_entry.version);
    assert_eq!(11, compressed_entry.iteration);
    assert!(compressed_entry
        .flags
        .contains(DatBTreeFileFlags::IsCompressed));

    let read_bytes = db.try_get_file_bytes(0x0500_0042, true).unwrap().unwrap();
    assert_eq!(compressed_payload, read_bytes);
}

#[test]
fn dat_database_async_file_byte_paths_match_sync_behavior() {
    let path = unique_temp_file();
    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        access_type: DatAccessType::ReadWrite,
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    db.block_allocator
        .init_new(DatFileType::Portal, 0, 1024, 4)
        .unwrap();

    let payload = b"AsyncPortalPayloadAsyncPortalPayload".repeat(24);
    assert!(block_on(db.try_write_compressed_bytes_with_template_async(
        0x0500_0099,
        &payload,
        payload.len(),
        dat_reader_writer::Lib::IO::DatBTree::DatBTreeFile::DatBTreeFile {
            version: 3,
            iteration: 4,
            ..Default::default()
        },
    ))
    .unwrap());

    let entry = db.try_get_file_entry(0x0500_0099).unwrap().unwrap();
    assert_eq!(3, entry.version);
    assert_eq!(4, entry.iteration);

    let sync_bytes = db.try_get_file_bytes(0x0500_0099, true).unwrap().unwrap();
    let async_bytes = block_on(db.try_get_file_bytes_async(0x0500_0099, true))
        .unwrap()
        .unwrap();
    assert_eq!(sync_bytes, async_bytes);
    assert_eq!(payload, async_bytes);
}

#[test]
fn dat_collection_can_read_db_properties_and_layout_desc() {
    let mut master_property = MasterProperty::default();
    master_property.properties.insert(
        0x10,
        BasePropertyDesc {
            property_type:
                dat_reader_writer::Generated::Enums::BasePropertyType::BasePropertyType::Integer,
            ..Default::default()
        },
    );
    master_property.properties.insert(
        0x11,
        BasePropertyDesc {
            property_type:
                dat_reader_writer::Generated::Enums::BasePropertyType::BasePropertyType::Struct,
            ..Default::default()
        },
    );
    master_property.properties.insert(
        0x12,
        BasePropertyDesc {
            property_type:
                dat_reader_writer::Generated::Enums::BasePropertyType::BasePropertyType::Array,
            ..Default::default()
        },
    );

    let db_properties = DBProperties {
        properties: std::collections::BTreeMap::from([
            (
                1,
                BaseProperty::Integer {
                    header: BasePropertyHeader {
                        master_property_id: 0x10,
                        should_pack_master_property_id: true,
                    },
                    value: 42,
                },
            ),
            (
                2,
                BaseProperty::Struct {
                    header: BasePropertyHeader {
                        master_property_id: 0x11,
                        should_pack_master_property_id: true,
                    },
                    value: std::collections::BTreeMap::from([(
                        7,
                        BaseProperty::Integer {
                            header: BasePropertyHeader {
                                master_property_id: 0x10,
                                should_pack_master_property_id: true,
                            },
                            value: 99,
                        },
                    )]),
                },
            ),
        ]),
        ..Default::default()
    };

    let state_desc = StateDesc {
        state_id: 1,
        pass_to_children: true,
        incorporation_flags: IncorporationFlags::X
            | IncorporationFlags::Y
            | IncorporationFlags::Width
            | IncorporationFlags::Height
            | IncorporationFlags::ZLevel,
        properties: std::collections::BTreeMap::from([(
            3,
            BaseProperty::Array {
                header: BasePropertyHeader {
                    master_property_id: 0x12,
                    should_pack_master_property_id: true,
                },
                value: vec![
                    BaseProperty::Integer {
                        header: BasePropertyHeader {
                            master_property_id: 0x10,
                            should_pack_master_property_id: true,
                        },
                        value: 5,
                    },
                    BaseProperty::Integer {
                        header: BasePropertyHeader {
                            master_property_id: 0x10,
                            should_pack_master_property_id: true,
                        },
                        value: 6,
                    },
                ],
            },
        )]),
        ..Default::default()
    };

    let mut elements = HashTable::<u32, ElementDesc>::default();
    elements.insert(
        9,
        ElementDesc {
            state_desc: state_desc.clone(),
            read_order: 2,
            element_id: 9,
            element_type: 3,
            base_element: 0,
            base_layout_id: 0,
            default_state: UIStateId::NORMAL,
            x: 10,
            y: 20,
            width: 300,
            height: 120,
            z_level: 7,
            left_edge: 1,
            top_edge: 2,
            right_edge: 3,
            bottom_edge: 4,
            states: std::collections::BTreeMap::from([(UIStateId::NORMAL, state_desc)]),
            ..Default::default()
        },
    );
    let layout_desc = LayoutDesc {
        width: 640,
        height: 480,
        elements,
        ..Default::default()
    };

    let mut master_payload = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut master_payload);
    assert!(master_property.pack(&mut writer));
    let master_used = writer.offset();

    let mut db_properties_payload = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut db_properties_payload);
    assert!(db_properties.pack(&mut writer));
    let db_properties_used = writer.offset();

    let mut layout_payload = vec![0u8; 4096];
    let mut writer = DatBinWriter::new(&mut layout_payload);
    assert!(layout_desc.pack(&mut writer));
    let layout_used = writer.offset();

    let dir = unique_temp_dir();
    fs::write(
        dir.join("client_portal.dat"),
        build_multi_block_dat(
            DatFileType::Portal,
            &[
                (0x39000001, master_payload[..master_used].to_vec()),
                (
                    0x78000010,
                    db_properties_payload[..db_properties_used].to_vec(),
                ),
            ],
        ),
    )
    .unwrap();
    fs::write(
        dir.join("client_local_English.dat"),
        build_multi_block_dat(
            DatFileType::Local,
            &[(0x21000010, layout_payload[..layout_used].to_vec())],
        ),
    )
    .unwrap();
    fs::write(
        dir.join("client_cell_1.dat"),
        build_multi_block_dat(DatFileType::Cell, &[]),
    )
    .unwrap();
    fs::write(
        dir.join("client_highres.dat"),
        build_multi_block_dat(DatFileType::Portal, &[]),
    )
    .unwrap();

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::Read)
            .unwrap();

    let read_db_properties = collection
        .try_get::<DBProperties>(0x78000010)
        .unwrap()
        .unwrap();
    match read_db_properties.properties.get(&1).unwrap() {
        BaseProperty::Integer { value, .. } => assert_eq!(42, *value),
        other => panic!("unexpected DBProperties property: {other:?}"),
    }
    match read_db_properties.properties.get(&2).unwrap() {
        BaseProperty::Struct { value, .. } => match value.get(&7).unwrap() {
            BaseProperty::Integer { value, .. } => assert_eq!(99, *value),
            other => panic!("unexpected nested DBProperties property: {other:?}"),
        },
        other => panic!("unexpected DBProperties struct property: {other:?}"),
    }

    let read_layout = collection
        .try_get::<LayoutDesc>(0x21000010)
        .unwrap()
        .unwrap();
    assert_eq!(640, read_layout.width);
    assert_eq!(480, read_layout.height);
    let read_element = read_layout.elements.get(&9).unwrap();
    assert_eq!(10, read_element.x);
    assert_eq!(20, read_element.y);
    assert_eq!(300, read_element.width);
    assert_eq!(120, read_element.height);
    match read_element.state_desc.properties.get(&3).unwrap() {
        BaseProperty::Array { value, .. } => {
            assert_eq!(2, value.len());
            match &value[0] {
                BaseProperty::Integer { value, .. } => assert_eq!(5, *value),
                other => panic!("unexpected layout array value: {other:?}"),
            }
            match &value[1] {
                BaseProperty::Integer { value, .. } => assert_eq!(6, *value),
                other => panic!("unexpected layout array value: {other:?}"),
            }
        }
        other => panic!("unexpected layout property: {other:?}"),
    }
}

#[test]
fn dat_database_can_read_cell_environment_types() {
    use dat_reader_writer::Generated::Enums::BSPNodeType::BSPNodeType;
    use dat_reader_writer::Lib::IO::Numerics::{Plane, Quaternion, Vector3};

    let frame = Frame {
        origin: Vector3::new(1.0, 2.0, 3.0),
        orientation: Quaternion::new(0.0, 0.0, 0.0, 1.0),
    };

    let cell_struct = CellStruct {
        vertex_array: Default::default(),
        polygons: std::collections::BTreeMap::new(),
        portals: vec![3, 4],
        cell_bsp: CellBSPTree {
            root: CellBSPNode {
                node_type: BSPNodeType::LEAF,
                splitting_plane: Plane::default(),
                pos_node: None,
                neg_node: None,
                leaf_index: 7,
            },
        },
        physics_polygons: std::collections::BTreeMap::new(),
        physics_bsp: Default::default(),
        drawing_bsp: None,
    };

    let environment = Environment {
        cells: std::collections::BTreeMap::from([(0x0001_0001, cell_struct.clone())]),
        ..Default::default()
    };

    let land_block_info = LandBlockInfo {
        num_cells: 2,
        objects: vec![Stab {
            id: 0x01000010,
            frame: frame.clone(),
        }],
        buildings: vec![BuildingInfo {
            model_id: 0x02000010,
            frame: frame.clone(),
            num_leaves: 3,
            portals: vec![BuildingPortal {
                flags: PortalFlags::ExactMatch,
                other_cell_id: 5,
                other_portal_id: 6,
                stab_list: vec![1, 2],
            }],
        }],
        restriction_table: std::collections::BTreeMap::from([(1, 2)]),
        ..Default::default()
    };

    let mut terrain = [TerrainInfo::default(); 81];
    terrain[0].set_road(2);
    terrain[0].set_terrain_type(
        dat_reader_writer::Generated::Enums::TerrainTextureType::TerrainTextureType::LUSH_GRASS,
    );
    terrain[0].set_scenery(7);
    terrain[1].set_terrain_type(
        dat_reader_writer::Generated::Enums::TerrainTextureType::TerrainTextureType::SAND_YELLOW,
    );
    let mut height = [0u8; 81];
    height[0] = 9;
    height[80] = 27;
    let land_block = LandBlock {
        has_objects: true,
        terrain,
        height,
        ..Default::default()
    };

    let env_cell = EnvCell {
        base: dat_reader_writer::Types::DBObj::DBObjBase {
            id: 0x0001_0123,
            ..Default::default()
        },
        flags: EnvCellFlags::HasStaticObjs | EnvCellFlags::HasRestrictionObj,
        surfaces: vec![10, 11],
        environment_id: 0x0001,
        cell_structure: 0x0002,
        position: frame.clone(),
        cell_portals: vec![CellPortal {
            flags: PortalFlags::PortalSide,
            polygon_id: 12,
            other_cell_id: 13,
            other_portal_id: 14,
        }],
        visible_cells: vec![21, 22],
        static_objects: vec![Stab {
            id: 0x03000010,
            frame,
        }],
        restriction_obj: 0x04000010,
    };

    let mut environment_payload = vec![0u8; 4096];
    let mut writer = DatBinWriter::new(&mut environment_payload);
    assert!(environment.pack(&mut writer));
    let environment_used = writer.offset();

    let mut landblock_payload = vec![0u8; 4096];
    let mut writer = DatBinWriter::new(&mut landblock_payload);
    assert!(land_block_info.pack(&mut writer));
    let landblock_used = writer.offset();

    let mut envcell_payload = vec![0u8; 4096];
    let mut writer = DatBinWriter::new(&mut envcell_payload);
    assert!(env_cell.pack(&mut writer));
    let envcell_used = writer.offset();

    let mut land_payload = vec![0u8; 4096];
    let mut writer = DatBinWriter::new(&mut land_payload);
    assert!(land_block.pack(&mut writer));
    let land_used = writer.offset();

    let environment_path = unique_temp_file();
    let landblock_path = unique_temp_file();
    let land_path = unique_temp_file();
    let envcell_path = unique_temp_file();

    fs::write(
        &environment_path,
        build_single_block_dat(
            DatFileType::Cell,
            0x0D000123,
            &environment_payload[..environment_used],
        ),
    )
    .unwrap();
    fs::write(
        &landblock_path,
        build_single_block_dat(
            DatFileType::Cell,
            0x0001FFFE,
            &landblock_payload[..landblock_used],
        ),
    )
    .unwrap();
    fs::write(
        &land_path,
        build_single_block_dat(DatFileType::Cell, 0x0001FFFF, &land_payload[..land_used]),
    )
    .unwrap();
    fs::write(
        &envcell_path,
        build_single_block_dat(
            DatFileType::Cell,
            0x00010123,
            &envcell_payload[..envcell_used],
        ),
    )
    .unwrap();

    let environment_db = DatDatabase::new(DatDatabaseOptions {
        file_path: environment_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let landblock_db = DatDatabase::new(DatDatabaseOptions {
        file_path: landblock_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let land_db = DatDatabase::new(DatDatabaseOptions {
        file_path: land_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let envcell_db = DatDatabase::new(DatDatabaseOptions {
        file_path: envcell_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_environment = environment_db
        .try_get::<Environment>(0x0D000123)
        .unwrap()
        .unwrap();
    assert_eq!(
        vec![0x0D000123],
        environment_db.get_all_ids_of_type::<Environment>().unwrap()
    );
    assert_eq!(1, read_environment.cells.len());
    assert_eq!(
        vec![3, 4],
        read_environment.cells.get(&0x00010001).unwrap().portals
    );
    assert_eq!(
        7,
        read_environment
            .cells
            .get(&0x00010001)
            .unwrap()
            .cell_bsp
            .root
            .leaf_index
    );

    let read_landblock = landblock_db
        .try_get::<LandBlockInfo>(0x0001FFFE)
        .unwrap()
        .unwrap();
    assert_eq!(
        vec![0x0001FFFE],
        landblock_db.get_all_ids_of_type::<LandBlockInfo>().unwrap()
    );
    assert_eq!(2, read_landblock.num_cells);
    assert_eq!(1, read_landblock.objects.len());
    assert_eq!(1, read_landblock.buildings.len());
    assert_eq!(2, *read_landblock.restriction_table.get(&1).unwrap());

    let read_land = land_db.try_get::<LandBlock>(0x0001FFFF).unwrap().unwrap();
    assert_eq!(
        vec![0x0001FFFF],
        land_db.get_all_ids_of_type::<LandBlock>().unwrap()
    );
    assert!(read_land.has_objects);
    assert_eq!(2, read_land.terrain[0].road());
    assert_eq!(7, read_land.terrain[0].scenery());
    assert_eq!(9, read_land.height[0]);
    assert_eq!(27, read_land.height[80]);

    let read_envcell = envcell_db.try_get::<EnvCell>(0x00010123).unwrap().unwrap();
    assert_eq!(
        vec![0x00010123],
        envcell_db.get_all_ids_of_type::<EnvCell>().unwrap()
    );
    assert_eq!(vec![10, 11], read_envcell.surfaces);
    assert_eq!(1, read_envcell.cell_portals.len());
    assert_eq!(vec![21, 22], read_envcell.visible_cells);
    assert_eq!(1, read_envcell.static_objects.len());
    assert_eq!(0x04000010, read_envcell.restriction_obj);
}

#[test]
fn dat_database_can_read_degrade_quality_and_spell_component_tables() {
    use dat_reader_writer::Generated::Enums::ComponentType::ComponentType;
    use dat_reader_writer::Types::GfxObjInfo::GfxObjInfo;

    let gfx_obj_degrade_info = GfxObjDegradeInfo {
        degrades: vec![GfxObjInfo {
            id: QualifiedDataId::new(0x01000010),
            degrade_mode: 2,
            min_dist: 1.5,
            ideal_dist: 3.5,
            max_dist: 6.5,
        }],
        ..Default::default()
    };

    let quality_filter = QualityFilter {
        int_stat_filter: vec![1, 2],
        int64_stat_filter: vec![3],
        bool_stat_filter: vec![4],
        float_stat_filter: vec![5],
        data_id_stat_filter: vec![6],
        instance_id_stat_filter: vec![7],
        string_stat_filter: vec![8],
        position_stat_filter: vec![9],
        attribute_stat_filter: vec![10],
        attribute2nd_stat_filter: vec![11],
        skill_stat_filter: vec![12],
        ..Default::default()
    };

    let mut components = dat_reader_writer::Types::PackableHashTable::PackableHashTable::<
        u32,
        SpellComponentBase,
    >::default();
    components.insert(
        0x100,
        SpellComponentBase {
            name: ObfuscatedPStringBase::from("Scarab"),
            category: 3,
            icon: QualifiedDataId::new(0x06000010),
            component_type: ComponentType::Scarab,
            gesture: 4,
            time: 1.25,
            text: ObfuscatedPStringBase::from("Spell text"),
            cdm: 2.5,
        },
    );
    let spell_component_table = SpellComponentTable {
        components,
        ..Default::default()
    };

    let mut degrade_payload = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut degrade_payload);
    assert!(gfx_obj_degrade_info.pack(&mut writer));
    let degrade_used = writer.offset();

    let mut quality_payload = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut quality_payload);
    assert!(quality_filter.pack(&mut writer));
    let quality_used = writer.offset();

    let mut spell_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut spell_payload);
    assert!(spell_component_table.pack(&mut writer));
    let spell_used = writer.offset();

    let degrade_path = unique_temp_file();
    let quality_path = unique_temp_file();
    let spell_path = unique_temp_file();

    fs::write(
        &degrade_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x11000010,
            &degrade_payload[..degrade_used],
        ),
    )
    .unwrap();
    fs::write(
        &quality_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x0E010010,
            &quality_payload[..quality_used],
        ),
    )
    .unwrap();
    fs::write(
        &spell_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x0E00000F,
            &spell_payload[..spell_used],
        ),
    )
    .unwrap();

    let degrade_db = DatDatabase::new(DatDatabaseOptions {
        file_path: degrade_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let quality_db = DatDatabase::new(DatDatabaseOptions {
        file_path: quality_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let spell_db = DatDatabase::new(DatDatabaseOptions {
        file_path: spell_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_degrade = degrade_db
        .try_get::<GfxObjDegradeInfo>(0x11000010)
        .unwrap()
        .unwrap();
    assert_eq!(1, read_degrade.degrades.len());
    assert_eq!(0x01000010, read_degrade.degrades[0].id.data_id);

    let read_quality = quality_db
        .try_get::<QualityFilter>(0x0E010010)
        .unwrap()
        .unwrap();
    assert_eq!(vec![1, 2], read_quality.int_stat_filter);
    assert_eq!(vec![12], read_quality.skill_stat_filter);

    let read_spell = spell_db
        .try_get::<SpellComponentTable>(0x0E00000F)
        .unwrap()
        .unwrap();
    let component = read_spell.components.get(&0x100).unwrap();
    assert_eq!("Scarab", component.name.value);
    assert_eq!(ComponentType::Scarab, component.component_type);
    assert_eq!(0x06000010, component.icon.data_id);
}

#[test]
fn dat_database_can_read_spell_table() {
    let spell = SpellBase {
        name: ObfuscatedPStringBase::from("Test"),
        description: ObfuscatedPStringBase::from("This is a description"),
        components: vec![1, 2, 3, 4, 5, 6, 7, 8],
        school: MagicSchool::ItemEnchantment,
        icon: 0x12345678,
        category: SpellCategory::StrengthRaising,
        bitfield: SpellIndex::Resistable,
        base_mana: 103,
        base_range_constant: 1.1,
        base_range_mod: 0.5,
        power: 500,
        spell_economy_mod: 0.3,
        formula_version: 1,
        component_loss: 0.2,
        meta_spell_type: SpellType::Enchantment,
        meta_spell_id: 0x87654321,
        duration: 2.2,
        degrade_modifier: 0.5,
        degrade_limit: 1.0,
        portal_lifetime: 0.0,
        caster_effect: PlayScript::LAUNCH,
        target_effect: PlayScript::EXPLODE,
        fizzle_effect: PlayScript::FIZZLE,
        recovery_interval: 0.1,
        recovery_amount: 1.0,
        display_order: 123,
        non_component_target_type: ItemType::Armor,
        mana_mod: 5,
    };

    let mut spells =
        dat_reader_writer::Types::PackableHashTable::PackableHashTable::<u32, SpellBase>::default();
    spells.insert(1, spell);

    let mut spell_set_tiers = PHashTable::<u32, SpellSetTiers>::default();
    spell_set_tiers.insert(
        0,
        SpellSetTiers {
            spells: vec![1, 2, 3],
        },
    );
    spell_set_tiers.insert(
        1,
        SpellSetTiers {
            spells: vec![4, 5, 6],
        },
    );

    let mut spell_sets = PHashTable::<EquipmentSet, SpellSet>::default();
    spell_sets.insert(EquipmentSet::Ninja, SpellSet { spell_set_tiers });

    let spell_table = SpellTable {
        spells,
        spell_sets,
        ..Default::default()
    };

    let mut payload = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut payload);
    assert!(spell_table.pack(&mut writer));
    let used = writer.offset();

    let path = unique_temp_file();
    fs::write(
        &path,
        build_single_block_dat(DatFileType::Portal, 0x0E00000E, &payload[..used]),
    )
    .unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_spell_table = db.try_get::<SpellTable>(0x0E00000E).unwrap().unwrap();
    let read_spell = read_spell_table.spells.get(&1).unwrap();
    assert_eq!("Test", read_spell.name.value);
    assert_eq!("This is a description", read_spell.description.value);
    assert_eq!(MagicSchool::ItemEnchantment, read_spell.school);
    assert_eq!(SpellCategory::StrengthRaising, read_spell.category);
    assert_eq!(SpellIndex::Resistable, read_spell.bitfield);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], read_spell.components);
    assert_eq!(ItemType::Armor, read_spell.non_component_target_type);
    assert_eq!(PlayScript::LAUNCH, read_spell.caster_effect);
    let read_set = read_spell_table
        .spell_sets
        .get(&EquipmentSet::Ninja)
        .unwrap();
    assert_eq!(
        vec![1, 2, 3],
        read_set.spell_set_tiers.get(&0).unwrap().spells
    );
    assert_eq!(
        vec![4, 5, 6],
        read_set.spell_set_tiers.get(&1).unwrap().spells
    );
}

#[test]
fn dat_database_can_read_bad_contract_and_taboo_tables() {
    let mut bad_ids = dat_reader_writer::Types::PackableHashTable::PackableHashTable::<
        QualifiedDataId<BadDataTable>,
        u32,
    >::default();
    bad_ids.insert(QualifiedDataId::new(0x0E00001A), 77);
    let bad_data_table = BadDataTable {
        bad_ids,
        ..Default::default()
    };

    let mut contracts =
        dat_reader_writer::Types::PackableHashTable::PackableHashTable::<u32, Contract>::default();
    contracts.insert(
        1,
        Contract {
            version: 2,
            contract_id: 10,
            contract_name: AC1LegacyPStringBase::from("Contract"),
            description: AC1LegacyPStringBase::from("Description"),
            description_progress: AC1LegacyPStringBase::from("Progress"),
            name_npc_start: AC1LegacyPStringBase::from("StartNPC"),
            name_npc_end: AC1LegacyPStringBase::from("EndNPC"),
            questflag_stamped: AC1LegacyPStringBase::from("Stamped"),
            questflag_started: AC1LegacyPStringBase::from("Started"),
            questflag_finished: AC1LegacyPStringBase::from("Finished"),
            questflag_progress: AC1LegacyPStringBase::from("QuestProgress"),
            questflag_timer: AC1LegacyPStringBase::from("Timer"),
            questflag_repeat_time: AC1LegacyPStringBase::from("Repeat"),
            location_npc_start: Position {
                cell_id: 1,
                frame: Frame {
                    origin: dat_reader_writer::Lib::IO::Numerics::Vector3::new(1.0, 2.0, 3.0),
                    orientation: dat_reader_writer::Lib::IO::Numerics::Quaternion::new(
                        0.0, 0.0, 0.0, 1.0,
                    ),
                },
            },
            location_npc_end: Position::default(),
            location_quest_area: Position::default(),
        },
    );
    let contract_table = ContractTable {
        contracts,
        ..Default::default()
    };

    let mut taboo_entries = HashTable::<u32, TabooTableEntry>::default();
    taboo_entries.insert(
        5,
        TabooTableEntry {
            key: 5,
            unknown2: 2,
            banned_patterns: vec![PStringBase::from("badword"), PStringBase::from("worseword")],
        },
    );
    let taboo_table = TabooTable {
        audience_to_banned_patterns: taboo_entries,
        ..Default::default()
    };

    let mut bad_payload = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut bad_payload);
    assert!(bad_data_table.pack(&mut writer));
    let bad_used = writer.offset();

    let mut contract_payload = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut contract_payload);
    assert!(contract_table.pack(&mut writer));
    let contract_used = writer.offset();

    let mut taboo_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut taboo_payload);
    assert!(taboo_table.pack(&mut writer));
    let taboo_used = writer.offset();

    let bad_path = unique_temp_file();
    let contract_path = unique_temp_file();
    let taboo_path = unique_temp_file();

    fs::write(
        &bad_path,
        build_single_block_dat(DatFileType::Portal, 0x0E00001A, &bad_payload[..bad_used]),
    )
    .unwrap();
    fs::write(
        &contract_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x0E00001D,
            &contract_payload[..contract_used],
        ),
    )
    .unwrap();
    fs::write(
        &taboo_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x0E00001E,
            &taboo_payload[..taboo_used],
        ),
    )
    .unwrap();

    let bad_db = DatDatabase::new(DatDatabaseOptions {
        file_path: bad_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let contract_db = DatDatabase::new(DatDatabaseOptions {
        file_path: contract_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let taboo_db = DatDatabase::new(DatDatabaseOptions {
        file_path: taboo_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_bad = bad_db.try_get::<BadDataTable>(0x0E00001A).unwrap().unwrap();
    assert_eq!(
        Some(&77),
        read_bad.bad_ids.get(&QualifiedDataId::new(0x0E00001A))
    );

    let read_contract = contract_db
        .try_get::<ContractTable>(0x0E00001D)
        .unwrap()
        .unwrap();
    let contract = read_contract.contracts.get(&1).unwrap();
    assert_eq!("Contract", contract.contract_name.value);
    assert_eq!(10, contract.contract_id);

    let read_taboo = taboo_db.try_get::<TabooTable>(0x0E00001E).unwrap().unwrap();
    let entry = read_taboo.audience_to_banned_patterns.get(&5).unwrap();
    assert_eq!(2, entry.banned_patterns.len());
    assert_eq!("badword", entry.banned_patterns[0].value);
}

#[test]
fn dat_database_can_read_chat_pose_table() {
    let mut chat_poses = dat_reader_writer::Types::PackableHashTable::PackableHashTable::<
        AC1LegacyPStringBase<u8>,
        AC1LegacyPStringBase<u8>,
    >::default();
    chat_poses.insert(
        AC1LegacyPStringBase::from("wave"),
        AC1LegacyPStringBase::from("anim_wave"),
    );

    let mut chat_emotes = dat_reader_writer::Types::PackableHashTable::PackableHashTable::<
        AC1LegacyPStringBase<u8>,
        ChatEmoteData,
    >::default();
    chat_emotes.insert(
        AC1LegacyPStringBase::from("anim_wave"),
        ChatEmoteData {
            my_emote: AC1LegacyPStringBase::from("You wave."),
            other_emote: AC1LegacyPStringBase::from("%s waves."),
        },
    );

    let chat_pose_table = ChatPoseTable {
        chat_poses,
        chat_emotes,
        ..Default::default()
    };

    let mut payload = vec![0u8; 1024];
    let mut writer = DatBinWriter::new(&mut payload);
    assert!(chat_pose_table.pack(&mut writer));
    let used = writer.offset();

    let path = unique_temp_file();
    fs::write(
        &path,
        build_single_block_dat(DatFileType::Portal, 0x0E000007, &payload[..used]),
    )
    .unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_chat_pose = db.try_get::<ChatPoseTable>(0x0E000007).unwrap().unwrap();
    assert_eq!(
        "anim_wave",
        read_chat_pose
            .chat_poses
            .get(&AC1LegacyPStringBase::from("wave"))
            .unwrap()
            .value
    );
    let emote = read_chat_pose
        .chat_emotes
        .get(&AC1LegacyPStringBase::from("anim_wave"))
        .unwrap();
    assert_eq!("You wave.", emote.my_emote.value);
    assert_eq!("%s waves.", emote.other_emote.value);
}

#[test]
fn dat_database_can_read_object_hierarchy() {
    let object_hierarchy = ObjectHierarchy {
        root_node: ObjHierarchyNode {
            menu_name: ObfuscatedPStringBase::from("Root"),
            wcid: 100,
            children: vec![ObjHierarchyNode {
                menu_name: ObfuscatedPStringBase::from("Child"),
                wcid: 200,
                children: vec![],
            }],
        },
        ..Default::default()
    };

    let mut payload = vec![0u8; 1024];
    let mut writer = DatBinWriter::new(&mut payload);
    assert!(object_hierarchy.pack(&mut writer));
    let used = writer.offset();

    let path = unique_temp_file();
    fs::write(
        &path,
        build_single_block_dat(DatFileType::Portal, 0x0E00000D, &payload[..used]),
    )
    .unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_hierarchy = db.try_get::<ObjectHierarchy>(0x0E00000D).unwrap().unwrap();
    assert_eq!("Root", read_hierarchy.root_node.menu_name.value);
    assert_eq!(100, read_hierarchy.root_node.wcid);
    assert_eq!(1, read_hierarchy.root_node.children.len());
    assert_eq!(
        "Child",
        read_hierarchy.root_node.children[0].menu_name.value
    );
    assert_eq!(200, read_hierarchy.root_node.children[0].wcid);
}

#[test]
fn dat_database_can_read_master_input_map() {
    use dat_reader_writer::Generated::Enums::DeviceType::DeviceType;

    let mut input_maps = std::collections::BTreeMap::new();
    input_maps.insert(
        1,
        CInputMap {
            mappings: vec![QualifiedControl {
                key: dat_reader_writer::Types::ControlSpecification::ControlSpecification {
                    key: 0x41,
                    modifier: 0x02,
                },
                activation: 3,
                unknown: 4,
            }],
        },
    );

    let master_input_map = MasterInputMap {
        name: PStringBase::from("default"),
        guid_map: Uuid::from_u128(0x11223344556677889900AABBCCDDEEFF),
        devices: vec![DeviceKeyMapEntry {
            device_type: DeviceType::KEYBOARD,
            guid: Uuid::from_u128(0x0102030405060708090A0B0C0D0E0F10),
        }],
        meta_keys: vec![
            dat_reader_writer::Types::ControlSpecification::ControlSpecification {
                key: 0x11,
                modifier: 0x22,
            },
        ],
        input_maps,
        ..Default::default()
    };

    let mut payload = vec![0u8; 1024];
    let mut writer = DatBinWriter::new(&mut payload);
    assert!(master_input_map.pack(&mut writer));
    let used = writer.offset();

    let path = unique_temp_file();
    fs::write(
        &path,
        build_single_block_dat(DatFileType::Portal, 0x14000010, &payload[..used]),
    )
    .unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_input_map = db.try_get::<MasterInputMap>(0x14000010).unwrap().unwrap();
    assert_eq!("default", read_input_map.name.value);
    assert_eq!(1, read_input_map.devices.len());
    assert_eq!(DeviceType::KEYBOARD, read_input_map.devices[0].device_type);
    assert_eq!(1, read_input_map.input_maps.len());
    let mapping = &read_input_map.input_maps.get(&1).unwrap().mappings[0];
    assert_eq!(0x41, mapping.key.key);
    assert_eq!(3, mapping.activation);
}

#[test]
fn dat_database_can_read_palette_set_clothing_and_particle_emitter_info() {
    use dat_reader_writer::Generated::Enums::{
        EmitterType::EmitterType, ParticleType::ParticleType,
    };

    let palette_set = PaletteSet {
        palettes: vec![0x04000010, 0x04000011],
        ..Default::default()
    };

    let clothing = Clothing {
        clothing_base_effects: std::collections::BTreeMap::from([(
            0x01000010,
            dat_reader_writer::Types::ClothingBaseEffect::ClothingBaseEffect {
                clo_object_effects: vec![
                    dat_reader_writer::Types::CloObjectEffect::CloObjectEffect {
                        index: 1,
                        model_id: QualifiedDataId::new(0x01000020),
                        clo_texture_effects: vec![],
                    },
                ],
            },
        )]),
        clothing_sub_pal_effects: std::collections::BTreeMap::from([(
            5,
            dat_reader_writer::Types::CloSubPalEffect::CloSubPalEffect {
                icon: QualifiedDataId::new(0x0D000002),
                clo_sub_palettes: vec![dat_reader_writer::Types::CloSubPalette::CloSubPalette {
                    palette_set: QualifiedDataId::new(0x0F000010),
                    ranges: vec![
                        dat_reader_writer::Types::CloSubPaletteRange::CloSubPaletteRange {
                            offset: 3,
                            num_colors: 4,
                        },
                    ],
                }],
            },
        )]),
        ..Default::default()
    };

    let emitter_info = ParticleEmitterInfo {
        unknown: 7,
        emitter_type: EmitterType::BirthratePerSec,
        particle_type: ParticleType::Still,
        gfx_obj_id: QualifiedDataId::new(0x01000030),
        hw_gfx_obj_id: QualifiedDataId::new(0x01000031),
        birthrate: 0.25,
        max_particles: 10,
        initial_particles: 2,
        total_particles: 20,
        total_seconds: 4.5,
        lifespan: 1.25,
        lifespan_rand: 0.5,
        offset_dir: dat_reader_writer::Lib::IO::Numerics::Vector3::new(1.0, 2.0, 3.0),
        min_offset: 0.1,
        max_offset: 0.2,
        a: dat_reader_writer::Lib::IO::Numerics::Vector3::new(4.0, 5.0, 6.0),
        min_a: 0.3,
        max_a: 0.4,
        b: dat_reader_writer::Lib::IO::Numerics::Vector3::new(7.0, 8.0, 9.0),
        min_b: 0.5,
        max_b: 0.6,
        c: dat_reader_writer::Lib::IO::Numerics::Vector3::new(10.0, 11.0, 12.0),
        min_c: 0.7,
        max_c: 0.8,
        start_scale: 1.1,
        final_scale: 1.2,
        scale_rand: 0.9,
        start_trans: 0.95,
        final_trans: 0.15,
        trans_rand: 0.05,
        is_parent_local: true,
        ..Default::default()
    };

    let mut palette_payload = vec![0u8; 128];
    let mut writer = DatBinWriter::new(&mut palette_payload);
    assert!(palette_set.pack(&mut writer));
    let palette_used = writer.offset();

    let mut clothing_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut clothing_payload);
    assert!(clothing.pack(&mut writer));
    let clothing_used = writer.offset();

    let mut emitter_payload = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut emitter_payload);
    assert!(emitter_info.pack(&mut writer));
    let emitter_used = writer.offset();

    let palette_path = unique_temp_file();
    let clothing_path = unique_temp_file();
    let emitter_path = unique_temp_file();

    fs::write(
        &palette_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x0F000010,
            &palette_payload[..palette_used],
        ),
    )
    .unwrap();
    fs::write(
        &clothing_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x10000010,
            &clothing_payload[..clothing_used],
        ),
    )
    .unwrap();
    fs::write(
        &emitter_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x32000010,
            &emitter_payload[..emitter_used],
        ),
    )
    .unwrap();

    let palette_db = DatDatabase::new(DatDatabaseOptions {
        file_path: palette_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let clothing_db = DatDatabase::new(DatDatabaseOptions {
        file_path: clothing_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();
    let emitter_db = DatDatabase::new(DatDatabaseOptions {
        file_path: emitter_path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_palette_set = palette_db
        .try_get::<PaletteSet>(0x0F000010)
        .unwrap()
        .unwrap();
    assert_eq!(vec![0x04000010, 0x04000011], read_palette_set.palettes);

    let read_clothing = clothing_db
        .try_get::<Clothing>(0x10000010)
        .unwrap()
        .unwrap();
    assert_eq!(1, read_clothing.clothing_base_effects.len());
    assert_eq!(1, read_clothing.clothing_sub_pal_effects.len());
    assert_eq!(
        0x01000020,
        read_clothing
            .clothing_base_effects
            .get(&0x01000010)
            .unwrap()
            .clo_object_effects[0]
            .model_id
            .data_id
    );
    assert_eq!(
        0x0F000010,
        read_clothing
            .clothing_sub_pal_effects
            .get(&5)
            .unwrap()
            .clo_sub_palettes[0]
            .palette_set
            .data_id
    );

    let read_emitter = emitter_db
        .try_get::<ParticleEmitterInfo>(0x32000010)
        .unwrap()
        .unwrap();
    assert_eq!(7, read_emitter.unknown);
    assert_eq!(EmitterType::BirthratePerSec, read_emitter.emitter_type);
    assert_eq!(ParticleType::Still, read_emitter.particle_type);
    assert_eq!(0x01000030, read_emitter.gfx_obj_id.data_id);
    assert!(read_emitter.is_parent_local);
}

#[test]
fn pal_set_roundtrip_reads_palette_references() {
    use dat_reader_writer::DBObjs::PalSet::PalSet;

    let pal_set = PalSet {
        base: DBObjBase {
            id: 0x0F00_0010,
            ..Default::default()
        },
        palettes: vec![
            QualifiedDataId::new(0x0400_0010),
            QualifiedDataId::new(0x0400_0011),
            QualifiedDataId::new(0x0400_0012),
        ],
    };

    let mut payload = vec![0u8; 128];
    let mut writer = DatBinWriter::new(&mut payload);
    assert!(pal_set.pack(&mut writer));
    let used = writer.offset();

    let path = unique_temp_file();
    fs::write(
        &path,
        build_single_block_dat(DatFileType::Portal, 0x0F00_0010, &payload[..used]),
    )
    .unwrap();

    let db = DatDatabase::new(DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        ..DatDatabaseOptions::default()
    })
    .unwrap();

    let read_pal_set = db.try_get::<PalSet>(0x0F00_0010).unwrap().unwrap();
    assert_eq!(0x0F00_0010, read_pal_set.base.id);
    assert_eq!(3, read_pal_set.palettes.len());
    assert_eq!(0x0400_0011, read_pal_set.palettes[1].data_id);
}

#[test]
fn hash_table_roundtrip_reads_string_and_primitive_entries() {
    let mut strings = HashTable::<String, u32>::default();
    strings.bucket_size_index = 2;
    strings.insert("alpha".to_string(), 1);
    strings.insert("beta".to_string(), 2);

    let mut flags = HashTable::<u32, bool>::default();
    flags.insert(7, true);
    flags.insert(9, false);

    let mut wide = HashTable::<u64, i64>::default();
    wide.bucket_size_index = 3;
    wide.insert(0x0000_0001_0000_0002, -5);
    wide.insert(0x0000_0003_0000_0004, 7);

    let mut string_bytes = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut string_bytes);
    assert!(strings.pack(&mut writer));
    let string_used = writer.offset();

    let mut flag_bytes = vec![0u8; 128];
    let mut writer = DatBinWriter::new(&mut flag_bytes);
    assert!(flags.pack(&mut writer));
    let flag_used = writer.offset();

    let mut wide_bytes = vec![0u8; 128];
    let mut writer = DatBinWriter::new(&mut wide_bytes);
    assert!(wide.pack(&mut writer));
    let wide_used = writer.offset();

    let mut unpacked_strings = HashTable::<String, u32>::default();
    assert!(unpacked_strings.unpack(&mut DatBinReader::new(&string_bytes[..string_used])));
    assert_eq!(Some(&1), unpacked_strings.get(&"alpha".to_string()));
    assert_eq!(Some(&2), unpacked_strings.get(&"beta".to_string()));

    let mut unpacked_flags = HashTable::<u32, bool>::default();
    assert!(unpacked_flags.unpack(&mut DatBinReader::new(&flag_bytes[..flag_used])));
    assert_eq!(Some(&true), unpacked_flags.get(&7));
    assert_eq!(Some(&false), unpacked_flags.get(&9));

    let mut unpacked_wide = HashTable::<u64, i64>::default();
    assert!(unpacked_wide.unpack(&mut DatBinReader::new(&wide_bytes[..wide_used])));
    assert_eq!(Some(&-5), unpacked_wide.get(&0x0000_0001_0000_0002));
    assert_eq!(Some(&7), unpacked_wide.get(&0x0000_0003_0000_0004));
}

#[test]
fn hash_table_helpers_match_reference_bucket_selection_rules() {
    use dat_reader_writer::Lib::HashTableHelpers::{BUCKET_SIZES, get_bucket_size, get_bucket_size_index};

    assert_eq!(11, get_bucket_size(1, false));
    assert_eq!(23, get_bucket_size(12, false));
    assert_eq!(11, get_bucket_size(10, true));
    assert_eq!(23, get_bucket_size(22, true));

    assert_eq!(0, get_bucket_size_index(1, false));
    assert_eq!(1, get_bucket_size_index(12, false));
    assert_eq!(0, get_bucket_size_index(10, true));
    assert_eq!(1, get_bucket_size_index(22, true));
    assert_eq!(50331599, *BUCKET_SIZES.last().unwrap());
}

#[test]
fn hash_table_string_keys_pack_in_reference_hash_bucket_order() {
    use dat_reader_writer::Lib::HashTableHelpers::{BUCKET_SIZES, HashKeyable};

    let mut table = HashTable::<String, u32>::default();
    table.bucket_size_index = 1;
    table.insert("portal".to_string(), 1);
    table.insert("alpha".to_string(), 2);
    table.insert("zeta".to_string(), 3);

    let mut bytes = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(table.pack(&mut writer));
    let used = writer.offset();

    let mut reader = DatBinReader::new(&bytes[..used]);
    let bucket_size_index = reader.read_byte();
    let count = reader.read_compressed_uint() as usize;
    assert_eq!(1, bucket_size_index);
    assert_eq!(3, count);

    let mut packed_keys = Vec::new();
    for _ in 0..count {
        packed_keys.push(reader.read_string16_l());
        let _ = reader.read_u32();
    }

    let bucket_size = BUCKET_SIZES[table.bucket_size_index as usize] as u64;
    let mut expected_keys: Vec<_> = table.entries.keys().cloned().collect();
    expected_keys.sort_by_key(|key| key.hash_key() % bucket_size);
    assert_eq!(expected_keys, packed_keys);
}

#[test]
fn explicit_base_property_wrappers_roundtrip_current_scalar_variants() {
    let header = BasePropertyHeader {
        master_property_id: 0x1234_5678,
        should_pack_master_property_id: true,
    };

    let bool_property = BoolBaseProperty {
        header: header.clone(),
        value: true,
    };
    let integer_property = IntegerBaseProperty {
        header: header.clone(),
        value: -42,
    };
    let float_property = FloatBaseProperty {
        header: header.clone(),
        value: 3.25,
    };
    let vector_property = VectorBaseProperty {
        header: header.clone(),
        value: dat_reader_writer::Lib::IO::Numerics::Vector3::new(1.0, 2.0, 3.0),
    };
    let color_property = ColorBaseProperty {
        header: header.clone(),
        value: ColorARGB {
            blue: 4,
            green: 3,
            red: 2,
            alpha: 1,
        },
    };
    let enum_property = EnumBaseProperty {
        header: header.clone(),
        value: 0xAA55_AA55,
    };
    let data_id_property = DataIdBaseProperty {
        header: header.clone(),
        value: 0x0400_0010,
    };
    let instance_id_property = InstanceIdBaseProperty {
        header: header.clone(),
        value: 0x5000_0010,
    };
    let bitfield32_property = Bitfield32BaseProperty {
        header: header.clone(),
        value: 0xF0F0_0F0F,
    };
    let bitfield64_property = Bitfield64BaseProperty {
        header,
        value: 0x1122_3344_5566_7788,
    };

    let mut bool_bytes = vec![0u8; 16];
    let mut integer_bytes = vec![0u8; 16];
    let mut float_bytes = vec![0u8; 16];
    let mut vector_bytes = vec![0u8; 32];
    let mut color_bytes = vec![0u8; 16];
    let mut enum_bytes = vec![0u8; 16];
    let mut data_id_bytes = vec![0u8; 16];
    let mut instance_id_bytes = vec![0u8; 16];
    let mut bitfield32_bytes = vec![0u8; 16];
    let mut bitfield64_bytes = vec![0u8; 16];

    assert!(bool_property.pack(&mut DatBinWriter::new(&mut bool_bytes)));
    assert!(integer_property.pack(&mut DatBinWriter::new(&mut integer_bytes)));
    assert!(float_property.pack(&mut DatBinWriter::new(&mut float_bytes)));
    assert!(vector_property.pack(&mut DatBinWriter::new(&mut vector_bytes)));
    assert!(color_property.pack(&mut DatBinWriter::new(&mut color_bytes)));
    assert!(enum_property.pack(&mut DatBinWriter::new(&mut enum_bytes)));
    assert!(data_id_property.pack(&mut DatBinWriter::new(&mut data_id_bytes)));
    assert!(instance_id_property.pack(&mut DatBinWriter::new(&mut instance_id_bytes)));
    assert!(bitfield32_property.pack(&mut DatBinWriter::new(&mut bitfield32_bytes)));
    assert!(bitfield64_property.pack(&mut DatBinWriter::new(&mut bitfield64_bytes)));

    let mut unpacked_bool = BoolBaseProperty::default();
    let mut unpacked_integer = IntegerBaseProperty::default();
    let mut unpacked_float = FloatBaseProperty::default();
    let mut unpacked_vector = VectorBaseProperty::default();
    let mut unpacked_color = ColorBaseProperty::default();
    let mut unpacked_enum = EnumBaseProperty::default();
    let mut unpacked_data_id = DataIdBaseProperty::default();
    let mut unpacked_instance_id = InstanceIdBaseProperty::default();
    let mut unpacked_bitfield32 = Bitfield32BaseProperty::default();
    let mut unpacked_bitfield64 = Bitfield64BaseProperty::default();

    assert!(unpacked_bool.unpack(&mut DatBinReader::new(&bool_bytes[4..5])));
    assert!(unpacked_integer.unpack(&mut DatBinReader::new(&integer_bytes[4..8])));
    assert!(unpacked_float.unpack(&mut DatBinReader::new(&float_bytes[4..8])));
    assert!(unpacked_vector.unpack(&mut DatBinReader::new(&vector_bytes[4..16])));
    assert!(unpacked_color.unpack(&mut DatBinReader::new(&color_bytes[4..8])));
    assert!(unpacked_enum.unpack(&mut DatBinReader::new(&enum_bytes[4..8])));
    assert!(unpacked_data_id.unpack(&mut DatBinReader::new(&data_id_bytes[4..8])));
    assert!(unpacked_instance_id.unpack(&mut DatBinReader::new(&instance_id_bytes[4..8])));
    assert!(unpacked_bitfield32.unpack(&mut DatBinReader::new(&bitfield32_bytes[4..8])));
    assert!(unpacked_bitfield64.unpack(&mut DatBinReader::new(&bitfield64_bytes[4..12])));

    assert_eq!(true, unpacked_bool.value);
    assert_eq!(-42, unpacked_integer.value);
    assert_eq!(3.25, unpacked_float.value);
    assert_eq!(
        dat_reader_writer::Lib::IO::Numerics::Vector3::new(1.0, 2.0, 3.0),
        unpacked_vector.value
    );
    assert_eq!(
        ColorARGB {
            blue: 4,
            green: 3,
            red: 2,
            alpha: 1
        },
        unpacked_color.value
    );
    assert_eq!(0xAA55_AA55, unpacked_enum.value);
    assert_eq!(0x0400_0010, unpacked_data_id.value);
    assert_eq!(0x5000_0010, unpacked_instance_id.value);
    assert_eq!(0xF0F0_0F0F, unpacked_bitfield32.value);
    assert_eq!(0x1122_3344_5566_7788, unpacked_bitfield64.value);

    assert!(matches!(
        bool_property.as_base_property(),
        BaseProperty::Bool { value: true, .. }
    ));
    assert!(matches!(
        integer_property.as_base_property(),
        BaseProperty::Integer { value: -42, .. }
    ));
    assert!(matches!(
        enum_property.as_base_property(),
        BaseProperty::Enum {
            value: 0xAA55_AA55,
            ..
        }
    ));
}

#[test]
fn generated_misc_type_surfaces_roundtrip_cleanly() {
    let waveform = Waveform { raw_data: 0x7B };
    let portal_poly = PortalPoly {
        portal_index: 12,
        polygon_id: 34,
    };
    let material_layer = MaterialLayer {
        options: 1,
        true_flags: 2,
        false_flags: 3,
        render_pass:
            dat_reader_writer::Generated::Enums::RenderPassType::RenderPassType::AlphaBlend,
    };
    let layer_stage = LayerStage {
        sampler_name: PStringBase::from("Diffuse"),
        texture: 0x0500_0010,
        special_texture: 9,
        address_mode_u: 1,
        address_mode_v: 2,
        min_filter_mode: 3,
        mag_filter_mode: 4,
        mip_filter_mode: 5,
        ff_color_op: 6,
        ff_color_arg1: 7,
        ff_color_arg2: 8,
        ff_alpha_op: 9,
        ff_alpha_arg1: 10,
        ff_alpha_arg2: 11,
        ff_tex_coord_index: 12,
        ff_use_projection: 13,
    };
    let layer_modifier = LayerModifier;
    let uv_translate = LM_UVTranslate {
        type_discriminator: 1,
        offset_u: 1.5,
        offset_v: 2.5,
    };
    let uv_rotate = LM_UVRotate {
        type_discriminator: 2,
        center_u: 0.25,
        center_v: 0.75,
        angle: 3.5,
    };
    let uv_scale = LM_UVScale {
        type_discriminator: 3,
        scale_u: 4.5,
        scale_v: 5.5,
    };
    let uv_transform = LM_UVTransform {
        type_discriminator: 4,
    };

    let mut waveform_bytes = vec![0u8; 64];
    let mut waveform_writer = DatBinWriter::new(&mut waveform_bytes);
    assert!(waveform.pack(&mut waveform_writer));

    let mut portal_poly_bytes = vec![0u8; 8];
    let mut portal_poly_writer = DatBinWriter::new(&mut portal_poly_bytes);
    assert!(portal_poly.pack(&mut portal_poly_writer));

    let mut material_layer_bytes = vec![0u8; 32];
    let mut material_layer_writer = DatBinWriter::new(&mut material_layer_bytes);
    assert!(material_layer.pack(&mut material_layer_writer));

    let mut layer_stage_bytes = vec![0u8; 128];
    let mut layer_stage_writer = DatBinWriter::new(&mut layer_stage_bytes);
    assert!(layer_stage.pack(&mut layer_stage_writer));
    let layer_stage_used = layer_stage_writer.offset();

    let mut layer_modifier_bytes = vec![0u8; 1];
    let mut layer_modifier_writer = DatBinWriter::new(&mut layer_modifier_bytes);
    assert!(layer_modifier.pack(&mut layer_modifier_writer));

    let mut uv_translate_bytes = vec![0u8; 16];
    let mut uv_translate_writer = DatBinWriter::new(&mut uv_translate_bytes);
    assert!(uv_translate.pack(&mut uv_translate_writer));

    let mut uv_rotate_bytes = vec![0u8; 16];
    let mut uv_rotate_writer = DatBinWriter::new(&mut uv_rotate_bytes);
    assert!(uv_rotate.pack(&mut uv_rotate_writer));

    let mut uv_scale_bytes = vec![0u8; 16];
    let mut uv_scale_writer = DatBinWriter::new(&mut uv_scale_bytes);
    assert!(uv_scale.pack(&mut uv_scale_writer));

    let mut uv_transform_bytes = vec![0u8; 8];
    let mut uv_transform_writer = DatBinWriter::new(&mut uv_transform_bytes);
    assert!(uv_transform.pack(&mut uv_transform_writer));

    let mut unpacked_waveform = Waveform::default();
    let mut unpacked_portal_poly = PortalPoly::default();
    let mut unpacked_material_layer = MaterialLayer::default();
    let mut unpacked_layer_stage = LayerStage::default();
    let mut unpacked_layer_modifier = LayerModifier;
    let mut unpacked_uv_translate = LM_UVTranslate::default();
    let mut unpacked_uv_rotate = LM_UVRotate::default();
    let mut unpacked_uv_scale = LM_UVScale::default();
    let mut unpacked_uv_transform = LM_UVTransform::default();

    assert!(unpacked_waveform.unpack(&mut DatBinReader::new(&waveform_bytes[..1])));
    assert!(unpacked_portal_poly.unpack(&mut DatBinReader::new(&portal_poly_bytes[..4])));
    assert!(unpacked_material_layer.unpack(&mut DatBinReader::new(&material_layer_bytes[..16])));
    assert!(unpacked_layer_stage.unpack(&mut DatBinReader::new(
        &layer_stage_bytes[..layer_stage_used]
    )));
    assert!(unpacked_layer_modifier.unpack(&mut DatBinReader::new(&[])));
    assert!(unpacked_uv_translate.unpack(&mut DatBinReader::new(&uv_translate_bytes[..9])));
    assert!(unpacked_uv_rotate.unpack(&mut DatBinReader::new(&uv_rotate_bytes[..13])));
    assert!(unpacked_uv_scale.unpack(&mut DatBinReader::new(&uv_scale_bytes[..9])));
    assert!(unpacked_uv_transform.unpack(&mut DatBinReader::new(&uv_transform_bytes[..1])));

    assert_eq!(0x7B, unpacked_waveform.raw_data);
    assert_eq!(12, unpacked_portal_poly.portal_index);
    assert_eq!(34, unpacked_portal_poly.polygon_id);
    assert_eq!(material_layer, unpacked_material_layer);
    assert_eq!("Diffuse", unpacked_layer_stage.sampler_name.value);
    assert_eq!(0x0500_0010, unpacked_layer_stage.texture);
    assert_eq!(1.5, unpacked_uv_translate.offset_u);
    assert_eq!(2.5, unpacked_uv_translate.offset_v);
    assert_eq!(3.5, unpacked_uv_rotate.angle);
    assert_eq!(4.5, unpacked_uv_scale.scale_u);
    assert_eq!(4, unpacked_uv_transform.type_discriminator);
}

#[test]
fn generated_split_type_surfaces_roundtrip_cleanly() {
    let mut banned_patterns = BannedPatterns::default();
    banned_patterns.patterns.insert(1, PStringBase::from("bad"));
    banned_patterns
        .patterns
        .insert(2, PStringBase::from("worse"));

    let shader_resource = ShaderResourceEntry {
        start_offset: 10,
        resource_id: 20,
        resource_data: 30,
    };

    let sub_palette = SubPalette {
        sub_id: PackedQualifiedDataId::new(0x0400_0010),
        offset: 4,
        num_colors: 5,
    };

    let texture_map_change = TextureMapChange {
        part_index: 7,
        old_texture: PackedQualifiedDataId::new(0x0500_0010),
        new_texture: PackedQualifiedDataId::new(0x0500_0011),
    };

    let mut banned_bytes = vec![0u8; 128];
    let mut banned_writer = DatBinWriter::new(&mut banned_bytes);
    assert!(banned_patterns.pack(&mut banned_writer));
    let banned_used = banned_writer.offset();

    let mut shader_bytes = vec![0u8; 32];
    let mut shader_writer = DatBinWriter::new(&mut shader_bytes);
    assert!(shader_resource.pack(&mut shader_writer));

    let mut sub_palette_bytes = vec![0u8; 32];
    let mut sub_palette_writer = DatBinWriter::new(&mut sub_palette_bytes);
    assert!(sub_palette.pack(&mut sub_palette_writer));

    let mut texture_change_bytes = vec![0u8; 32];
    let mut texture_change_writer = DatBinWriter::new(&mut texture_change_bytes);
    assert!(texture_map_change.pack(&mut texture_change_writer));

    let mut unpacked_banned = BannedPatterns::default();
    let mut unpacked_shader = ShaderResourceEntry::default();
    let mut unpacked_sub_palette = SubPalette::default();
    let mut unpacked_texture_change = TextureMapChange::default();

    assert!(unpacked_banned.unpack(&mut DatBinReader::new(&banned_bytes[..banned_used])));
    assert!(unpacked_shader.unpack(&mut DatBinReader::new(&shader_bytes[..12])));
    assert!(unpacked_sub_palette.unpack(&mut DatBinReader::new(&sub_palette_bytes[..6])));
    assert!(unpacked_texture_change.unpack(&mut DatBinReader::new(&texture_change_bytes[..9])));

    assert_eq!("bad", unpacked_banned.patterns.get(&1).unwrap().value);
    assert_eq!("worse", unpacked_banned.patterns.get(&2).unwrap().value);
    assert_eq!(shader_resource, unpacked_shader);
    assert_eq!(0x0400_0010, unpacked_sub_palette.sub_id.data_id);
    assert_eq!(4, unpacked_sub_palette.offset);
    assert_eq!(5, unpacked_sub_palette.num_colors);
    assert_eq!(7, unpacked_texture_change.part_index);
    assert_eq!(0x0500_0010, unpacked_texture_change.old_texture.data_id);
    assert_eq!(0x0500_0011, unpacked_texture_change.new_texture.data_id);
}

#[test]
fn generated_hook_wrappers_roundtrip_first_batch() {
    use dat_reader_writer::Generated::Enums::{AnimationHookDir::AnimationHookDir, Sound::Sound};

    let sound_hook = SoundHook {
        direction: AnimationHookDir::FORWARD,
        id: QualifiedDataId::new(0x0A00_0010),
    };
    let sound_table_hook = SoundTableHook {
        direction: AnimationHookDir::BACKWARD,
        sound_type: Sound::ATTACK1,
    };
    let attack_hook = AttackHook {
        direction: AnimationHookDir::BOTH,
        attack_cone: AttackCone {
            part_index: 3,
            left_x: 1.0,
            left_y: 2.0,
            right_x: 3.0,
            right_y: 4.0,
            radius: 5.0,
            height: 6.0,
        },
    };
    let animation_done_hook = AnimationDoneHook {
        direction: AnimationHookDir::FORWARD,
    };
    let replace_object_hook = ReplaceObjectHook {
        direction: AnimationHookDir::FORWARD,
        part_index: 5,
        part_id: PackedQualifiedDataId::new(0x0100_0020),
    };
    let ethereal_hook = EtherealHook {
        direction: AnimationHookDir::BACKWARD,
        ethereal: true,
    };

    let mut sound_bytes = vec![0u8; 32];
    let mut sound_writer = DatBinWriter::new(&mut sound_bytes);
    assert!(sound_hook.pack(&mut sound_writer));
    let sound_used = sound_writer.offset();

    let mut sound_table_bytes = vec![0u8; 32];
    let mut sound_table_writer = DatBinWriter::new(&mut sound_table_bytes);
    assert!(sound_table_hook.pack(&mut sound_table_writer));
    let sound_table_used = sound_table_writer.offset();

    let mut attack_bytes = vec![0u8; 64];
    let mut attack_writer = DatBinWriter::new(&mut attack_bytes);
    assert!(attack_hook.pack(&mut attack_writer));
    let attack_used = attack_writer.offset();

    let mut done_bytes = vec![0u8; 16];
    let mut done_writer = DatBinWriter::new(&mut done_bytes);
    assert!(animation_done_hook.pack(&mut done_writer));
    let done_used = done_writer.offset();

    let mut replace_bytes = vec![0u8; 32];
    let mut replace_writer = DatBinWriter::new(&mut replace_bytes);
    assert!(replace_object_hook.pack(&mut replace_writer));
    let replace_used = replace_writer.offset();

    let mut ethereal_bytes = vec![0u8; 16];
    let mut ethereal_writer = DatBinWriter::new(&mut ethereal_bytes);
    assert!(ethereal_hook.pack(&mut ethereal_writer));
    let ethereal_used = ethereal_writer.offset();

    let mut unpacked_sound = SoundHook::default();
    let mut unpacked_sound_table = SoundTableHook::default();
    let mut unpacked_attack = AttackHook::default();
    let mut unpacked_done = AnimationDoneHook::default();
    let mut unpacked_replace = ReplaceObjectHook::default();
    let mut unpacked_ethereal = EtherealHook::default();

    assert!(unpacked_sound.unpack(&mut DatBinReader::new(&sound_bytes[..sound_used])));
    assert!(unpacked_sound_table.unpack(&mut DatBinReader::new(
        &sound_table_bytes[..sound_table_used]
    )));
    assert!(unpacked_attack.unpack(&mut DatBinReader::new(&attack_bytes[..attack_used])));
    assert!(unpacked_done.unpack(&mut DatBinReader::new(&done_bytes[..done_used])));
    assert!(unpacked_replace.unpack(&mut DatBinReader::new(&replace_bytes[..replace_used])));
    assert!(unpacked_ethereal.unpack(&mut DatBinReader::new(&ethereal_bytes[..ethereal_used])));

    assert_eq!(0x0A00_0010, unpacked_sound.id.data_id);
    assert_eq!(Sound::ATTACK1, unpacked_sound_table.sound_type);
    assert_eq!(3, unpacked_attack.attack_cone.part_index);
    assert_eq!(5, unpacked_replace.part_index);
    assert_eq!(0x0100_0020, unpacked_replace.part_id.data_id);
    assert!(unpacked_ethereal.ethereal);

    let mut enum_sound = AnimationHook::default();
    assert!(enum_sound.unpack(&mut DatBinReader::new(&sound_bytes[..sound_used])));
    assert!(matches!(
        enum_sound,
        AnimationHook::Sound { id, .. } if id.data_id == 0x0A00_0010
    ));

    let mut enum_replace = AnimationHook::default();
    assert!(enum_replace.unpack(&mut DatBinReader::new(&replace_bytes[..replace_used])));
    assert!(matches!(
        enum_replace,
        AnimationHook::ReplaceObject { part_index, .. } if part_index == 5
    ));
}

#[test]
fn generated_hook_wrappers_roundtrip_second_batch() {
    use dat_reader_writer::Generated::Enums::AnimationHookDir::AnimationHookDir;

    let transparent_hook = TransparentHook {
        direction: AnimationHookDir::FORWARD,
        start: 0.1,
        end: 0.2,
        time: 0.3,
    };
    let transparent_part_hook = TransparentPartHook {
        direction: AnimationHookDir::BACKWARD,
        part_index: 9,
        start: 0.4,
        end: 0.5,
        time: 0.6,
    };
    let luminous_hook = LuminousHook {
        direction: AnimationHookDir::FORWARD,
        start: 1.1,
        end: 1.2,
        time: 1.3,
    };
    let luminous_part_hook = LuminousPartHook {
        direction: AnimationHookDir::BACKWARD,
        part_index: 10,
        start: 1.4,
        end: 1.5,
        time: 1.6,
    };
    let diffuse_hook = DiffuseHook {
        direction: AnimationHookDir::FORWARD,
        start: 2.1,
        end: 2.2,
        time: 2.3,
    };
    let diffuse_part_hook = DiffusePartHook {
        direction: AnimationHookDir::BACKWARD,
        part_index: 11,
        start: 2.4,
        end: 2.5,
        time: 2.6,
    };
    let scale_hook = ScaleHook {
        direction: AnimationHookDir::FORWARD,
        end: 3.2,
        time: 3.3,
    };
    let destroy_hook = DestroyParticleHook {
        direction: AnimationHookDir::FORWARD,
        emitter_id: 12,
    };
    let stop_hook = StopParticleHook {
        direction: AnimationHookDir::BACKWARD,
        emitter_id: 13,
    };
    let no_draw_hook = NoDrawHook {
        direction: AnimationHookDir::FORWARD,
        no_draw: true,
    };
    let default_script_hook = DefaultScriptHook {
        direction: AnimationHookDir::BOTH,
    };
    let default_script_part_hook = DefaultScriptPartHook {
        direction: AnimationHookDir::FORWARD,
        part_index: 14,
    };
    let call_pes_hook = CallPESHook {
        direction: AnimationHookDir::BACKWARD,
        pes: 15,
        pause: 4.5,
    };
    let set_omega_hook = SetOmegaHook {
        direction: AnimationHookDir::FORWARD,
        axis: dat_reader_writer::Lib::IO::Numerics::Vector3::new(7.0, 8.0, 9.0),
    };
    let texture_velocity_hook = TextureVelocityHook {
        direction: AnimationHookDir::FORWARD,
        u_speed: 5.1,
        v_speed: 5.2,
    };
    let texture_velocity_part_hook = TextureVelocityPartHook {
        direction: AnimationHookDir::BACKWARD,
        part_index: 16,
        u_speed: 5.3,
        v_speed: 5.4,
    };
    let set_light_hook = SetLightHook {
        direction: AnimationHookDir::FORWARD,
        lights_on: true,
    };
    let create_blocking_hook = CreateBlockingParticleHook {
        direction: AnimationHookDir::BOTH,
    };

    fn roundtrip_bytes<T: IPackable + IUnpackable + Default>(value: &T, size: usize) -> T {
        let mut bytes = vec![0u8; size];
        let mut writer = DatBinWriter::new(&mut bytes);
        assert!(value.pack(&mut writer));
        let used = writer.offset();
        let mut unpacked = T::default();
        assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
        unpacked
    }

    let unpacked_transparent = roundtrip_bytes(&transparent_hook, 32);
    let unpacked_transparent_part = roundtrip_bytes(&transparent_part_hook, 32);
    let unpacked_luminous = roundtrip_bytes(&luminous_hook, 32);
    let unpacked_luminous_part = roundtrip_bytes(&luminous_part_hook, 32);
    let unpacked_diffuse = roundtrip_bytes(&diffuse_hook, 32);
    let unpacked_diffuse_part = roundtrip_bytes(&diffuse_part_hook, 32);
    let unpacked_scale = roundtrip_bytes(&scale_hook, 24);
    let unpacked_destroy = roundtrip_bytes(&destroy_hook, 16);
    let unpacked_stop = roundtrip_bytes(&stop_hook, 16);
    let unpacked_no_draw = roundtrip_bytes(&no_draw_hook, 16);
    let unpacked_default_script = roundtrip_bytes(&default_script_hook, 16);
    let unpacked_default_script_part = roundtrip_bytes(&default_script_part_hook, 16);
    let unpacked_call_pes = roundtrip_bytes(&call_pes_hook, 24);
    let unpacked_set_omega = roundtrip_bytes(&set_omega_hook, 24);
    let unpacked_texture_velocity = roundtrip_bytes(&texture_velocity_hook, 24);
    let unpacked_texture_velocity_part = roundtrip_bytes(&texture_velocity_part_hook, 24);
    let unpacked_set_light = roundtrip_bytes(&set_light_hook, 16);
    let unpacked_create_blocking = roundtrip_bytes(&create_blocking_hook, 16);

    assert_eq!(9, unpacked_transparent_part.part_index);
    assert_eq!(1.3, unpacked_luminous.time);
    assert_eq!(10, unpacked_luminous_part.part_index);
    assert_eq!(2.2, unpacked_diffuse.end);
    assert_eq!(11, unpacked_diffuse_part.part_index);
    assert_eq!(3.2, unpacked_scale.end);
    assert_eq!(12, unpacked_destroy.emitter_id);
    assert_eq!(13, unpacked_stop.emitter_id);
    assert!(unpacked_no_draw.no_draw);
    assert_eq!(14, unpacked_default_script_part.part_index);
    assert_eq!(15, unpacked_call_pes.pes);
    assert_eq!(
        dat_reader_writer::Lib::IO::Numerics::Vector3::new(7.0, 8.0, 9.0),
        unpacked_set_omega.axis
    );
    assert_eq!(5.1, unpacked_texture_velocity.u_speed);
    assert_eq!(16, unpacked_texture_velocity_part.part_index);
    assert!(unpacked_set_light.lights_on);
    assert_eq!(AnimationHookDir::BOTH, unpacked_create_blocking.direction);

    let mut enum_no_draw = AnimationHook::default();
    let mut bytes = vec![0u8; 16];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(no_draw_hook.pack(&mut writer));
    let used = writer.offset();
    assert!(enum_no_draw.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert!(matches!(
        enum_no_draw,
        AnimationHook::NoDraw { no_draw, .. } if no_draw
    ));

    let _ = unpacked_transparent;
    let _ = unpacked_default_script;
}

#[test]
fn generated_hook_wrappers_roundtrip_final_batch() {
    use dat_reader_writer::Generated::Enums::AnimationHookDir::AnimationHookDir;

    let create_particle_hook = CreateParticleHook {
        direction: AnimationHookDir::FORWARD,
        emitter_info_id: QualifiedDataId::new(0x3200_0010),
        part_index: 17,
        offset: Frame {
            origin: dat_reader_writer::Lib::IO::Numerics::Vector3::new(1.0, 2.0, 3.0),
            orientation: dat_reader_writer::Lib::IO::Numerics::Quaternion::new(0.0, 0.0, 0.0, 1.0),
        },
        emitter_id: 18,
    };
    let sound_tweaked_hook = SoundTweakedHook {
        direction: AnimationHookDir::BACKWARD,
        sound_id: QualifiedDataId::new(0x0A00_0020),
        priority: 6.1,
        probability: 6.2,
        volume: 6.3,
    };

    let mut create_particle_bytes = vec![0u8; 64];
    let mut create_particle_writer = DatBinWriter::new(&mut create_particle_bytes);
    assert!(create_particle_hook.pack(&mut create_particle_writer));
    let create_particle_used = create_particle_writer.offset();

    let mut sound_tweaked_bytes = vec![0u8; 32];
    let mut sound_tweaked_writer = DatBinWriter::new(&mut sound_tweaked_bytes);
    assert!(sound_tweaked_hook.pack(&mut sound_tweaked_writer));
    let sound_tweaked_used = sound_tweaked_writer.offset();

    let mut unpacked_create_particle = CreateParticleHook::default();
    let mut unpacked_sound_tweaked = SoundTweakedHook::default();

    assert!(unpacked_create_particle.unpack(&mut DatBinReader::new(
        &create_particle_bytes[..create_particle_used]
    )));
    assert!(unpacked_sound_tweaked.unpack(&mut DatBinReader::new(
        &sound_tweaked_bytes[..sound_tweaked_used]
    )));

    assert_eq!(
        0x3200_0010,
        unpacked_create_particle.emitter_info_id.data_id
    );
    assert_eq!(17, unpacked_create_particle.part_index);
    assert_eq!(18, unpacked_create_particle.emitter_id);
    assert_eq!(0x0A00_0020, unpacked_sound_tweaked.sound_id.data_id);
    assert_eq!(6.1, unpacked_sound_tweaked.priority);
    assert_eq!(6.2, unpacked_sound_tweaked.probability);
    assert_eq!(6.3, unpacked_sound_tweaked.volume);

    let mut enum_create_particle = AnimationHook::default();
    assert!(enum_create_particle.unpack(&mut DatBinReader::new(
        &create_particle_bytes[..create_particle_used]
    )));
    assert!(matches!(
        enum_create_particle,
        AnimationHook::CreateParticle { part_index, .. } if part_index == 17
    ));

    let mut enum_sound_tweaked = AnimationHook::default();
    assert!(enum_sound_tweaked.unpack(&mut DatBinReader::new(
        &sound_tweaked_bytes[..sound_tweaked_used]
    )));
    assert!(matches!(
        enum_sound_tweaked,
        AnimationHook::SoundTweaked { priority, .. } if (priority - 6.1).abs() < f32::EPSILON
    ));
}

#[test]
fn animation_hook_unknown_variant_preserves_raw_payload_bytes() {
    use dat_reader_writer::Generated::Enums::{
        AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType,
    };

    let payload = vec![0xAA, 0xBB, 0xCC, 0xDD, 0x01, 0x02];
    let mut bytes = vec![0u8; 64];
    let used = {
        let mut writer = DatBinWriter::new(&mut bytes);
        writer.write_u32(0xFFFF_FF00);
        writer.write_u32(AnimationHookDir::BACKWARD.into());
        for byte in &payload {
            writer.write_byte(*byte);
        }
        writer.offset()
    };

    let mut hook = AnimationHook::default();
    assert!(hook.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert!(matches!(
        &hook,
        AnimationHook::Unknown {
            hook_type,
            direction,
            payload: stored_payload,
        } if *hook_type == AnimationHookType::from(0xFFFF_FF00)
            && *direction == AnimationHookDir::BACKWARD
            && stored_payload == &payload
    ));

    let mut repacked = vec![0u8; 64];
    let repacked_used = {
        let mut writer = DatBinWriter::new(&mut repacked);
        assert!(hook.pack(&mut writer));
        writer.offset()
    };

    assert_eq!(&bytes[..used], &repacked[..repacked_used]);
}
