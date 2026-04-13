use crate::DBObjs::Scene::Scene;
use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::QualifiedDataId::QualifiedDataId;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneType {
    pub stb_index: u32,
    pub scenes: Vec<QualifiedDataId<Scene>>,
}

impl IUnpackable for SceneType {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.stb_index = reader.read_u32();
        let count = reader.read_u32() as usize;
        self.scenes.clear();
        for _ in 0..count {
            self.scenes
                .push(reader.read_item::<QualifiedDataId<Scene>>());
        }
        true
    }
}

impl IPackable for SceneType {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.stb_index);
        writer.write_u32(self.scenes.len() as u32);
        for item in &self.scenes {
            writer.write_item(item);
        }
        true
    }
}
