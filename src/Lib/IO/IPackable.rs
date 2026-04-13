use crate::Lib::IO::DatBinWriter::DatBinWriter;

pub trait IPackable {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool;
}
