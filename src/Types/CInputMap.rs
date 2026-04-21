use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::InputMapBinding::InputMapBinding,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CInputMap {
    pub bindings: Vec<InputMapBinding>,
}

impl IUnpackable for CInputMap {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.bindings.clear();
        self.bindings.reserve(count);
        for _ in 0..count {
            self.bindings.push(reader.read_item::<InputMapBinding>());
        }
        true
    }
}

impl IPackable for CInputMap {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.bindings.len() as u32);
        for item in &self.bindings {
            writer.write_item(item);
        }
        true
    }
}
