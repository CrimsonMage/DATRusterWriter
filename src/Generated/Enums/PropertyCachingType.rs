#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct PropertyCachingType(pub u8);

impl PropertyCachingType {
    pub const Global: Self = Self(0x0);
    pub const Internal: Self = Self(0x1);

    pub const GLOBAL: Self = Self::Global;
    pub const INTERNAL: Self = Self::Internal;
}

impl From<u8> for PropertyCachingType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PropertyCachingType> for u8 {
    fn from(value: PropertyCachingType) -> Self {
        value.0
    }
}
