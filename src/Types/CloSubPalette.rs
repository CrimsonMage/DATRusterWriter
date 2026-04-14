use crate::{
    DBObjs::PalSet::PalSet,
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::{CloSubPaletteRange::CloSubPaletteRange, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CloSubPalette {
    pub ranges: Vec<CloSubPaletteRange>,
    pub palette_set: QualifiedDataId<PalSet>,
}

impl IUnpackable for CloSubPalette {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.ranges.clear();
        for _ in 0..count { self.ranges.push(reader.read_item::<CloSubPaletteRange>()); }
        self.palette_set = reader.read_item::<QualifiedDataId<PalSet>>();
        true
    }
}

impl IPackable for CloSubPalette {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.ranges.len() as u32);
        for item in &self.ranges { writer.write_item(item); }
        writer.write_item(&self.palette_set);
        true
    }
}
