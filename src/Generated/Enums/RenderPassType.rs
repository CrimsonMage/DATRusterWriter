#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct RenderPassType(pub u32);

impl RenderPassType {
    pub const Default: Self = Self(0x00000000);
    pub const DirectionalLightDiffuseAndSpecular: Self = Self(0x00000001);
    pub const PointLightDiffuseAndSpecular: Self = Self(0x00000002);
    pub const PointLightDiffuseAndSpecularProjector: Self = Self(0x00000003);
    pub const AmbientLight_DirectionalLightDiffuseAndSpecular: Self = Self(0x00000004);
    pub const AmbientLight_PointLightDiffuseAndSpecular: Self = Self(0x00000005);
    pub const DistanceFog: Self = Self(0x00000006);
    pub const FixedFunctionGlow: Self = Self(0x00000007);
    pub const ShaderGlow: Self = Self(0x00000008);
    pub const LandscapeShadowMap: Self = Self(0x00000009);
    pub const AlphaBlend: Self = Self(0x0000000A);
    pub const MaxPasses: Self = Self(0x0000002D);
    pub const Invalid: Self = Self(0x0000002E);

    pub const DEFAULT: Self = Self::Default;
    pub const DIRECTIONAL_LIGHT_DIFFUSE_AND_SPECULAR: Self = Self::DirectionalLightDiffuseAndSpecular;
    pub const POINT_LIGHT_DIFFUSE_AND_SPECULAR: Self = Self::PointLightDiffuseAndSpecular;
    pub const POINT_LIGHT_DIFFUSE_AND_SPECULAR_PROJECTOR: Self = Self::PointLightDiffuseAndSpecularProjector;
    pub const AMBIENT_LIGHT_DIRECTIONAL_LIGHT_DIFFUSE_AND_SPECULAR: Self = Self::AmbientLight_DirectionalLightDiffuseAndSpecular;
    pub const AMBIENT_LIGHT_POINT_LIGHT_DIFFUSE_AND_SPECULAR: Self = Self::AmbientLight_PointLightDiffuseAndSpecular;
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
