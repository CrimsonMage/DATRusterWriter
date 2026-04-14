use crate::{
    Generated::Enums::{MediaType::MediaType, UIStateId::UIStateId},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MediaDescState {
    pub ty: MediaType,
    pub state_id: UIStateId,
    pub probability: f32,
}

impl IUnpackable for MediaDescState {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.state_id = UIStateId::from(reader.read_u32());
        self.probability = reader.read_single();
        true
    }
}

impl IPackable for MediaDescState {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::State, self.ty);
        writer.write_u32(self.state_id.into());
        writer.write_single(self.probability);
        true
    }
}
