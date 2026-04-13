#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CullMode(pub i32);

impl CullMode {
    pub const CLOCKWISE: Self = Self(2);
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
