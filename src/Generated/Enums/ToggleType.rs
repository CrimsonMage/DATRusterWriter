#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ToggleType(pub u32);

impl ToggleType {
    pub const Invalid: Self = Self(0x00000000);
    pub const Momentary: Self = Self(0x00000001);
    pub const Toggle: Self = Self(0x00000002);
    pub const Impulse: Self = Self(0x00000003);
    pub const AutoRepeat: Self = Self(0x00000004);
    pub const Continuous: Self = Self(0x00000005);
}

impl From<u32> for ToggleType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<ToggleType> for u32 {
    fn from(value: ToggleType) -> Self {
        value.0
    }
}
