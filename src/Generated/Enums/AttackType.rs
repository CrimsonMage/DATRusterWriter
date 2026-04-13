#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct AttackType(pub i32);

impl AttackType {
    pub const UNDEF: Self = Self(0x00000000);
    pub const PUNCH: Self = Self(0x00000001);
    pub const THRUST: Self = Self(0x00000002);
    pub const SLASH: Self = Self(0x00000004);
    pub const KICK: Self = Self(0x00000008);
    pub const OFFHAND_PUNCH: Self = Self(0x00000010);
    pub const PUNCHES: Self = Self(0x00000011);
    pub const UNARMED: Self = Self(0x00000019);
    pub const DOUBLE_SLASH: Self = Self(0x00000020);
    pub const TRIPLE_SLASH: Self = Self(0x00000040);
    pub const DOUBLE_THRUST: Self = Self(0x00000080);
    pub const TRIPLE_THRUST: Self = Self(0x00000100);
    pub const OFFHAND_THRUST: Self = Self(0x00000200);
    pub const OFFHAND_SLASH: Self = Self(0x00000400);
    pub const OFFHAND_DOUBLE_SLASH: Self = Self(0x00000800);
    pub const OFFHAND_TRIPLE_SLASH: Self = Self(0x00001000);
    pub const SLASHES: Self = Self(0x00001C64);
    pub const OFFHAND_DOUBLE_THRUST: Self = Self(0x00002000);
    pub const DOUBLE_STRIKE: Self = Self(0x000028A0);
    pub const OFFHAND_TRIPLE_THRUST: Self = Self(0x00004000);
    pub const TRIPLE_STRIKE: Self = Self(0x00005140);
    pub const THRUSTS: Self = Self(0x00006382);
    pub const MULTI_STRIKE: Self = Self(0x000079E0);
    pub const OFFHAND: Self = Self(0x00007E00);
}

impl From<i32> for AttackType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<AttackType> for i32 {
    fn from(value: AttackType) -> Self {
        value.0
    }
}
