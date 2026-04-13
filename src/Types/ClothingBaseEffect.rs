use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::CloObjectEffect::CloObjectEffect,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClothingBaseEffect {
    pub clo_object_effects: Vec<CloObjectEffect>,
}

impl IUnpackable for ClothingBaseEffect {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.clo_object_effects.clear();
        for _ in 0..count {
            self.clo_object_effects
                .push(reader.read_item::<CloObjectEffect>());
        }
        true
    }
}

impl IPackable for ClothingBaseEffect {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.clo_object_effects.len() as u32);
        for item in &self.clo_object_effects {
            writer.write_item(item);
        }
        true
    }
}
