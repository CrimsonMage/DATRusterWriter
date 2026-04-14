#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct AttackHeight(pub i32);

impl AttackHeight {
    pub const HIGH: Self = Self(0x00000001);
    pub const MEDIUM: Self = Self(0x00000002);
    pub const LOW: Self = Self(0x00000003);
}

impl From<i32> for AttackHeight {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl From<AttackHeight> for i32 {
    fn from(value: AttackHeight) -> Self {
        value.0
    }
}
