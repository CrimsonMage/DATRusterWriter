#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct PropertyInheritanceType(pub u8);

impl PropertyInheritanceType {
    pub const ClassOnly: Self = Self(0x0);
    pub const InstanceOnly: Self = Self(0x1);
    pub const Either: Self = Self(0x2);

    pub const CLASS_ONLY: Self = Self::ClassOnly;
    pub const INSTANCE_ONLY: Self = Self::InstanceOnly;
    pub const EITHER: Self = Self::Either;
}

impl From<u8> for PropertyInheritanceType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PropertyInheritanceType> for u8 {
    fn from(value: PropertyInheritanceType) -> Self {
        value.0
    }
}
