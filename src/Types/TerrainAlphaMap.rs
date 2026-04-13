use crate::DBObjs::SurfaceTexture::SurfaceTexture;
use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::QualifiedDataId::QualifiedDataId;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TerrainAlphaMap {
    pub t_code: u32,
    pub texture_id: QualifiedDataId<SurfaceTexture>,
}

impl IUnpackable for TerrainAlphaMap {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.t_code = reader.read_u32();
        self.texture_id = reader.read_item::<QualifiedDataId<SurfaceTexture>>();
        true
    }
}

impl IPackable for TerrainAlphaMap {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.t_code);
        writer.write_item(&self.texture_id);
        true
    }
}
