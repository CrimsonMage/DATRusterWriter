use crate::{
    DBObjs::GfxObj::GfxObj,
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::{CloTextureEffect::CloTextureEffect, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CloObjectEffect {
    pub index: u32,
    pub model_id: QualifiedDataId<GfxObj>,
    pub clo_texture_effects: Vec<CloTextureEffect>,
}

impl IUnpackable for CloObjectEffect {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.index = reader.read_u32();
        self.model_id = reader.read_item::<QualifiedDataId<GfxObj>>();
        let count = reader.read_u32() as usize;
        self.clo_texture_effects.clear();
        for _ in 0..count { self.clo_texture_effects.push(reader.read_item::<CloTextureEffect>()); }
        true
    }
}

impl IPackable for CloObjectEffect {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.index);
        writer.write_item(&self.model_id);
        writer.write_u32(self.clo_texture_effects.len() as u32);
        for item in &self.clo_texture_effects { writer.write_item(item); }
        true
    }
}
