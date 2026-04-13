use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{AnimationHook::AnimationHook, Frame::Frame},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationFrame {
    pub frames: Vec<Frame>,
    pub hooks: Vec<AnimationHook>,
}

impl AnimationFrame {
    pub fn unpack_with_num_parts(&mut self, reader: &mut DatBinReader<'_>, num_parts: u32) -> bool {
        self.frames.clear();
        for _ in 0..num_parts {
            self.frames.push(reader.read_item::<Frame>());
        }

        let hook_count = reader.read_u32() as usize;
        self.hooks.clear();
        for _ in 0..hook_count {
            self.hooks.push(reader.read_item::<AnimationHook>());
        }
        true
    }
}

impl IUnpackable for AnimationFrame {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.unpack_with_num_parts(reader, 0)
    }
}

impl IPackable for AnimationFrame {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        for frame in &self.frames {
            writer.write_item(frame);
        }
        writer.write_u32(self.hooks.len() as u32);
        for hook in &self.hooks {
            writer.write_item(hook);
        }
        true
    }
}
