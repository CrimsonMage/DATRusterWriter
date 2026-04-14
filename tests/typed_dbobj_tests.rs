use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use dat_reader_writer::{
    DBObjs::{
        DualEnumIDMap::DualEnumIDMap, EnumIDMap::EnumIDMap, EnumMapper::EnumMapper, Font::Font,
        GfxObjDegradeInfo::GfxObjDegradeInfo, Iteration::Iteration, LanguageInfo::LanguageInfo,
        LanguageString::LanguageString,
        MaterialInstance::MaterialInstance, MaterialModifier::MaterialModifier,
        NameFilterTable::NameFilterTable, Palette::Palette, QualityFilter::QualityFilter,
        SpellComponentTable::SpellComponentTable, StringTable::StringTable,
        RenderMaterial::RenderMaterial, RenderTexture::RenderTexture,
    },
    DatDatabase::DatDatabase,
    Generated::Enums::{DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{
        DBObjAttributeCache,
        IO::{DatBinWriter::DatBinWriter, DatHeader::DatHeader, IPackable::IPackable},
    },
    Options::DatDatabaseOptions::DatDatabaseOptions,
    Types::{
        AutoGrowHashTable::AutoGrowHashTable, FontCharDesc::FontCharDesc, HashTable::HashTable,
        IntrusiveHashTable::IntrusiveHashTable, NameFilterLanguageData::NameFilterLanguageData,
        MaterialProperty::MaterialProperty, ObfuscatedPStringBase::ObfuscatedPStringBase,
        PStringBase::PStringBase, QualifiedDataId::QualifiedDataId,
        SpellComponentBase::SpellComponentBase, StringTableString::StringTableString,
    },
};
use uuid::Uuid;

fn unique_temp_file() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir();
    dir.join(format!("dat_reader_writer_typed_{stamp}.dat"))
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
    let language_info =
        DBObjAttributeCache::type_from_id(DatFileType::Local, 0x41000010).unwrap();
    let name_filter =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E000020).unwrap();
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
    let spell_component_table =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E00000F).unwrap();
    let quality_filter =
        DBObjAttributeCache::type_from_id(DatFileType::Portal, 0x0E010010).unwrap();

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
    assert_eq!(DBObjType::GfxObjDegradeInfo, gfx_obj_degrade_info.db_obj_type);
    assert_eq!(DBObjType::SpellComponentTable, spell_component_table.db_obj_type);
    assert_eq!(DBObjType::QualityFilter, quality_filter.db_obj_type);
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
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::EnumMapper));
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::EnumIDMap));
    assert!(
        attrs
            .iter()
            .any(|attr| attr.db_obj_type == DBObjType::DualEnumIDMap)
    );
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::RenderTexture));
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::RenderMaterial));
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::MaterialModifier));
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::MaterialInstance));
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::GfxObjDegradeInfo));
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::SpellComponentTable));
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::QualityFilter));
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
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::VitalTable));
    assert!(attrs.iter().any(|attr| attr.db_obj_type == DBObjType::SkillTable));
}

#[test]
fn dat_database_can_read_typed_vital_table() {
    use dat_reader_writer::{
        DBObjs::VitalTable::VitalTable,
        Generated::Enums::AttributeId::AttributeId,
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
    assert_eq!(2, read_name_filter.language_data.get(&1).unwrap().compound_letter_groups.len());
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
        build_single_block_dat(DatFileType::Portal, 0x22000010, &mapper_payload[..mapper_used]),
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

    let read_mapper = mapper_db.try_get::<EnumMapper>(0x22000010).unwrap().unwrap();
    assert_eq!(0x10, read_mapper.base_enum_map);
    assert_eq!("one", read_mapper.id_to_string_map.get(&1).unwrap().value);

    let read_enum_id = enum_id_db.try_get::<EnumIDMap>(0x25000010).unwrap().unwrap();
    assert_eq!(Some(&0x05000010), read_enum_id.client_enum_to_id.get(&4));
    assert_eq!("client", read_enum_id.client_enum_to_name.get(&2).unwrap().value);

    let read_dual_enum_id = dual_enum_id_db
        .try_get::<DualEnumIDMap>(0x27000010)
        .unwrap()
        .unwrap();
    assert_eq!(Some(&0x05000020), read_dual_enum_id.server_enum_to_id.get(&5));
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
        source_levels: vec![QualifiedDataId::new(0x06000010), QualifiedDataId::new(0x06000011)],
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
    assert_eq!(0x11223344, read_material_modifier.material_properties[0].name_id);

    let read_material_instance = material_instance_db
        .try_get::<MaterialInstance>(0x18000010)
        .unwrap()
        .unwrap();
    assert_eq!(0x16000010, read_material_instance.material_id);
    assert_eq!(vec![0x17000010, 0x17000011], read_material_instance.modifier_refs);
    assert!(read_material_instance.allow_stencil_shadows);
    assert!(!read_material_instance.want_discard_geometry);
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

    let mut components = dat_reader_writer::Types::PackableHashTable::PackableHashTable::<u32, SpellComponentBase>::default();
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
