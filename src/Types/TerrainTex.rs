use crate::DBObjs::SurfaceTexture::SurfaceTexture;
use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::QualifiedDataId::QualifiedDataId;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TerrainTex {
    pub texture_id: QualifiedDataId<SurfaceTexture>,
    pub tex_tiling: u32,
    pub max_vert_bright: u32,
    pub min_vert_bright: u32,
    pub max_vert_saturate: u32,
    pub min_vert_saturate: u32,
    pub max_vert_hue: u32,
    pub min_vert_hue: u32,
    pub detail_tex_tiling: u32,
    pub detail_texture_id: QualifiedDataId<SurfaceTexture>,
}

impl IUnpackable for TerrainTex {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.texture_id = reader.read_item::<QualifiedDataId<SurfaceTexture>>();
        self.tex_tiling = reader.read_u32();
        self.max_vert_bright = reader.read_u32();
        self.min_vert_bright = reader.read_u32();
        self.max_vert_saturate = reader.read_u32();
        self.min_vert_saturate = reader.read_u32();
        self.max_vert_hue = reader.read_u32();
        self.min_vert_hue = reader.read_u32();
        self.detail_tex_tiling = reader.read_u32();
        self.detail_texture_id = reader.read_item::<QualifiedDataId<SurfaceTexture>>();
        true
    }
}

impl IPackable for TerrainTex {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.texture_id);
        writer.write_u32(self.tex_tiling);
        writer.write_u32(self.max_vert_bright);
        writer.write_u32(self.min_vert_bright);
        writer.write_u32(self.max_vert_saturate);
        writer.write_u32(self.min_vert_saturate);
        writer.write_u32(self.max_vert_hue);
        writer.write_u32(self.min_vert_hue);
        writer.write_u32(self.detail_tex_tiling);
        writer.write_item(&self.detail_texture_id);
        true
    }
}
