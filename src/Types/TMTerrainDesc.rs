use crate::Generated::Enums::TerrainTextureType::TerrainTextureType;
use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::TerrainTex::TerrainTex;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TMTerrainDesc {
    pub terrain_type: TerrainTextureType,
    pub terrain_tex: TerrainTex,
}

impl IUnpackable for TMTerrainDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.terrain_type = reader.read_i32().into();
        self.terrain_tex = reader.read_item::<TerrainTex>();
        true
    }
}

impl IPackable for TMTerrainDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.terrain_type.into());
        writer.write_item(&self.terrain_tex);
        true
    }
}
