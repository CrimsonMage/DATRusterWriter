use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::TexMerge::TexMerge;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LandSurf {
    pub land_type: u32,
    pub tex_merge: TexMerge,
}

impl IUnpackable for LandSurf {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.land_type = reader.read_u32();
        self.tex_merge = reader.read_item::<TexMerge>();
        true
    }
}

impl IPackable for LandSurf {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.land_type);
        writer.write_item(&self.tex_merge);
        true
    }
}
