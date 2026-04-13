use crate::{
    DBObjs::RenderSurface::RenderSurface,
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::{ObjDesc::ObjDesc, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FaceStripCG {
    pub icon_id: QualifiedDataId<RenderSurface>,
    pub obj_desc: ObjDesc,
}

impl IUnpackable for FaceStripCG {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.icon_id = reader.read_item::<QualifiedDataId<RenderSurface>>();
        self.obj_desc = reader.read_item::<ObjDesc>();
        true
    }
}

impl IPackable for FaceStripCG {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.icon_id);
        writer.write_item(&self.obj_desc);
        true
    }
}
