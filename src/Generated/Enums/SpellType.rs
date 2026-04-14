#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SpellType(pub u32);

impl SpellType {
    pub const Undef: Self = Self(0x00000000);
    pub const Enchantment: Self = Self(0x00000001);
    pub const Projectile: Self = Self(0x00000002);
    pub const Boost: Self = Self(0x00000003);
    pub const Transfer: Self = Self(0x00000004);
    pub const PortalLink: Self = Self(0x00000005);
    pub const PortalRecall: Self = Self(0x00000006);
    pub const PortalSummon: Self = Self(0x00000007);
    pub const PortalSending: Self = Self(0x00000008);
    pub const Dispel: Self = Self(0x00000009);
    pub const LifeProjectile: Self = Self(0x0000000A);
    pub const FellowBoost: Self = Self(0x0000000B);
    pub const FellowEnchantment: Self = Self(0x0000000C);
    pub const FellowPortalSending: Self = Self(0x0000000D);
    pub const FellowDispel: Self = Self(0x0000000E);
    pub const EnchantmentProjectile: Self = Self(0x0000000F);
}

impl From<u32> for SpellType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<SpellType> for u32 {
    fn from(value: SpellType) -> Self {
        value.0
    }
}
