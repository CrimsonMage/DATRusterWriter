use crate::{
    DBObjs::RenderSurface::RenderSurface,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{ObjDesc::ObjDesc, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HairStyleCG {
    pub icon_id: QualifiedDataId<RenderSurface>,
    pub bald: bool,
    pub alternate_setup: u32,
    pub obj_desc: ObjDesc,
}

impl IUnpackable for HairStyleCG {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.icon_id = reader.read_item::<QualifiedDataId<RenderSurface>>();
        self.bald = reader.read_bool(1);
        self.alternate_setup = reader.read_u32();
        self.obj_desc = reader.read_item::<ObjDesc>();
        true
    }
}

impl IPackable for HairStyleCG {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.icon_id);
        writer.write_bool(self.bald, 1);
        writer.write_u32(self.alternate_setup);
        writer.write_item(&self.obj_desc);
        true
    }
}
