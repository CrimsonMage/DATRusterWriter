use crate::{
    DBObjs::GfxObj::GfxObj,
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::PackedQualifiedDataId::PackedQualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationPartChange {
    pub part_index: u8,
    pub part_id: PackedQualifiedDataId<GfxObj>,
}

impl IUnpackable for AnimationPartChange {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.part_index = reader.read_byte();
        self.part_id = reader.read_item::<PackedQualifiedDataId<GfxObj>>();
        true
    }
}

impl IPackable for AnimationPartChange {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.part_index);
        writer.write_item(&self.part_id);
        true
    }
}
