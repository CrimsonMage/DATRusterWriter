use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::QualifiedControl::QualifiedControl,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CInputMap {
    pub mappings: Vec<QualifiedControl>,
}

impl IUnpackable for CInputMap {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.mappings.clear();
        self.mappings.reserve(count);
        for _ in 0..count {
            self.mappings.push(reader.read_item::<QualifiedControl>());
        }
        true
    }
}

impl IPackable for CInputMap {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.mappings.len() as u32);
        for item in &self.mappings {
            writer.write_item(item);
        }
        true
    }
}
