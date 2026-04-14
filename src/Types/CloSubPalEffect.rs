use crate::{
    DBObjs::RenderSurface::RenderSurface,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{CloSubPalette::CloSubPalette, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CloSubPalEffect {
    pub icon: QualifiedDataId<RenderSurface>,
    pub clo_sub_palettes: Vec<CloSubPalette>,
}

impl IUnpackable for CloSubPalEffect {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.icon = reader.read_item::<QualifiedDataId<RenderSurface>>();
        let count = reader.read_u32() as usize;
        self.clo_sub_palettes.clear();
        for _ in 0..count {
            self.clo_sub_palettes
                .push(reader.read_item::<CloSubPalette>());
        }
        true
    }
}

impl IPackable for CloSubPalEffect {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.icon);
        writer.write_u32(self.clo_sub_palettes.len() as u32);
        for item in &self.clo_sub_palettes {
            writer.write_item(item);
        }
        true
    }
}
