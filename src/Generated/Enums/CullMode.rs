#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CullMode(pub i32);

impl CullMode {
    pub const Landblock: Self = Self(0x00000000);
    pub const None: Self = Self(0x00000001);
    pub const Clockwise: Self = Self(0x00000002);
    pub const CounterClockwise: Self = Self(0x00000003);

    pub const LANDBLOCK: Self = Self::Landblock;
    pub const NONE: Self = Self::None;
    pub const CLOCKWISE: Self = Self::Clockwise;
    pub const COUNTER_CLOCKWISE: Self = Self::CounterClockwise;
}

impl From<i32> for CullMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<CullMode> for i32 {
    fn from(value: CullMode) -> Self {
        value.0
    }
}
