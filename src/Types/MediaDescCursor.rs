use crate::{
    Generated::Enums::MediaType::MediaType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MediaDescCursor {
    pub ty: MediaType,
    pub file: u32,
    pub x_hotspot: u32,
    pub y_hotspot: u32,
}

impl IUnpackable for MediaDescCursor {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.file = reader.read_u32();
        self.x_hotspot = reader.read_u32();
        self.y_hotspot = reader.read_u32();
        true
    }
}

impl IPackable for MediaDescCursor {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Cursor, self.ty);
        writer.write_u32(self.file);
        writer.write_u32(self.x_hotspot);
        writer.write_u32(self.y_hotspot);
        true
    }
}
