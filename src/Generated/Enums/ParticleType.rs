#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ParticleType(pub i32);

impl From<i32> for ParticleType {
    fn from(value: i32) -> Self { Self(value) }
}

impl From<ParticleType> for i32 {
    fn from(value: ParticleType) -> Self { value.0 }
}
