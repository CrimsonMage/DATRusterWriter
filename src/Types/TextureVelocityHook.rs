use crate::{
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextureVelocityHook {
    pub direction: AnimationHookDir,
    pub u_speed: f32,
    pub v_speed: f32,
}

impl TextureVelocityHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::TEXTURE_VELOCITY
    }
}

impl IUnpackable for TextureVelocityHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.u_speed = reader.read_single();
        self.v_speed = reader.read_single();
        true
    }
}

impl IPackable for TextureVelocityHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_single(self.u_speed);
        writer.write_single(self.v_speed);
        true
    }
}
