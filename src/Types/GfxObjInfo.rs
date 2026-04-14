use crate::{
    DBObjs::GfxObj::GfxObj,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::QualifiedDataId::QualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GfxObjInfo {
    pub id: QualifiedDataId<GfxObj>,
    pub degrade_mode: u32,
    pub min_dist: f32,
    pub ideal_dist: f32,
    pub max_dist: f32,
}

impl IUnpackable for GfxObjInfo {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.id = reader.read_item::<QualifiedDataId<GfxObj>>();
        self.degrade_mode = reader.read_u32();
        self.min_dist = reader.read_single();
        self.ideal_dist = reader.read_single();
        self.max_dist = reader.read_single();
        true
    }
}

impl IPackable for GfxObjInfo {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.id);
        writer.write_u32(self.degrade_mode);
        writer.write_single(self.min_dist);
        writer.write_single(self.ideal_dist);
        writer.write_single(self.max_dist);
        true
    }
}
