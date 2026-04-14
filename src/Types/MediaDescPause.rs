use crate::{
    Generated::Enums::MediaType::MediaType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MediaDescPause {
    pub ty: MediaType,
    pub min_duration: f32,
    pub max_duration: f32,
}

impl IUnpackable for MediaDescPause {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.min_duration = reader.read_single();
        self.max_duration = reader.read_single();
        true
    }
}

impl IPackable for MediaDescPause {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Pause, self.ty);
        writer.write_single(self.min_duration);
        writer.write_single(self.max_duration);
        true
    }
}
