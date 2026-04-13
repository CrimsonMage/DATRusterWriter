#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EmitterType(pub i32);

impl From<i32> for EmitterType {
    fn from(value: i32) -> Self { Self(value) }
}

impl From<EmitterType> for i32 {
    fn from(value: EmitterType) -> Self { value.0 }
}
