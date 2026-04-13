#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AnimationHookType(pub u32);

impl AnimationHookType {
    pub const NO_OP: Self = Self(0x00000000);
    pub const SOUND: Self = Self(0x00000001);
    pub const SOUND_TABLE: Self = Self(0x00000002);
    pub const ATTACK: Self = Self(0x00000003);
    pub const ANIMATION_DONE: Self = Self(0x00000004);
    pub const REPLACE_OBJECT: Self = Self(0x00000005);
    pub const ETHEREAL: Self = Self(0x00000006);
    pub const TRANSPARENT_PART: Self = Self(0x00000007);
    pub const LUMINOUS: Self = Self(0x00000008);
    pub const LUMINOUS_PART: Self = Self(0x00000009);
    pub const DIFFUSE: Self = Self(0x0000000A);
    pub const DIFFUSE_PART: Self = Self(0x0000000B);
    pub const SCALE: Self = Self(0x0000000C);
    pub const CREATE_PARTICLE: Self = Self(0x0000000D);
    pub const DESTROY_PARTICLE: Self = Self(0x0000000E);
    pub const STOP_PARTICLE: Self = Self(0x0000000F);
    pub const NO_DRAW: Self = Self(0x00000010);
    pub const DEFAULT_SCRIPT: Self = Self(0x00000011);
    pub const DEFAULT_SCRIPT_PART: Self = Self(0x00000012);
    pub const CALL_PES: Self = Self(0x00000013);
    pub const TRANSPARENT: Self = Self(0x00000014);
    pub const SOUND_TWEAKED: Self = Self(0x00000015);
    pub const SET_OMEGA: Self = Self(0x00000016);
    pub const TEXTURE_VELOCITY: Self = Self(0x00000017);
    pub const TEXTURE_VELOCITY_PART: Self = Self(0x00000018);
    pub const SET_LIGHT: Self = Self(0x00000019);
    pub const CREATE_BLOCKING_PARTICLE: Self = Self(0x0000001A);
    pub const UNKNOWN: Self = Self(0xFFFFFFFF);
}

impl From<u32> for AnimationHookType {
    fn from(value: u32) -> Self { Self(value) }
}

impl From<AnimationHookType> for u32 {
    fn from(value: AnimationHookType) -> Self { value.0 }
}
