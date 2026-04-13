use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::SceneType::SceneType;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneDesc {
    pub scene_types: Vec<SceneType>,
}

impl IUnpackable for SceneDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.scene_types.clear();
        for _ in 0..count {
            self.scene_types.push(reader.read_item::<SceneType>());
        }
        true
    }
}

impl IPackable for SceneDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.scene_types.len() as u32);
        for item in &self.scene_types {
            writer.write_item(item);
        }
        true
    }
}
