#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct AttributeId(pub u32);

impl AttributeId {
    pub const Strength: Self = Self(0x01);
    pub const Endurance: Self = Self(0x02);
    pub const Quickness: Self = Self(0x03);
    pub const Coordination: Self = Self(0x04);
    pub const Focus: Self = Self(0x05);
    pub const SelfValue: Self = Self(0x06);

    pub const STRENGTH: Self = Self::Strength;
    pub const ENDURANCE: Self = Self::Endurance;
    pub const QUICKNESS: Self = Self::Quickness;
    pub const COORDINATION: Self = Self::Coordination;
    pub const FOCUS: Self = Self::Focus;
    pub const SELF: Self = Self::SelfValue;
}

impl From<u32> for AttributeId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<AttributeId> for u32 {
    fn from(value: AttributeId) -> Self {
        value.0
    }
}
