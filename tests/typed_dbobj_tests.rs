use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use dat_reader_writer::{
    DBObjs::{
        ActionMap::ActionMap, BadDataTable::BadDataTable, ChatPoseTable::ChatPoseTable,
        ContractTable::ContractTable, DualEnumIDMap::DualEnumIDMap, EnumIDMap::EnumIDMap,
        EnumMapper::EnumMapper, EnvCell::EnvCell, Environment::Environment, Font::Font,
        GfxObjDegradeInfo::GfxObjDegradeInfo, Iteration::Iteration, LandBlock::LandBlock,
        LandBlockInfo::LandBlockInfo, LanguageInfo::LanguageInfo, LanguageString::LanguageString,
        MasterInputMap::MasterInputMap, MasterProperty::MasterProperty,
        MaterialInstance::MaterialInstance, MaterialModifier::MaterialModifier,
        NameFilterTable::NameFilterTable, ObjectHierarchy::ObjectHierarchy, Palette::Palette,
        QualityFilter::QualityFilter, RenderMaterial::RenderMaterial, RenderTexture::RenderTexture,
        SpellComponentTable::SpellComponentTable, SpellTable::SpellTable, StringTable::StringTable,
        TabooTable::TabooTable,
    },
    DatDatabase::DatDatabase,
    Generated::Enums::{
        DBObjType::DBObjType, DatFileType::DatFileType, EnvCellFlags::EnvCellFlags,
        EquipmentSet::EquipmentSet, ItemType::ItemType, MagicSchool::MagicSchool,
        PlayScript::PlayScript, PortalFlags::PortalFlags, SpellCategory::SpellCategory,
        SpellIndex::SpellIndex, SpellType::SpellType, ToggleType::ToggleType,
    },
    Lib::{
        DBObjAttributeCache,
        IO::{DatBinWriter::DatBinWriter, DatHeader::DatHeader, IPackable::IPackable},
    },
    Options::DatDatabaseOptions::DatDatabaseOptions,
    Types::{
        AC1LegacyPStringBase::AC1LegacyPStringBase,
        ActionMapValue::ActionMapValue,
        AutoGrowHashTable::AutoGrowHashTable,
        BSPTrees::{CellBSPNode, CellBSPTree},
        BaseProperty::BaseProperty,
        BasePropertyDesc::BasePropertyDesc,
        BuildingInfo::BuildingInfo,
        BuildingPortal::BuildingPortal,
        CInputMap::CInputMap,
        CellPortal::CellPortal,
        CellStruct::CellStruct,
        ChatEmoteData::ChatEmoteData,
        Contract::Contract,
        DeviceKeyMapEntry::DeviceKeyMapEntry,
        EnumMapperData::EnumMapperData,
        FontCharDesc::FontCharDesc,
        Frame::Frame,
        HashTable::HashTable,
        InputsConflictsValue::InputsConflictsValue,
        IntrusiveHashTable::IntrusiveHashTable,
        MaterialProperty::MaterialProperty,
        NameFilterLanguageData::NameFilterLanguageData,
        ObfuscatedPStringBase::ObfuscatedPStringBase,
        ObjHierarchyNode::ObjHierarchyNode,
        PHashTable::PHashTable,
        PStringBase::PStringBase,
        Position::Position,
        QualifiedControl::QualifiedControl,
        QualifiedDataId::QualifiedDataId,
        SpellBase::SpellBase,
        SpellComponentBase::SpellComponentBase,
        SpellSet::SpellSet,
        SpellSetTiers::SpellSetTiers,
        Stab::Stab,
        StringTableString::StringTableString,
        TabooTableEntry::TabooTableEntry,
        TerrainInfo::TerrainInfo,
        UserBindingData::UserBindingData,
    },
};
use uuid::Uuid;

static UNIQUE_TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

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
    let environment = DBObjAttributeCache::type_from_id(DatFileType::Cell, 0x0D000123).unwrap();
    let land_block_info = DBObjAttributeCache::type_from_id(DatFileType::Cell, 0x0001FFFE).unwrap();
    let land_block = DBObjAttributeCache::type_from_id(DatFileType::Cell, 0x0001FFFF).unwrap();
    let env_cell = DBObjAttributeCache::type_from_id(DatFileType::Cell, 0x00010123).unwrap();
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
    assert_eq!(DBObjType::Environment, environment.db_obj_type);
    assert_eq!(DBObjType::LandBlockInfo, land_block_info.db_obj_type);
    assert_eq!(DBObjType::LandBlock, land_block.db_obj_type);
    assert_eq!(DBObjType::EnvCell, env_cell.db_obj_type);
    assert_eq!(DBObjType::MasterInputMap, master_input_map.db_obj_type);
    assert_eq!(DBObjType::MasterProperty, master_property.db_obj_type);
    assert_eq!(DBObjType::ObjectHierarchy, object_hierarchy.db_obj_type);
    assert_eq!(DBObjType::TabooTable, taboo_table.db_obj_type);
}

#[test]
fn db_obj_attribute_cache_tracks_current_ported_dbobjs() {
    let attrs = DBObjAttributeCache::all_ported_attributes();
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

    let mut client_name_map = IntrusiveHashTable::<u32, PStringBase<u8>>::default();
    client_name_map.insert(2, PStringBase::from("client"));
    let mut server_name_map = IntrusiveHashTable::<u32, PStringBase<u8>>::default();
    server_name_map.insert(3, PStringBase::from("server"));
    let mut client_id_map = IntrusiveHashTable::<u32, u32>::default();
    client_id_map.insert(4, 0x05000010);
    let mut server_id_map = IntrusiveHashTable::<u32, u32>::default();
    server_id_map.insert(5, 0x05000020);

    let enum_id_map = EnumIDMap {
        client_enum_to_id: client_id_map.clone(),
        client_enum_to_name: client_name_map.clone(),
        server_enum_to_id: server_id_map.clone(),
        server_enum_to_name: server_name_map.clone(),
        ..Default::default()
    };
    let dual_enum_id_map = DualEnumIDMap {
        client_enum_to_id: client_id_map,
        client_enum_to_name: client_name_map,
        server_enum_to_id: server_id_map,
        server_enum_to_name: server_name_map,
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

    let mapper_path = unique_temp_file();
    let enum_id_path = unique_temp_file();
    let dual_enum_id_path = unique_temp_file();

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
        &dual_enum_id_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x27000010,
            &dual_enum_id_payload[..dual_enum_id_used],
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
    assert_eq!(
        "client",
        read_enum_id.client_enum_to_name.get(&2).unwrap().value
    );

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
        read_dual_enum_id.server_enum_to_name.get(&3).unwrap().value
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
