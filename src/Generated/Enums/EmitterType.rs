#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EmitterType(pub i32);

impl EmitterType {
    pub const Unknown: Self = Self(0x00000000);
    pub const BirthratePerSec: Self = Self(0x00000001);
    pub const BirthratePerMeter: Self = Self(0x00000002);
}

impl From<i32> for EmitterType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<EmitterType> for i32 {
    fn from(value: EmitterType) -> Self {
        value.0
    }
}
