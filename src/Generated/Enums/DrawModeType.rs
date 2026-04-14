#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct DrawModeType(pub u32);

impl DrawModeType {
    pub const Undefined: Self = Self(0x0);
    pub const Normal: Self = Self(0x1);
    pub const Overlay: Self = Self(0x2);
    pub const Alphablend: Self = Self(0x3);

    pub const UNDEFINED: Self = Self::Undefined;
    pub const NORMAL: Self = Self::Normal;
    pub const OVERLAY: Self = Self::Overlay;
    pub const ALPHABLEND: Self = Self::Alphablend;
}

impl From<u32> for DrawModeType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<DrawModeType> for u32 {
    fn from(value: DrawModeType) -> Self {
        value.0
    }
}
