#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ParentLocation(pub i32);

impl ParentLocation {
    pub const NONE: Self = Self(0x00000000);
    pub const RIGHT_HAND: Self = Self(0x00000001);
    pub const LEFT_HAND: Self = Self(0x00000002);
    pub const SHIELD: Self = Self(0x00000003);
    pub const BELT: Self = Self(0x00000004);
    pub const QUIVER: Self = Self(0x00000005);
    pub const HEARLDRY: Self = Self(0x00000006);
    pub const MOUTH: Self = Self(0x00000007);
    pub const LEFT_WEAPON: Self = Self(0x00000008);
    pub const LEFT_UNARMED: Self = Self(0x00000009);
}

impl From<i32> for ParentLocation {
    fn from(value: i32) -> Self { Self(value) }
}

impl From<ParentLocation> for i32 {
    fn from(value: ParentLocation) -> Self { value.0 }
}
