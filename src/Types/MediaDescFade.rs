use crate::{
    Generated::Enums::MediaType::MediaType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MediaDescFade {
    pub ty: MediaType,
    pub start_alpha: f32,
    pub end_alpha: f32,
    pub duration: f32,
}

impl IUnpackable for MediaDescFade {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.start_alpha = reader.read_single();
        self.end_alpha = reader.read_single();
        self.duration = reader.read_single();
        true
    }
}

impl IPackable for MediaDescFade {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Fade, self.ty);
        writer.write_single(self.start_alpha);
        writer.write_single(self.end_alpha);
        writer.write_single(self.duration);
        true
    }
}
