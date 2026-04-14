use std::any::Any;

use crate::{
    Generated::Enums::{
        DBObjHeaderFlags::DBObjHeaderFlags, DBObjType::DBObjType, DatFileType::DatFileType,
    },
    Lib::{
        Attributes::DBObjTypeAttribute::DBObjTypeAttribute,
        IO::{
            DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IDBObj::IDBObj,
            IPackable::IPackable, IUnpackable::IUnpackable,
        },
    },
    Types::{
        DBObj::{DBObj, DBObjBase},
        PStringBase::PStringBase,
    },
};

pub const LANGUAGE_INFO_ATTR: DBObjTypeAttribute = DBObjTypeAttribute {
    rust_type_name: "LanguageInfo",
    dat_file_type: DatFileType::Local,
    db_obj_type: DBObjType::LanguageInfo,
    header_flags: DBObjHeaderFlags::None,
    first_id: 0x41000000,
    last_id: 0x41FFFFFF,
    mask_id: 0x00000000,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LanguageInfo {
    pub base: DBObjBase,
    pub version: i32,
    pub base_value: u16,
    pub num_decimal_digits: u16,
    pub leading_zero: bool,
    pub grouping_size: u16,
    pub numerals: PStringBase<u16>,
    pub decimal_separator: PStringBase<u16>,
    pub grouping_separator: PStringBase<u16>,
    pub negative_number_format: PStringBase<u16>,
    pub is_zero_singular: bool,
    pub is_one_singular: bool,
    pub is_negative_one_singular: bool,
    pub is_two_or_more_singular: bool,
    pub is_negative_two_or_less_singular: bool,
    pub treasure_prefix_letters: PStringBase<u16>,
    pub treasure_middle_letters: PStringBase<u16>,
    pub treasure_suffix_letters: PStringBase<u16>,
    pub male_player_letters: PStringBase<u16>,
    pub female_player_letters: PStringBase<u16>,
    pub ime_enabled_setting: u32,
    pub symbol_color: u32,
    pub symbol_color_text: u32,
    pub symbol_height: u32,
    pub symbol_translucence: u32,
    pub symbol_placement: u32,
    pub cand_color_base: u32,
    pub cand_color_border: u32,
    pub cand_color_text: u32,
    pub comp_color_input: u32,
    pub comp_color_target_conv: u32,
    pub comp_color_converted: u32,
    pub comp_color_target_not_conv: u32,
    pub comp_color_input_err: u32,
    pub comp_translucence: u32,
    pub comp_color_text: u32,
    pub other_ime: u32,
    pub word_wrap_on_space: i32,
    pub additional_settings: PStringBase<u16>,
    pub additional_flags: u32,
}

impl DBObj for LanguageInfo {
    fn header_flags(&self) -> DBObjHeaderFlags {
        DBObjHeaderFlags::None
    }
    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LanguageInfo
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

impl IUnpackable for LanguageInfo {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _ = self.base.unpack_with_flags(reader, DBObjHeaderFlags::None);
        self.version = reader.read_i32();
        self.base_value = reader.read_u16();
        self.num_decimal_digits = reader.read_u16();
        self.leading_zero = reader.read_bool(1);
        self.grouping_size = reader.read_u16();
        self.numerals = reader.read_item::<PStringBase<u16>>();
        self.decimal_separator = reader.read_item::<PStringBase<u16>>();
        self.grouping_separator = reader.read_item::<PStringBase<u16>>();
        self.negative_number_format = reader.read_item::<PStringBase<u16>>();
        self.is_zero_singular = reader.read_bool(1);
        self.is_one_singular = reader.read_bool(1);
        self.is_negative_one_singular = reader.read_bool(1);
        self.is_two_or_more_singular = reader.read_bool(1);
        self.is_negative_two_or_less_singular = reader.read_bool(1);
        reader.align(4);
        self.treasure_prefix_letters = reader.read_item::<PStringBase<u16>>();
        self.treasure_middle_letters = reader.read_item::<PStringBase<u16>>();
        self.treasure_suffix_letters = reader.read_item::<PStringBase<u16>>();
        self.male_player_letters = reader.read_item::<PStringBase<u16>>();
        self.female_player_letters = reader.read_item::<PStringBase<u16>>();
        self.ime_enabled_setting = reader.read_u32();
        self.symbol_color = reader.read_u32();
        self.symbol_color_text = reader.read_u32();
        self.symbol_height = reader.read_u32();
        self.symbol_translucence = reader.read_u32();
        self.symbol_placement = reader.read_u32();
        self.cand_color_base = reader.read_u32();
        self.cand_color_border = reader.read_u32();
        self.cand_color_text = reader.read_u32();
        self.comp_color_input = reader.read_u32();
        self.comp_color_target_conv = reader.read_u32();
        self.comp_color_converted = reader.read_u32();
        self.comp_color_target_not_conv = reader.read_u32();
        self.comp_color_input_err = reader.read_u32();
        self.comp_translucence = reader.read_u32();
        self.comp_color_text = reader.read_u32();
        self.other_ime = reader.read_u32();
        self.word_wrap_on_space = reader.read_i32();
        self.additional_settings = reader.read_item::<PStringBase<u16>>();
        self.additional_flags = reader.read_u32();
        true
    }
}

impl IPackable for LanguageInfo {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.base.pack_with_flags(writer, DBObjHeaderFlags::None);
        writer.write_i32(self.version);
        writer.write_u16(self.base_value);
        writer.write_u16(self.num_decimal_digits);
        writer.write_bool(self.leading_zero, 1);
        writer.write_u16(self.grouping_size);
        writer.write_item(&self.numerals);
        writer.write_item(&self.decimal_separator);
        writer.write_item(&self.grouping_separator);
        writer.write_item(&self.negative_number_format);
        writer.write_bool(self.is_zero_singular, 1);
        writer.write_bool(self.is_one_singular, 1);
        writer.write_bool(self.is_negative_one_singular, 1);
        writer.write_bool(self.is_two_or_more_singular, 1);
        writer.write_bool(self.is_negative_two_or_less_singular, 1);
        writer.align(4);
        writer.write_item(&self.treasure_prefix_letters);
        writer.write_item(&self.treasure_middle_letters);
        writer.write_item(&self.treasure_suffix_letters);
        writer.write_item(&self.male_player_letters);
        writer.write_item(&self.female_player_letters);
        writer.write_u32(self.ime_enabled_setting);
        writer.write_u32(self.symbol_color);
        writer.write_u32(self.symbol_color_text);
        writer.write_u32(self.symbol_height);
        writer.write_u32(self.symbol_translucence);
        writer.write_u32(self.symbol_placement);
        writer.write_u32(self.cand_color_base);
        writer.write_u32(self.cand_color_border);
        writer.write_u32(self.cand_color_text);
        writer.write_u32(self.comp_color_input);
        writer.write_u32(self.comp_color_target_conv);
        writer.write_u32(self.comp_color_converted);
        writer.write_u32(self.comp_color_target_not_conv);
        writer.write_u32(self.comp_color_input_err);
        writer.write_u32(self.comp_translucence);
        writer.write_u32(self.comp_color_text);
        writer.write_u32(self.other_ime);
        writer.write_i32(self.word_wrap_on_space);
        writer.write_item(&self.additional_settings);
        writer.write_u32(self.additional_flags);
        true
    }
}

impl IDBObj for LanguageInfo {
    fn db_obj_type_attr() -> &'static DBObjTypeAttribute
    where
        Self: Sized,
    {
        &LANGUAGE_INFO_ATTR
    }

    fn db_obj_type(&self) -> DBObjType {
        DBObjType::LanguageInfo
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
