use crate::{
    Generated::Enums::MediaType::MediaType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MediaDescAlpha {
    pub ty: MediaType,
    pub file: u32,
}

impl IUnpackable for MediaDescAlpha {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.file = reader.read_u32();
        true
    }
}

impl IPackable for MediaDescAlpha {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Alpha, self.ty);
        writer.write_u32(self.file);
        true
    }
}
