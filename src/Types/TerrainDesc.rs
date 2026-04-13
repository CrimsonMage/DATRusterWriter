use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::{LandSurf::LandSurf, TerrainType::TerrainType};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TerrainDesc {
    pub terrain_types: Vec<TerrainType>,
    pub land_surfaces: LandSurf,
}

impl IUnpackable for TerrainDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.terrain_types.clear();
        for _ in 0..count {
            self.terrain_types.push(reader.read_item::<TerrainType>());
        }
        self.land_surfaces = reader.read_item::<LandSurf>();
        true
    }
}

impl IPackable for TerrainDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.terrain_types.len() as u32);
        for item in &self.terrain_types {
            writer.write_item(item);
        }
        writer.write_item(&self.land_surfaces);
        true
    }
}
