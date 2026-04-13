use crate::DBObjs::GfxObj::GfxObj;
use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::QualifiedDataId::QualifiedDataId;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkyObjectReplace {
    pub object_index: u32,
    pub gfx_obj_id: QualifiedDataId<GfxObj>,
    pub rotate: f32,
    pub transparent: f32,
    pub luminosity: f32,
    pub max_bright: f32,
}

impl IUnpackable for SkyObjectReplace {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.object_index = reader.read_u32();
        self.gfx_obj_id = reader.read_item::<QualifiedDataId<GfxObj>>();
        self.rotate = reader.read_single();
        self.transparent = reader.read_single();
        self.luminosity = reader.read_single();
        self.max_bright = reader.read_single();
        true
    }
}

impl IPackable for SkyObjectReplace {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.object_index);
        writer.write_item(&self.gfx_obj_id);
        writer.write_single(self.rotate);
        writer.write_single(self.transparent);
        writer.write_single(self.luminosity);
        writer.write_single(self.max_bright);
        true
    }
}
