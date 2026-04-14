#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct NumberingType(pub u8);

impl NumberingType {
    pub const Undefined: Self = Self(0x00);
    pub const Normal: Self = Self(0x01);
    pub const Sequential: Self = Self(0x01);
    pub const Bitfield: Self = Self(0x02);
    pub const Bitfield32: Self = Self(0x03);
    pub const Bitfield64: Self = Self(0x04);

    pub const UNDEFINED: Self = Self::Undefined;
    pub const NORMAL: Self = Self::Normal;
    pub const SEQUENTIAL: Self = Self::Sequential;
    pub const BITFIELD: Self = Self::Bitfield;
    pub const BITFIELD32: Self = Self::Bitfield32;
    pub const BITFIELD64: Self = Self::Bitfield64;
}

impl From<u8> for NumberingType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<NumberingType> for u8 {
    fn from(value: NumberingType) -> Self {
        value.0
    }
}
