use crate::DBObjs::SurfaceTexture::SurfaceTexture;
use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::QualifiedDataId::QualifiedDataId;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RoadAlphaMap {
    pub r_code: u32,
    pub texture_id: QualifiedDataId<SurfaceTexture>,
}

impl IUnpackable for RoadAlphaMap {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.r_code = reader.read_u32();
        self.texture_id = reader.read_item::<QualifiedDataId<SurfaceTexture>>();
        true
    }
}

impl IPackable for RoadAlphaMap {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.r_code);
        writer.write_item(&self.texture_id);
        true
    }
}
