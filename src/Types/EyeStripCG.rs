use crate::{
    DBObjs::RenderSurface::RenderSurface,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{ObjDesc::ObjDesc, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EyeStripCG {
    pub icon_id: QualifiedDataId<RenderSurface>,
    pub bald_icon_id: u32,
    pub obj_desc: ObjDesc,
    pub bald_obj_desc: ObjDesc,
}

impl IUnpackable for EyeStripCG {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.icon_id = reader.read_item::<QualifiedDataId<RenderSurface>>();
        self.bald_icon_id = reader.read_u32();
        self.obj_desc = reader.read_item::<ObjDesc>();
        self.bald_obj_desc = reader.read_item::<ObjDesc>();
        true
    }
}

impl IPackable for EyeStripCG {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.icon_id);
        writer.write_u32(self.bald_icon_id);
        writer.write_item(&self.obj_desc);
        writer.write_item(&self.bald_obj_desc);
        true
    }
}
