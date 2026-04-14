#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ComponentType(pub u32);

impl ComponentType {
    pub const Undef: Self = Self(0x00000000);
    pub const Scarab: Self = Self(0x00000001);
    pub const Herb: Self = Self(0x00000002);
    pub const Powder: Self = Self(0x00000003);
    pub const Potion: Self = Self(0x00000004);
    pub const Talisman: Self = Self(0x00000005);
    pub const Taper: Self = Self(0x00000006);
    pub const PotionPea: Self = Self(0x00000007);
    pub const TalismanPea: Self = Self(0x00000005);
    pub const TaperPea: Self = Self(0x00000007);

    pub const UNDEF: Self = Self::Undef;
    pub const SCARAB: Self = Self::Scarab;
    pub const HERB: Self = Self::Herb;
    pub const POWDER: Self = Self::Powder;
    pub const POTION: Self = Self::Potion;
    pub const TALISMAN: Self = Self::Talisman;
    pub const TAPER: Self = Self::Taper;
    pub const POTION_PEA: Self = Self::PotionPea;
    pub const TALISMAN_PEA: Self = Self::TalismanPea;
    pub const TAPER_PEA: Self = Self::TaperPea;
}

impl From<u32> for ComponentType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<ComponentType> for u32 {
    fn from(value: ComponentType) -> Self {
        value.0
    }
}
