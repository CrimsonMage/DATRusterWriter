#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct PixelFormat(pub u32);

impl PixelFormat {
    pub const PFID_INDEX16: Self = Self(0x00000065);
    pub const PFID_P8: Self = Self(0x00000029);
}

impl From<u32> for PixelFormat {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<PixelFormat> for u32 {
    fn from(value: PixelFormat) -> Self {
        value.0
    }
}
