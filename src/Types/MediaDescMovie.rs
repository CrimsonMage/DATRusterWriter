use crate::{
    Generated::Enums::MediaType::MediaType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{
        MediaDesc::{read_header, write_header},
        PStringBase::PStringBase,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MediaDescMovie {
    pub ty: MediaType,
    pub file_name: PStringBase<u8>,
    pub stretch_to_full_screen: bool,
}

impl IUnpackable for MediaDescMovie {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let (_, ty) = read_header(reader);
        self.ty = ty;
        self.file_name = reader.read_item::<PStringBase<u8>>();
        self.stretch_to_full_screen = reader.read_bool(1);
        true
    }
}

impl IPackable for MediaDescMovie {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        write_header(writer, MediaType::Movie, self.ty);
        writer.write_item(&self.file_name);
        writer.write_bool(self.stretch_to_full_screen, 1);
        true
    }
}
