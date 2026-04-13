use crate::{
    DBObjs::SurfaceTexture::SurfaceTexture,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::QualifiedDataId::QualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CloTextureEffect {
    pub old_texture: QualifiedDataId<SurfaceTexture>,
    pub new_texture: QualifiedDataId<SurfaceTexture>,
}

impl IUnpackable for CloTextureEffect {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.old_texture = reader.read_item::<QualifiedDataId<SurfaceTexture>>();
        self.new_texture = reader.read_item::<QualifiedDataId<SurfaceTexture>>();
        true
    }
}

impl IPackable for CloTextureEffect {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.old_texture);
        writer.write_item(&self.new_texture);
        true
    }
}
