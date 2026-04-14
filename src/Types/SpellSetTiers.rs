use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpellSetTiers {
    pub spells: Vec<u32>,
}

impl IUnpackable for SpellSetTiers {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_i32().max(0) as usize;
        self.spells = (0..count).map(|_| reader.read_u32()).collect();
        true
    }
}

impl IPackable for SpellSetTiers {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.spells.len() as i32);
        for spell in &self.spells {
            writer.write_u32(*spell);
        }
        true
    }
}
