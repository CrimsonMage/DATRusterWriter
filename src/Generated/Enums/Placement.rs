#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Placement(pub u32);

impl Placement {
    pub const DEFAULT: Self = Self(0x00000000);
    pub const RIGHT_HAND_COMBAT: Self = Self(0x00000001);
    pub const RIGHT_HAND_NON_COMBAT: Self = Self(0x00000002);
    pub const LEFT_HAND: Self = Self(0x00000003);
    pub const BELT: Self = Self(0x00000004);
    pub const QUIVER: Self = Self(0x00000005);
    pub const SHIELD: Self = Self(0x00000006);
    pub const LEFT_WEAPON: Self = Self(0x00000007);
    pub const LEFT_UNARMED: Self = Self(0x00000008);
    pub const SPECIAL_CROWSSBOW_BOLT: Self = Self(0x00000033);
    pub const MISSILE_FLIGHT: Self = Self(0x00000034);
    pub const RESTING: Self = Self(0x00000065);
    pub const OTHER: Self = Self(0x00000066);
    pub const HOOK: Self = Self(0x00000067);
}

impl From<u32> for Placement {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Placement> for u32 {
    fn from(value: Placement) -> Self {
        value.0
    }
}
