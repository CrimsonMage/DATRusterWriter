use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::AnimationHook::AnimationHook,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PhysicsScriptData {
    pub start_time: f64,
    pub hook: AnimationHook,
}

impl IUnpackable for PhysicsScriptData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.start_time = reader.read_double();
        self.hook = reader.read_item::<AnimationHook>();
        true
    }
}

impl IPackable for PhysicsScriptData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_double(self.start_time);
        writer.write_item(&self.hook);
        true
    }
}
