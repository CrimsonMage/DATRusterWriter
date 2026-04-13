use crate::Lib::IO::DatBinReader::DatBinReader;

pub trait IUnpackable {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool;
}
