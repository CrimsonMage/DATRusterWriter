use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{ColorARGB::ColorARGB, Frame::Frame},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LightInfo {
    pub view_space_location: Frame,
    pub color: ColorARGB,
    pub intensity: f32,
    pub falloff: f32,
    pub cone_angle: f32,
}

impl IUnpackable for LightInfo {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.view_space_location = reader.read_item::<Frame>();
        self.color = reader.read_item::<ColorARGB>();
        self.intensity = reader.read_single();
        self.falloff = reader.read_single();
        self.cone_angle = reader.read_single();
        true
    }
}

impl IPackable for LightInfo {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.view_space_location);
        writer.write_item(&self.color);
        writer.write_single(self.intensity);
        writer.write_single(self.falloff);
        writer.write_single(self.cone_angle);
        true
    }
}
