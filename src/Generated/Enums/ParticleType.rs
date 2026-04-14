#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ParticleType(pub i32);

impl ParticleType {
    pub const Unknown: Self = Self(0x00000000);
    pub const Still: Self = Self(0x00000001);
    pub const LocalVelocity: Self = Self(0x00000002);
    pub const ParabolicLVGA: Self = Self(0x00000003);
    pub const ParabolicLVGAGR: Self = Self(0x00000004);
    pub const Swarm: Self = Self(0x00000005);
    pub const Explode: Self = Self(0x00000006);
    pub const Implode: Self = Self(0x00000007);
    pub const ParabolicLVLA: Self = Self(0x00000008);
    pub const ParabolicLVLALR: Self = Self(0x00000009);
    pub const ParabolicGVGA: Self = Self(0x0000000A);
    pub const ParabolicGVGAGR: Self = Self(0x0000000B);
    pub const GlobalVelocity: Self = Self(0x0000000C);
    pub const NumParticleType: Self = Self(0x0000000D);
}

impl From<i32> for ParticleType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<ParticleType> for i32 {
    fn from(value: ParticleType) -> Self {
        value.0
    }
}
