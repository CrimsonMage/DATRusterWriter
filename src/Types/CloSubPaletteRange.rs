use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CloSubPaletteRange {
    pub offset: u32,
    pub num_colors: u32,
}

impl IUnpackable for CloSubPaletteRange {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.offset = reader.read_u32();
        self.num_colors = reader.read_u32();
        true
    }
}

impl IPackable for CloSubPaletteRange {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.offset);
        writer.write_u32(self.num_colors);
        true
    }
}
