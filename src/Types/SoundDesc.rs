use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};
use crate::Types::AmbientSTBDesc::AmbientSTBDesc;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SoundDesc {
    pub stb_desc: Vec<AmbientSTBDesc>,
}

impl IUnpackable for SoundDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.stb_desc.clear();
        for _ in 0..count {
            self.stb_desc.push(reader.read_item::<AmbientSTBDesc>());
        }
        true
    }
}

impl IPackable for SoundDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.stb_desc.len() as u32);
        for item in &self.stb_desc {
            writer.write_item(item);
        }
        true
    }
}
