use crate::{
    Generated::Enums::MediaType::MediaType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MediaDescJump {
    pub ty: MediaType,
    pub jump_item_index: u32,
    pub probability: f32,
}

impl IUnpackable for MediaDescJump {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.jump_item_index = reader.read_u32();
        self.probability = reader.read_single();
        true
    }
}

impl IPackable for MediaDescJump {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Jump, self.ty);
        writer.write_u32(self.jump_item_index);
        writer.write_single(self.probability);
        true
    }
}
