#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct TextureType(pub u8);

impl TextureType {
    pub const UNDEFINED: Self = Self(0x1);
    pub const TEXTURE2D: Self = Self(0x2);
    pub const TEXTURE3D: Self = Self(0x3);
    pub const CUBE: Self = Self(0x4);
    pub const MOVIE2D: Self = Self(0x5);
}

impl From<u8> for TextureType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<TextureType> for u8 {
    fn from(value: TextureType) -> Self {
        value.0
    }
}
