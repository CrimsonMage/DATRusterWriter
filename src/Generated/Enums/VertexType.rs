#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VertexType(pub i32);

impl VertexType {
    pub const UnknownVertexType: Self = Self(0x00000000);
    pub const CSWVertexType: Self = Self(0x00000001);
}

impl From<i32> for VertexType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<VertexType> for i32 {
    fn from(value: VertexType) -> Self {
        value.0
    }
}
