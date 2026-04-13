#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Sound(pub u32);

impl From<u32> for Sound {
    fn from(value: u32) -> Self { Self(value) }
}

impl From<Sound> for u32 {
    fn from(value: Sound) -> Self { value.0 }
}
