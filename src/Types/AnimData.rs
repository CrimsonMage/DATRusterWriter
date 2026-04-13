use crate::{Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable}, Types::QualifiedDataId::QualifiedDataId};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimData {
    pub anim_id: QualifiedDataId<()>,
    pub low_frame: i32,
    pub high_frame: i32,
    pub framerate: f32,
}

impl IUnpackable for AnimData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.anim_id = reader.read_item::<QualifiedDataId<()>>();
        self.low_frame = reader.read_i32();
        self.high_frame = reader.read_i32();
        self.framerate = reader.read_single();
        true
    }
}

impl IPackable for AnimData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let _ = self.anim_id.pack(writer);
        writer.write_i32(self.low_frame);
        writer.write_i32(self.high_frame);
        writer.write_single(self.framerate);
        true
    }
}
