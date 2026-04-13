use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::{ColorARGB::ColorARGB, SkyObjectReplace::SkyObjectReplace};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkyTimeOfDay {
    pub begin: f32,
    pub dir_bright: f32,
    pub dir_heading: f32,
    pub dir_pitch: f32,
    pub dir_color: ColorARGB,
    pub amb_bright: f32,
    pub amb_color: ColorARGB,
    pub min_world_fog: f32,
    pub max_world_fog: f32,
    pub world_fog_color: ColorARGB,
    pub world_fog: u32,
    pub sky_obj_replace: Vec<SkyObjectReplace>,
}

impl IUnpackable for SkyTimeOfDay {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.begin = reader.read_single();
        self.dir_bright = reader.read_single();
        self.dir_heading = reader.read_single();
        self.dir_pitch = reader.read_single();
        self.dir_color = reader.read_item::<ColorARGB>();
        self.amb_bright = reader.read_single();
        self.amb_color = reader.read_item::<ColorARGB>();
        self.min_world_fog = reader.read_single();
        self.max_world_fog = reader.read_single();
        self.world_fog_color = reader.read_item::<ColorARGB>();
        self.world_fog = reader.read_u32();
        let count = reader.read_u32() as usize;
        self.sky_obj_replace.clear();
        for _ in 0..count {
            self.sky_obj_replace.push(reader.read_item::<SkyObjectReplace>());
        }
        true
    }
}

impl IPackable for SkyTimeOfDay {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_single(self.begin);
        writer.write_single(self.dir_bright);
        writer.write_single(self.dir_heading);
        writer.write_single(self.dir_pitch);
        writer.write_item(&self.dir_color);
        writer.write_single(self.amb_bright);
        writer.write_item(&self.amb_color);
        writer.write_single(self.min_world_fog);
        writer.write_single(self.max_world_fog);
        writer.write_item(&self.world_fog_color);
        writer.write_u32(self.world_fog);
        writer.write_u32(self.sky_obj_replace.len() as u32);
        for item in &self.sky_obj_replace {
            writer.write_item(item);
        }
        true
    }
}
