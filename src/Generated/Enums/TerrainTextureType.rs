#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct TerrainTextureType(pub i32);

impl From<i32> for TerrainTextureType {
    fn from(value: i32) -> Self { Self(value) }
}

impl From<TerrainTextureType> for i32 {
    fn from(value: TerrainTextureType) -> Self { value.0 }
}
