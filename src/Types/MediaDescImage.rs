use crate::{
    Generated::Enums::{DrawModeType::DrawModeType, MediaType::MediaType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MediaDescImage {
    pub ty: MediaType,
    pub file: u32,
    pub draw_mode: DrawModeType,
}

impl IUnpackable for MediaDescImage {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.file = reader.read_u32();
        self.draw_mode = DrawModeType::from(reader.read_u32());
        true
    }
}

impl IPackable for MediaDescImage {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Image, self.ty);
        writer.write_u32(self.file);
        writer.write_u32(self.draw_mode.into());
        true
    }
}
