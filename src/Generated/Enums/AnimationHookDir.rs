#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AnimationHookDir(pub u32);

impl AnimationHookDir {
    pub const BOTH: Self = Self(0x00000000);
    pub const FORWARD: Self = Self(0x00000001);
    pub const UNKNOWN: Self = Self(0xFFFFFFFE);
    pub const BACKWARD: Self = Self(0xFFFFFFFF);
}

impl From<u32> for AnimationHookDir {
    fn from(value: u32) -> Self { Self(value) }
}

impl From<AnimationHookDir> for u32 {
    fn from(value: AnimationHookDir) -> Self { value.0 }
}
