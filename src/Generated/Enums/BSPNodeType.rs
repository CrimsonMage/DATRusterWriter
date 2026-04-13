#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct BSPNodeType(pub i32);

impl BSPNodeType {
    pub const LEAF: Self = Self(1279607110);
    pub const PORTAL: Self = Self(1347375700);
    pub const BPNN: Self = Self(0x42506E6E);
    pub const BPIN_LOWER: Self = Self(0x4250496E);
    pub const BpIN: Self = Self(0x4270494E);
    pub const BpnN: Self = Self(0x42706E4E);
    pub const BPIN: Self = Self(0x4250494E);
    pub const BPnN: Self = Self(0x42506E4E);
    pub const BPOL: Self = Self(0x42504F4C);
    pub const BPFL: Self = Self(0x4250464C);
}

impl From<i32> for BSPNodeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<BSPNodeType> for i32 {
    fn from(value: BSPNodeType) -> Self {
        value.0
    }
}
