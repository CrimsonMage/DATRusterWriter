use crate::{
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::Frame::Frame,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ObjectDesc {
    pub object_id: u32,
    pub base_loc: Frame,
    pub frequency: f32,
    pub displace_x: f32,
    pub displace_y: f32,
    pub min_scale: f32,
    pub max_scale: f32,
    pub max_rotation: f32,
    pub min_slope: f32,
    pub max_slope: f32,
    pub align: i32,
    pub orient: i32,
    pub weenie_obj: u32,
}

impl IUnpackable for ObjectDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.object_id = reader.read_u32();
        self.base_loc = reader.read_item::<Frame>();
        self.frequency = reader.read_single();
        self.displace_x = reader.read_single();
        self.displace_y = reader.read_single();
        self.min_scale = reader.read_single();
        self.max_scale = reader.read_single();
        self.max_rotation = reader.read_single();
        self.min_slope = reader.read_single();
        self.max_slope = reader.read_single();
        self.align = reader.read_i32();
        self.orient = reader.read_i32();
        self.weenie_obj = reader.read_u32();
        true
    }
}

impl IPackable for ObjectDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.object_id);
        writer.write_item(&self.base_loc);
        writer.write_single(self.frequency);
        writer.write_single(self.displace_x);
        writer.write_single(self.displace_y);
        writer.write_single(self.min_scale);
        writer.write_single(self.max_scale);
        writer.write_single(self.max_rotation);
        writer.write_single(self.min_slope);
        writer.write_single(self.max_slope);
        writer.write_i32(self.align);
        writer.write_i32(self.orient);
        writer.write_u32(self.weenie_obj);
        true
    }
}
