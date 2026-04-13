use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AttackCone {
    pub part_index: u32,
    pub left_x: f32,
    pub left_y: f32,
    pub right_x: f32,
    pub right_y: f32,
    pub radius: f32,
    pub height: f32,
}

impl IUnpackable for AttackCone {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.part_index = reader.read_u32();
        self.left_x = reader.read_single();
        self.left_y = reader.read_single();
        self.right_x = reader.read_single();
        self.right_y = reader.read_single();
        self.radius = reader.read_single();
        self.height = reader.read_single();
        true
    }
}

impl IPackable for AttackCone {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.part_index);
        writer.write_single(self.left_x);
        writer.write_single(self.left_y);
        writer.write_single(self.right_x);
        writer.write_single(self.right_y);
        writer.write_single(self.radius);
        writer.write_single(self.height);
        true
    }
}
