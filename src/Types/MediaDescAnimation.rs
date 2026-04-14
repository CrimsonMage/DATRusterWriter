use crate::{
    Generated::Enums::{DrawModeType::DrawModeType, MediaType::MediaType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MediaDescAnimation {
    pub ty: MediaType,
    pub duration: f32,
    pub draw_mode: DrawModeType,
    pub frames: Vec<u32>,
}

impl IUnpackable for MediaDescAnimation {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.duration = reader.read_single();
        self.draw_mode = DrawModeType::from(reader.read_u32());
        let count = reader.read_u32() as usize;
        self.frames.clear();
        self.frames.reserve(count);
        for _ in 0..count {
            self.frames.push(reader.read_u32());
        }
        true
    }
}

impl IPackable for MediaDescAnimation {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Animation, self.ty);
        writer.write_single(self.duration);
        writer.write_u32(self.draw_mode.into());
        writer.write_u32(self.frames.len() as u32);
        for frame in &self.frames {
            writer.write_u32(*frame);
        }
        true
    }
}
