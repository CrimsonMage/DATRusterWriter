use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextureVelocityPartHook {
    pub direction: AnimationHookDir,
    pub part_index: u32,
    pub u_speed: f32,
    pub v_speed: f32,
}

impl TextureVelocityPartHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::TEXTURE_VELOCITY_PART
    }
}

impl IUnpackable for TextureVelocityPartHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.part_index = reader.read_u32();
        self.u_speed = reader.read_single();
        self.v_speed = reader.read_single();
        true
    }
}

impl IPackable for TextureVelocityPartHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_u32(self.part_index);
        writer.write_single(self.u_speed);
        writer.write_single(self.v_speed);
        true
    }
}
