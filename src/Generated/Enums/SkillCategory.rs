#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SkillCategory(pub u32);

impl SkillCategory {
    pub const Undefined: Self = Self(0x00);
    pub const Combat: Self = Self(0x01);
    pub const Other: Self = Self(0x02);
    pub const Magic: Self = Self(0x03);

    pub const UNDEFINED: Self = Self::Undefined;
    pub const COMBAT: Self = Self::Combat;
    pub const OTHER: Self = Self::Other;
    pub const MAGIC: Self = Self::Magic;
}

impl From<u32> for SkillCategory {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<SkillCategory> for u32 {
    fn from(value: SkillCategory) -> Self {
        value.0
    }
}
