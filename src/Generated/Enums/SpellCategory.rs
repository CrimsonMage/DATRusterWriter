#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SpellCategory(pub u32);

impl SpellCategory {
    pub const Undef: Self = Self(0x00000000);
    pub const StrengthRaising: Self = Self(0x00000001);
    pub const StrengthLowering: Self = Self(0x00000002);
    pub const EnduranceRaising: Self = Self(0x00000003);
    pub const EnduranceLowering: Self = Self(0x00000004);
    pub const PortalTie: Self = Self(0x000000C8);
    pub const PortalRecall: Self = Self(0x000000C9);
    pub const PortalCreation: Self = Self(0x000000CA);
    pub const PortalItemCreation: Self = Self(0x000000CB);
    pub const PortalSending: Self = Self(0x000000D6);
    pub const HealthDoT: Self = Self(0x00000212);
    pub const HealthHoT: Self = Self(0x00000215);
    pub const SummoningRaising: Self = Self(0x000002B8);
    pub const SummoningLowering: Self = Self(0x000002B9);
}

impl From<u32> for SpellCategory {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<SpellCategory> for u32 {
    fn from(value: SpellCategory) -> Self {
        value.0
    }
}
