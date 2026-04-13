use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::{AC1LegacyString::AC1LegacyString, ColorARGB::ColorARGB};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TerrainType {
    pub terrain_name: AC1LegacyString,
    pub terrain_color: ColorARGB,
    pub scene_types: Vec<u32>,
}

impl IUnpackable for TerrainType {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.terrain_name = reader.read_item::<AC1LegacyString>();
        reader.align(4);
        self.terrain_color = reader.read_item::<ColorARGB>();
        let count = reader.read_u32() as usize;
        self.scene_types.clear();
        for _ in 0..count {
            self.scene_types.push(reader.read_u32());
        }
        true
    }
}

impl IPackable for TerrainType {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.terrain_name);
        writer.align(4);
        writer.write_item(&self.terrain_color);
        writer.write_u32(self.scene_types.len() as u32);
        for item in &self.scene_types {
            writer.write_u32(*item);
        }
        true
    }
}
