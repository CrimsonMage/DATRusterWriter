use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::AC1LegacyPStringBase::AC1LegacyPStringBase,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChatEmoteData {
    pub my_emote: AC1LegacyPStringBase<u8>,
    pub other_emote: AC1LegacyPStringBase<u8>,
}

impl IUnpackable for ChatEmoteData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.my_emote = reader.read_item::<AC1LegacyPStringBase<u8>>();
        self.other_emote = reader.read_item::<AC1LegacyPStringBase<u8>>();
        true
    }
}

impl IPackable for ChatEmoteData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.my_emote);
        writer.write_item(&self.other_emote);
        true
    }
}
