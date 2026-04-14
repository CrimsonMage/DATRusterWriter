#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct MagicSchool(pub i32);

impl MagicSchool {
    pub const None: Self = Self(0x00000000);
    pub const WarMagic: Self = Self(0x00000001);
    pub const LifeMagic: Self = Self(0x00000002);
    pub const ItemEnchantment: Self = Self(0x00000003);
    pub const CreatureEnchantment: Self = Self(0x00000004);
    pub const VoidMagic: Self = Self(0x00000005);
}

impl From<i32> for MagicSchool {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<MagicSchool> for i32 {
    fn from(value: MagicSchool) -> Self {
        value.0
    }
}
