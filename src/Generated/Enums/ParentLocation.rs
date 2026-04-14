#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ParentLocation(pub i32);

impl ParentLocation {
    pub const None: Self = Self(0x00000000);
    pub const RightHand: Self = Self(0x00000001);
    pub const LeftHand: Self = Self(0x00000002);
    pub const Shield: Self = Self(0x00000003);
    pub const Belt: Self = Self(0x00000004);
    pub const Quiver: Self = Self(0x00000005);
    pub const Hearldry: Self = Self(0x00000006);
    pub const Mouth: Self = Self(0x00000007);
    pub const LeftWeapon: Self = Self(0x00000008);
    pub const LeftUnarmed: Self = Self(0x00000009);

    pub const NONE: Self = Self::None;
    pub const RIGHT_HAND: Self = Self::RightHand;
    pub const LEFT_HAND: Self = Self::LeftHand;
    pub const SHIELD: Self = Self::Shield;
    pub const BELT: Self = Self::Belt;
    pub const QUIVER: Self = Self::Quiver;
    pub const HEARLDRY: Self = Self::Hearldry;
    pub const MOUTH: Self = Self::Mouth;
    pub const LEFT_WEAPON: Self = Self::LeftWeapon;
    pub const LEFT_UNARMED: Self = Self::LeftUnarmed;
}

impl From<i32> for ParentLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<ParentLocation> for i32 {
    fn from(value: ParentLocation) -> Self {
        value.0
    }
}
