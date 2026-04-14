use crate::{
    Generated::Enums::MediaType::MediaType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MediaDescMessage {
    pub ty: MediaType,
    pub id: u32,
    pub probability: f32,
}

impl IUnpackable for MediaDescMessage {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.id = reader.read_u32();
        self.probability = reader.read_single();
        true
    }
}

impl IPackable for MediaDescMessage {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Message, self.ty);
        writer.write_u32(self.id);
        writer.write_single(self.probability);
        true
    }
}
