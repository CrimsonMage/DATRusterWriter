use crate::{
    Generated::Enums::{MediaType::MediaType, Sound::Sound},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::MediaDesc::{read_header, write_header},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MediaDescSound {
    pub ty: MediaType,
    pub file: u32,
    pub sound: Sound,
}

impl IUnpackable for MediaDescSound {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.file = reader.read_u32();
        self.sound = Sound::from(reader.read_u32());
        true
    }
}

impl IPackable for MediaDescSound {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Sound, self.ty);
        writer.write_u32(self.file);
        writer.write_u32(self.sound.into());
        true
    }
}
