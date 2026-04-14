#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct PatchFlags(pub i32);

impl PatchFlags {
    pub const EmapperId: Self = Self(3);

    pub const EMAPPER_ID: Self = Self::EmapperId;
}

impl From<i32> for PatchFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<PatchFlags> for i32 {
    fn from(value: PatchFlags) -> Self {
        value.0
    }
}
