#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct RenderPassType(pub u32);

impl RenderPassType {
    pub const Default: Self = Self(0x0);
    pub const DirectionalLightDiffuseAndSpecular: Self = Self(0x1);
    pub const PointLightDiffuseAndSpecular: Self = Self(0x2);
    pub const PointLightDiffuseAndSpecularProjector: Self = Self(0x3);
    pub const AmbientLight_DirectionalLightDiffuseAndSpecular: Self = Self(0x4);
    pub const AmbientLight_PointLightDiffuseAndSpecular: Self = Self(0x5);
    pub const DistanceFog: Self = Self(0x6);
    pub const FixedFunctionGlow: Self = Self(0x7);
    pub const ShaderGlow: Self = Self(0x8);
    pub const LandscapeShadowMap: Self = Self(0x9);
    pub const AlphaBlend: Self = Self(0xA);
    pub const AL_0DL_0PL: Self = Self(0xB);
    pub const AL_0DL_1PL: Self = Self(0xC);
    pub const AL_0DL_2PL: Self = Self(0xD);
    pub const AL_0DL_3PL: Self = Self(0xE);
    pub const AL_0DL_4PL: Self = Self(0xF);
    pub const AL_0DL_5PL: Self = Self(0x10);
    pub const AL_0DL_6PL: Self = Self(0x11);
    pub const AL_0DL_7PL: Self = Self(0x12);
    pub const AL_0DL_8PL: Self = Self(0x13);
    pub const AL_1DL_0PL: Self = Self(0x14);
    pub const AL_1DL_1PL: Self = Self(0x15);
    pub const AL_1DL_2PL: Self = Self(0x16);
    pub const AL_1DL_3PL: Self = Self(0x17);
    pub const AL_1DL_4PL: Self = Self(0x18);
    pub const AL_1DL_5PL: Self = Self(0x19);
    pub const AL_1DL_6PL: Self = Self(0x1A);
    pub const AL_1DL_7PL: Self = Self(0x1B);
    pub const AL_0DL_0PL_Fog: Self = Self(0x1C);
    pub const AL_0DL_1PL_Fog: Self = Self(0x1D);
    pub const AL_0DL_2PL_Fog: Self = Self(0x1E);
    pub const AL_0DL_3PL_Fog: Self = Self(0x1F);
    pub const AL_0DL_4PL_Fog: Self = Self(0x20);
    pub const AL_0DL_5PL_Fog: Self = Self(0x21);
    pub const AL_0DL_6PL_Fog: Self = Self(0x22);
    pub const AL_0DL_7PL_Fog: Self = Self(0x23);
    pub const AL_0DL_8PL_Fog: Self = Self(0x24);
    pub const AL_1DL_0PL_Fog: Self = Self(0x25);
    pub const AL_1DL_1PL_Fog: Self = Self(0x26);
    pub const AL_1DL_2PL_Fog: Self = Self(0x27);
    pub const AL_1DL_3PL_Fog: Self = Self(0x28);
    pub const AL_1DL_4PL_Fog: Self = Self(0x29);
    pub const AL_1DL_5PL_Fog: Self = Self(0x2A);
    pub const AL_1DL_6PL_Fog: Self = Self(0x2B);
    pub const AL_1DL_7PL_Fog: Self = Self(0x2C);
    pub const MaxPasses: Self = Self(0x2D);
    pub const Invalid: Self = Self(0x2E);

    pub const DEFAULT: Self = Self::Default;
    pub const DIRECTIONAL_LIGHT_DIFFUSE_AND_SPECULAR: Self =
        Self::DirectionalLightDiffuseAndSpecular;
    pub const POINT_LIGHT_DIFFUSE_AND_SPECULAR: Self = Self::PointLightDiffuseAndSpecular;
    pub const POINT_LIGHT_DIFFUSE_AND_SPECULAR_PROJECTOR: Self =
        Self::PointLightDiffuseAndSpecularProjector;
    pub const AMBIENT_LIGHT_DIRECTIONAL_LIGHT_DIFFUSE_AND_SPECULAR: Self =
        Self::AmbientLight_DirectionalLightDiffuseAndSpecular;
    pub const AMBIENT_LIGHT_POINT_LIGHT_DIFFUSE_AND_SPECULAR: Self =
        Self::AmbientLight_PointLightDiffuseAndSpecular;
    pub const DISTANCE_FOG: Self = Self::DistanceFog;
    pub const FIXED_FUNCTION_GLOW: Self = Self::FixedFunctionGlow;
    pub const SHADER_GLOW: Self = Self::ShaderGlow;
    pub const LANDSCAPE_SHADOW_MAP: Self = Self::LandscapeShadowMap;
    pub const ALPHA_BLEND: Self = Self::AlphaBlend;
    pub const MAX_PASSES: Self = Self::MaxPasses;
    pub const INVALID: Self = Self::Invalid;
}

impl From<u32> for RenderPassType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<RenderPassType> for u32 {
    fn from(value: RenderPassType) -> Self {
        value.0
    }
}
