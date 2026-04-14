#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct MotionStance(pub u32);

impl MotionStance {
    pub const INVALID: Self = Self(0x80000000);
    pub const HAND_COMBAT: Self = Self(0x8000003C);
    pub const NON_COMBAT: Self = Self(0x8000003D);
    pub const SWORD_COMBAT: Self = Self(0x8000003E);
    pub const BOW_COMBAT: Self = Self(0x8000003F);
    pub const SWORD_SHIELD_COMBAT: Self = Self(0x80000040);
    pub const CROSSBOW_COMBAT: Self = Self(0x80000041);
    pub const UNUSED_COMBAT: Self = Self(0x80000042);
    pub const SLING_COMBAT: Self = Self(0x80000043);
    pub const TWO_HANDED_SWORD_COMBAT: Self = Self(0x80000044);
    pub const TWO_HANDED_STAFF_COMBAT: Self = Self(0x80000045);
    pub const DUAL_WIELD_COMBAT: Self = Self(0x80000046);
    pub const THROWN_WEAPON_COMBAT: Self = Self(0x80000047);
    pub const GRAZE: Self = Self(0x80000048);
    pub const MAGIC: Self = Self(0x80000049);
    pub const BOW_NO_AMMO: Self = Self(0x800000E8);
    pub const CROSS_BOW_NO_AMMO: Self = Self(0x800000E9);
    pub const ATLATL_COMBAT: Self = Self(0x8000013B);
    pub const THROWN_SHIELD_COMBAT: Self = Self(0x8000013C);
}

impl From<u32> for MotionStance { fn from(value: u32) -> Self { Self(value) } }
impl From<MotionStance> for u32 { fn from(value: MotionStance) -> Self { value.0 } }
