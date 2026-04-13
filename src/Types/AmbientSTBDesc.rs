use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};
use crate::Types::AmbientSoundDesc::AmbientSoundDesc;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AmbientSTBDesc {
    pub stb_id: u32,
    pub ambient_sounds: Vec<AmbientSoundDesc>,
}

impl IUnpackable for AmbientSTBDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.stb_id = reader.read_u32();
        let count = reader.read_u32() as usize;
        self.ambient_sounds.clear();
        for _ in 0..count {
            self.ambient_sounds.push(reader.read_item::<AmbientSoundDesc>());
        }
        true
    }
}

impl IPackable for AmbientSTBDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.stb_id);
        writer.write_u32(self.ambient_sounds.len() as u32);
        for item in &self.ambient_sounds {
            writer.write_item(item);
        }
        true
    }
}
