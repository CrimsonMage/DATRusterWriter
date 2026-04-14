use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::PHashTable::PHashTable,
    Types::SpellSetTiers::SpellSetTiers,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpellSet {
    pub spell_set_tiers: PHashTable<u32, SpellSetTiers>,
}

impl IUnpackable for SpellSet {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.spell_set_tiers = reader.read_item::<PHashTable<u32, SpellSetTiers>>();
        true
    }
}

impl IPackable for SpellSet {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.spell_set_tiers);
        true
    }
}
