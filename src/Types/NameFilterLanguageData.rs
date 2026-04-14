use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::PStringBase::PStringBase,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NameFilterLanguageData {
    pub maximum_same_characters_in_a_row: u32,
    pub maximum_vowels_in_a_row: u32,
    pub first_n_characters_must_have_a_vowel: u32,
    pub vowel_containing_substring_length: u32,
    pub extra_allowed_characters: PStringBase<u16>,
    pub compound_letter_groups: Vec<PStringBase<u16>>,
}

impl IUnpackable for NameFilterLanguageData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.maximum_same_characters_in_a_row = reader.read_u32();
        self.maximum_vowels_in_a_row = reader.read_u32();
        self.first_n_characters_must_have_a_vowel = reader.read_u32();
        self.vowel_containing_substring_length = reader.read_u32();
        self.extra_allowed_characters = reader.read_item::<PStringBase<u16>>();

        let count = reader.read_u32() as usize;
        self.compound_letter_groups.clear();
        self.compound_letter_groups.reserve(count);
        for _ in 0..count {
            self.compound_letter_groups
                .push(reader.read_item::<PStringBase<u16>>());
        }

        true
    }
}

impl IPackable for NameFilterLanguageData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.maximum_same_characters_in_a_row);
        writer.write_u32(self.maximum_vowels_in_a_row);
        writer.write_u32(self.first_n_characters_must_have_a_vowel);
        writer.write_u32(self.vowel_containing_substring_length);
        writer.write_item(&self.extra_allowed_characters);
        writer.write_u32(self.compound_letter_groups.len() as u32);
        for group in &self.compound_letter_groups {
            writer.write_item(group);
        }
        true
    }
}
