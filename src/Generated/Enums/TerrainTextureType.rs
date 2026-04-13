#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct TerrainTextureType(pub i32);

impl TerrainTextureType {
    pub const BARREN_ROCK: Self = Self(0x00000000);
    pub const GRASSLAND: Self = Self(0x00000001);
    pub const ICE: Self = Self(0x00000002);
    pub const LUSH_GRASS: Self = Self(0x00000003);
    pub const MARSH_SPARSE_SWAMP: Self = Self(0x00000004);
    pub const MUD_RICH_DIRT: Self = Self(0x00000005);
    pub const OBSIDIAN_PLAIN: Self = Self(0x00000006);
    pub const PACKED_DIRT: Self = Self(0x00000007);
    pub const PATCHY_DIRT: Self = Self(0x00000008);
    pub const PATCHY_GRASSLAND: Self = Self(0x00000009);
    pub const SAND_YELLOW: Self = Self(0x0000000A);
    pub const SAND_GREY: Self = Self(0x0000000B);
    pub const SAND_ROCK_STREWN: Self = Self(0x0000000C);
    pub const SEDIMENTARY_ROCK: Self = Self(0x0000000D);
    pub const SEMI_BARREN_ROCK: Self = Self(0x0000000E);
    pub const SNOW: Self = Self(0x0000000F);
    pub const WATER_RUNNING: Self = Self(0x00000010);
    pub const WATER_STANDING_FRESH: Self = Self(0x00000011);
    pub const WATER_SHALLOW_SEA: Self = Self(0x00000012);
    pub const WATER_SHALLOW_STILL_SEA: Self = Self(0x00000013);
    pub const WATER_DEEP_SEA: Self = Self(0x00000014);
    pub const FOREST_FLOOR: Self = Self(0x00000015);
    pub const FAUX_WATER_RUNNING: Self = Self(0x00000016);
    pub const SEA_SLIME: Self = Self(0x00000017);
    pub const ARGILA: Self = Self(0x00000018);
    pub const VOLCANO1: Self = Self(0x00000019);
    pub const VOLCANO2: Self = Self(0x0000001A);
    pub const BLUE_ICE: Self = Self(0x0000001B);
    pub const MOSS: Self = Self(0x0000001C);
    pub const DARK_MOSS: Self = Self(0x0000001D);
    pub const OLTHOI: Self = Self(0x0000001E);
    pub const DESOLATE_LANDS: Self = Self(0x0000001F);
    pub const ROAD_TYPE: Self = Self(0x00000020);
}

impl From<i32> for TerrainTextureType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<TerrainTextureType> for i32 {
    fn from(value: TerrainTextureType) -> Self {
        value.0
    }
}
