use crate::{
    DBObjs::SurfaceTexture::SurfaceTexture,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::PackedQualifiedDataId::PackedQualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TextureMapChange {
    pub part_index: u8,
    pub old_texture: PackedQualifiedDataId<SurfaceTexture>,
    pub new_texture: PackedQualifiedDataId<SurfaceTexture>,
}

impl IUnpackable for TextureMapChange {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.part_index = reader.read_byte();
        self.old_texture = reader.read_item::<PackedQualifiedDataId<SurfaceTexture>>();
        self.new_texture = reader.read_item::<PackedQualifiedDataId<SurfaceTexture>>();
        true
    }
}

impl IPackable for TextureMapChange {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.part_index);
        writer.write_item(&self.old_texture);
        writer.write_item(&self.new_texture);
        true
    }
}
