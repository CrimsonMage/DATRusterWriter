#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct RMDataType(pub u16);

impl RMDataType {
    pub const WaveForm: Self = Self(0x03E8);
    pub const Color: Self = Self(0x07D0);
    pub const Texture: Self = Self(0x0BB8);
    pub const Bool: Self = Self(0x0FA0);
    pub const TexturePtr: Self = Self(0x2710);

    pub const WAVE_FORM: Self = Self::WaveForm;
    pub const COLOR: Self = Self::Color;
    pub const TEXTURE: Self = Self::Texture;
    pub const BOOL: Self = Self::Bool;
    pub const TEXTURE_PTR: Self = Self::TexturePtr;
}

impl From<u16> for RMDataType {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<RMDataType> for u16 {
    fn from(value: RMDataType) -> Self {
        value.0
    }
}
