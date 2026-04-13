use crate::{
    Generated::Enums::{CullMode::CullMode, StipplingType::StipplingType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Polygon {
    pub stippling: StipplingType,
    pub sides_type: CullMode,
    pub pos_surface: i16,
    pub neg_surface: i16,
    pub vertex_ids: Vec<i16>,
    pub pos_uv_indices: Vec<u8>,
    pub neg_uv_indices: Vec<u8>,
}

impl IUnpackable for Polygon {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let num_vertices = reader.read_byte() as usize;
        self.stippling = StipplingType::from_bits_truncate(reader.read_byte());
        self.sides_type = reader.read_i32().into();
        self.pos_surface = reader.read_i16();
        self.neg_surface = reader.read_i16();
        self.vertex_ids.clear();
        self.pos_uv_indices.clear();
        self.neg_uv_indices.clear();
        for _ in 0..num_vertices {
            self.vertex_ids.push(reader.read_i16());
        }
        if !self.stippling.contains(StipplingType::NoPos) {
            for _ in 0..num_vertices {
                self.pos_uv_indices.push(reader.read_byte());
            }
        }
        if !self.stippling.contains(StipplingType::NoNeg) && self.sides_type == CullMode::CLOCKWISE
        {
            for _ in 0..num_vertices {
                self.neg_uv_indices.push(reader.read_byte());
            }
        }
        true
    }
}

impl IPackable for Polygon {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.vertex_ids.len() as u8);
        writer.write_byte(self.stippling.bits());
        writer.write_i32(self.sides_type.into());
        writer.write_i16(self.pos_surface);
        writer.write_i16(self.neg_surface);
        for id in &self.vertex_ids {
            writer.write_i16(*id);
        }
        if !self.stippling.contains(StipplingType::NoPos) {
            for uv in &self.pos_uv_indices {
                writer.write_byte(*uv);
            }
        }
        if !self.stippling.contains(StipplingType::NoNeg) && self.sides_type == CullMode::CLOCKWISE
        {
            for uv in &self.neg_uv_indices {
                writer.write_byte(*uv);
            }
        }
        true
    }
}
