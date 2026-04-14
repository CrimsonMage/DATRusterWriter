#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Placement(pub u32);

impl Placement {
    pub const Default: Self = Self(0x00000000);
    pub const RightHandCombat: Self = Self(0x00000001);
    pub const RightHandNonCombat: Self = Self(0x00000002);
    pub const LeftHand: Self = Self(0x00000003);
    pub const Belt: Self = Self(0x00000004);
    pub const Quiver: Self = Self(0x00000005);
    pub const Shield: Self = Self(0x00000006);
    pub const LeftWeapon: Self = Self(0x00000007);
    pub const LeftUnarmed: Self = Self(0x00000008);
    pub const SpecialCrowssbowBolt: Self = Self(0x00000033);
    pub const MissileFlight: Self = Self(0x00000034);
    pub const Resting: Self = Self(0x00000065);
    pub const Other: Self = Self(0x00000066);
    pub const Hook: Self = Self(0x00000067);
    pub const Random1: Self = Self(0x00000079);
    pub const Random2: Self = Self(0x0000007A);
    pub const Random3: Self = Self(0x0000007B);
    pub const Random4: Self = Self(0x0000007C);
    pub const Random5: Self = Self(0x0000007D);
    pub const Random6: Self = Self(0x0000007E);
    pub const Random7: Self = Self(0x0000007F);
    pub const Random8: Self = Self(0x00000080);
    pub const Random9: Self = Self(0x00000081);
    pub const Random10: Self = Self(0x00000082);

    pub const DEFAULT: Self = Self::Default;
    pub const RIGHT_HAND_COMBAT: Self = Self::RightHandCombat;
    pub const RIGHT_HAND_NON_COMBAT: Self = Self::RightHandNonCombat;
    pub const LEFT_HAND: Self = Self::LeftHand;
    pub const BELT: Self = Self::Belt;
    pub const QUIVER: Self = Self::Quiver;
    pub const SHIELD: Self = Self::Shield;
    pub const LEFT_WEAPON: Self = Self::LeftWeapon;
    pub const LEFT_UNARMED: Self = Self::LeftUnarmed;
    pub const SPECIAL_CROWSSBOW_BOLT: Self = Self::SpecialCrowssbowBolt;
    pub const MISSILE_FLIGHT: Self = Self::MissileFlight;
    pub const RESTING: Self = Self::Resting;
    pub const OTHER: Self = Self::Other;
    pub const HOOK: Self = Self::Hook;
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
