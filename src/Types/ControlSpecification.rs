use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::HashTable::HashTableKey;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlSpecification {
    pub raw_key: u32,
}

impl IUnpackable for ControlSpecification {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.raw_key = reader.read_u32();
        true
    }
}

impl IPackable for ControlSpecification {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.raw_key);
        true
    }
}

impl HashTableKey for ControlSpecification {
    fn read_key(reader: &mut DatBinReader<'_>) -> Self {
        reader.read_item::<ControlSpecification>()
    }

    fn write_key(&self, writer: &mut DatBinWriter<'_>) {
        writer.write_item(self);
    }

    fn hash_key(&self) -> u64 {
        self.raw_key as u64
    }
}
