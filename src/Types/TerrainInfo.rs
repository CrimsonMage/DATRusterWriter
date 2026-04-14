use crate::{
    Generated::Enums::TerrainTextureType::TerrainTextureType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TerrainInfo {
    value: u16,
}

impl TerrainInfo {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn road(&self) -> u8 {
        (self.value & 0x0003) as u8
    }

    pub fn set_road(&mut self, road: u8) {
        self.value = (self.value & !0x0003) | (road as u16 & 0x0003);
    }

    pub fn terrain_type(&self) -> TerrainTextureType {
        TerrainTextureType(((self.value & 0x007C) >> 2) as i32)
    }

    pub fn set_terrain_type(&mut self, terrain_type: TerrainTextureType) {
        self.value = (self.value & !0x007C) | (((terrain_type.0 as u16) & 0x001F) << 2);
    }

    pub fn scenery(&self) -> u8 {
        ((self.value & 0xF800) >> 11) as u8
    }

    pub fn set_scenery(&mut self, scenery: u8) {
        self.value = (self.value & !0xF800) | (((scenery as u16) & 0x001F) << 11);
    }
}

impl From<u16> for TerrainInfo {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<TerrainInfo> for u16 {
    fn from(value: TerrainInfo) -> Self {
        value.value
    }
}

impl IUnpackable for TerrainInfo {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_u16();
        true
    }
}

impl IPackable for TerrainInfo {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.value);
        true
    }
}
