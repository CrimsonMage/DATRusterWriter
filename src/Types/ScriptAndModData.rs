use crate::{
    DBObjs::PhysicsScript::PhysicsScript,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::QualifiedDataId::QualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScriptAndModData {
    pub mod_value: f32,
    pub script_id: QualifiedDataId<PhysicsScript>,
}

impl IUnpackable for ScriptAndModData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.mod_value = reader.read_single();
        self.script_id = reader.read_item::<QualifiedDataId<PhysicsScript>>();
        true
    }
}

impl IPackable for ScriptAndModData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_single(self.mod_value);
        writer.write_item(&self.script_id);
        true
    }
}
