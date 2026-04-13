use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LandDefs {
    pub num_block_length: i32,
    pub num_block_width: i32,
    pub square_length: f32,
    pub lblock_length: i32,
    pub vertex_per_cell: i32,
    pub max_obj_height: f32,
    pub sky_height: f32,
    pub road_width: f32,
    pub land_height_table: Vec<f32>,
}

impl IUnpackable for LandDefs {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.num_block_length = reader.read_i32();
        self.num_block_width = reader.read_i32();
        self.square_length = reader.read_single();
        self.lblock_length = reader.read_i32();
        self.vertex_per_cell = reader.read_i32();
        self.max_obj_height = reader.read_single();
        self.sky_height = reader.read_single();
        self.road_width = reader.read_single();
        self.land_height_table.clear();
        for _ in 0..256 {
            self.land_height_table.push(reader.read_single());
        }
        true
    }
}

impl IPackable for LandDefs {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.num_block_length);
        writer.write_i32(self.num_block_width);
        writer.write_single(self.square_length);
        writer.write_i32(self.lblock_length);
        writer.write_i32(self.vertex_per_cell);
        writer.write_single(self.max_obj_height);
        writer.write_single(self.sky_height);
        writer.write_single(self.road_width);
        for value in &self.land_height_table {
            writer.write_single(*value);
        }
        true
    }
}
