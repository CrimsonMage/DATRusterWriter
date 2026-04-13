use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::ScriptAndModData::ScriptAndModData,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsScriptTableData {
    pub scripts: Vec<ScriptAndModData>,
}

impl IUnpackable for PhysicsScriptTableData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.scripts.clear();
        for _ in 0..count {
            self.scripts.push(reader.read_item::<ScriptAndModData>());
        }
        true
    }
}

impl IPackable for PhysicsScriptTableData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.scripts.len() as u32);
        for script in &self.scripts {
            writer.write_item(script);
        }
        true
    }
}
