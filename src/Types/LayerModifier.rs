use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LayerModifier;

impl IUnpackable for LayerModifier {
    fn unpack(&mut self, _reader: &mut DatBinReader<'_>) -> bool {
        true
    }
}

impl IPackable for LayerModifier {
    fn pack(&self, _writer: &mut DatBinWriter<'_>) -> bool {
        true
    }
}
