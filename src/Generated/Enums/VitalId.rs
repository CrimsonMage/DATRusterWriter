#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct VitalId(pub u32);

impl VitalId {
    pub const MaximumHealth: Self = Self(0x01);
    pub const MaximumStamina: Self = Self(0x03);
    pub const MaximumMana: Self = Self(0x05);

    pub const MAXIMUM_HEALTH: Self = Self::MaximumHealth;
    pub const MAXIMUM_STAMINA: Self = Self::MaximumStamina;
    pub const MAXIMUM_MANA: Self = Self::MaximumMana;
}

impl From<u32> for VitalId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<VitalId> for u32 {
    fn from(value: VitalId) -> Self {
        value.0
    }
}
