use crate::{
    DBObjs::GfxObj::GfxObj,
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::PackedQualifiedDataId::PackedQualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplaceObjectHook {
    pub direction: AnimationHookDir,
    pub part_index: u16,
    pub part_id: PackedQualifiedDataId<GfxObj>,
}

impl ReplaceObjectHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::REPLACE_OBJECT
    }
}

impl IUnpackable for ReplaceObjectHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.part_index = reader.read_u16();
        self.part_id = reader.read_item::<PackedQualifiedDataId<GfxObj>>();
        true
    }
}

impl IPackable for ReplaceObjectHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_u16(self.part_index);
        writer.write_item(&self.part_id);
        true
    }
}
