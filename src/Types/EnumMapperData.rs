use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{AutoGrowHashTable::AutoGrowHashTable, PStringBase::PStringBase},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnumMapperData {
    pub base_enum_map: u32,
    pub unknown: u32,
    pub id_to_string_map: AutoGrowHashTable<u32, PStringBase<u8>>,
}

impl IUnpackable for EnumMapperData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.base_enum_map = reader.read_u32();
        self.unknown = reader.read_u32();
        self.id_to_string_map = reader.read_item::<AutoGrowHashTable<u32, PStringBase<u8>>>();
        true
    }
}

impl IPackable for EnumMapperData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.base_enum_map);
        writer.write_u32(self.unknown);
        writer.write_item(&self.id_to_string_map);
        true
    }
}
