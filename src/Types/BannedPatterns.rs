use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{HashTable::HashTable, PStringBase::PStringBase},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BannedPatterns {
    pub patterns: HashTable<u32, PStringBase<u8>>,
}

impl IUnpackable for BannedPatterns {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.patterns = reader.read_item::<HashTable<u32, PStringBase<u8>>>();
        true
    }
}

impl IPackable for BannedPatterns {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.patterns);
        true
    }
}
