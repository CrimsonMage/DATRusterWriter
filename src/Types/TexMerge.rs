use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::{
    RoadAlphaMap::RoadAlphaMap, TMTerrainDesc::TMTerrainDesc, TerrainAlphaMap::TerrainAlphaMap,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TexMerge {
    pub base_tex_size: u32,
    pub corner_terrain_maps: Vec<TerrainAlphaMap>,
    pub side_terrain_maps: Vec<TerrainAlphaMap>,
    pub road_maps: Vec<RoadAlphaMap>,
    pub terrain_desc: Vec<TMTerrainDesc>,
}

impl IUnpackable for TexMerge {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.base_tex_size = reader.read_u32();

        let corner_count = reader.read_u32() as usize;
        self.corner_terrain_maps.clear();
        for _ in 0..corner_count {
            self.corner_terrain_maps
                .push(reader.read_item::<TerrainAlphaMap>());
        }

        let side_count = reader.read_u32() as usize;
        self.side_terrain_maps.clear();
        for _ in 0..side_count {
            self.side_terrain_maps
                .push(reader.read_item::<TerrainAlphaMap>());
        }

        let road_count = reader.read_u32() as usize;
        self.road_maps.clear();
        for _ in 0..road_count {
            self.road_maps.push(reader.read_item::<RoadAlphaMap>());
        }

        let terrain_count = reader.read_u32() as usize;
        self.terrain_desc.clear();
        for _ in 0..terrain_count {
            self.terrain_desc.push(reader.read_item::<TMTerrainDesc>());
        }
        true
    }
}

impl IPackable for TexMerge {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.base_tex_size);

        writer.write_u32(self.corner_terrain_maps.len() as u32);
        for item in &self.corner_terrain_maps {
            writer.write_item(item);
        }

        writer.write_u32(self.side_terrain_maps.len() as u32);
        for item in &self.side_terrain_maps {
            writer.write_item(item);
        }

        writer.write_u32(self.road_maps.len() as u32);
        for item in &self.road_maps {
            writer.write_item(item);
        }

        writer.write_u32(self.terrain_desc.len() as u32);
        for item in &self.terrain_desc {
            writer.write_item(item);
        }
        true
    }
}
