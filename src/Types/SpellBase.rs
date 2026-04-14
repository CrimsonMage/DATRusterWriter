use encoding_rs::WINDOWS_1252;

use crate::{
    Generated::Enums::{
        ItemType::ItemType, MagicSchool::MagicSchool, PlayScript::PlayScript,
        SpellCategory::SpellCategory, SpellIndex::SpellIndex, SpellType::SpellType,
    },
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::ObfuscatedPStringBase::ObfuscatedPStringBase,
};

const SPELLBASE_NAME_HASH_KEY: u32 = 0x1210_7680;
const SPELLBASE_DESC_HASH_KEY: u32 = 0xBEAD_CF45;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpellBase {
    pub name: ObfuscatedPStringBase,
    pub description: ObfuscatedPStringBase,
    pub components: Vec<u32>,
    pub school: MagicSchool,
    pub icon: u32,
    pub category: SpellCategory,
    pub bitfield: SpellIndex,
    pub base_mana: u32,
    pub base_range_constant: f32,
    pub base_range_mod: f32,
    pub power: u32,
    pub spell_economy_mod: f32,
    pub formula_version: u32,
    pub component_loss: f32,
    pub meta_spell_type: SpellType,
    pub meta_spell_id: u32,
    pub duration: f64,
    pub degrade_modifier: f32,
    pub degrade_limit: f32,
    pub portal_lifetime: f64,
    pub caster_effect: PlayScript,
    pub target_effect: PlayScript,
    pub fizzle_effect: PlayScript,
    pub recovery_interval: f64,
    pub recovery_amount: f32,
    pub display_order: u32,
    pub non_component_target_type: ItemType,
    pub mana_mod: u32,
}

impl SpellBase {
    pub fn get_string_hash(value: &str) -> u32 {
        let mut result: i64 = 0;
        if value.is_empty() {
            return 0;
        }

        let (encoded, _, _) = WINDOWS_1252.encode(value);
        for byte in encoded.iter() {
            result = (*byte as i8 as i64) + (result << 4);
            if (result & 0xF000_0000) != 0 {
                result = (result ^ ((result & 0xF000_0000) >> 24)) & 0x0FFF_FFFF;
            }
        }
        result as u32
    }

    fn hash_key(&self) -> u32 {
        let name_hash = Self::get_string_hash(&self.name.value);
        let desc_hash = Self::get_string_hash(&self.description.value);
        (name_hash % SPELLBASE_NAME_HASH_KEY) + (desc_hash % SPELLBASE_DESC_HASH_KEY)
    }

    fn decrypt_components(&self, encrypted: &[u32; 8]) -> Vec<u32> {
        let key = self.hash_key();
        encrypted
            .iter()
            .map(|component| {
                if *component == 0 {
                    0
                } else {
                    let mut value = component.wrapping_sub(key);
                    if value > 198 {
                        value &= 0xFF;
                    }
                    value
                }
            })
            .filter(|component| *component > 0)
            .collect()
    }

    fn encrypt_components(&self) -> [u32; 8] {
        let key = self.hash_key();
        let mut encrypted = [0u32; 8];
        for (index, component) in self.components.iter().take(8).enumerate() {
            encrypted[index] = if *component == 0 {
                0
            } else {
                component.wrapping_add(key)
            };
        }
        encrypted
    }
}

impl IUnpackable for SpellBase {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.name = reader.read_item::<ObfuscatedPStringBase>();
        reader.align(4);
        self.description = reader.read_item::<ObfuscatedPStringBase>();
        reader.align(4);
        self.school = MagicSchool::from(reader.read_i32());
        self.icon = reader.read_u32();
        self.category = SpellCategory::from(reader.read_u32());
        self.bitfield = SpellIndex::from_bits_truncate(reader.read_i32());
        self.base_mana = reader.read_u32();
        self.base_range_constant = reader.read_single();
        self.base_range_mod = reader.read_single();
        self.power = reader.read_u32();
        self.spell_economy_mod = reader.read_single();
        self.formula_version = reader.read_u32();
        self.component_loss = reader.read_single();
        self.meta_spell_type = SpellType::from(reader.read_u32());
        self.meta_spell_id = reader.read_u32();

        match self.meta_spell_type {
            SpellType::Enchantment | SpellType::FellowEnchantment => {
                self.duration = reader.read_double();
                self.degrade_modifier = reader.read_single();
                self.degrade_limit = reader.read_single();
            }
            SpellType::PortalSummon => {
                self.portal_lifetime = reader.read_double();
            }
            _ => {}
        }

        let encrypted_components = std::array::from_fn(|_| reader.read_u32());
        self.components = self.decrypt_components(&encrypted_components);
        self.caster_effect = PlayScript::from(reader.read_u32());
        self.target_effect = PlayScript::from(reader.read_u32());
        self.fizzle_effect = PlayScript::from(reader.read_u32());
        self.recovery_interval = reader.read_double();
        self.recovery_amount = reader.read_single();
        self.display_order = reader.read_u32();
        self.non_component_target_type = ItemType::from(reader.read_u32());
        self.mana_mod = reader.read_u32();
        true
    }
}

impl IPackable for SpellBase {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.name);
        writer.write_item(&self.description);
        writer.write_i32(self.school.into());
        writer.write_u32(self.icon);
        writer.write_u32(self.category.into());
        writer.write_i32(self.bitfield.bits());
        writer.write_u32(self.base_mana);
        writer.write_single(self.base_range_constant);
        writer.write_single(self.base_range_mod);
        writer.write_u32(self.power);
        writer.write_single(self.spell_economy_mod);
        writer.write_u32(self.formula_version);
        writer.write_single(self.component_loss);
        writer.write_u32(self.meta_spell_type.into());
        writer.write_u32(self.meta_spell_id);

        match self.meta_spell_type {
            SpellType::Enchantment | SpellType::FellowEnchantment => {
                writer.write_double(self.duration);
                writer.write_single(self.degrade_modifier);
                writer.write_single(self.degrade_limit);
            }
            SpellType::PortalSummon => {
                writer.write_double(self.portal_lifetime);
            }
            _ => {}
        }

        for component in self.encrypt_components() {
            writer.write_u32(component);
        }
        writer.write_u32(self.caster_effect.into());
        writer.write_u32(self.target_effect.into());
        writer.write_u32(self.fizzle_effect.into());
        writer.write_double(self.recovery_interval);
        writer.write_single(self.recovery_amount);
        writer.write_u32(self.display_order);
        writer.write_u32(self.non_component_target_type.into());
        writer.write_u32(self.mana_mod);
        true
    }
}
