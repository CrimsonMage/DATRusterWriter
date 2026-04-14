use crate::{
    DBObjs::ParticleEmitter::ParticleEmitter,
    Generated::Enums::{AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{Frame::Frame, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateParticleHook {
    pub direction: AnimationHookDir,
    pub emitter_info_id: QualifiedDataId<ParticleEmitter>,
    pub part_index: u32,
    pub offset: Frame,
    pub emitter_id: u32,
}

impl CreateParticleHook {
    pub fn hook_type(&self) -> AnimationHookType {
        AnimationHookType::CREATE_PARTICLE
    }
}

impl IUnpackable for CreateParticleHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _hook_type = AnimationHookType::from(reader.read_u32());
        self.direction = AnimationHookDir::from(reader.read_u32());
        self.emitter_info_id = reader.read_item::<QualifiedDataId<ParticleEmitter>>();
        self.part_index = reader.read_u32();
        self.offset = reader.read_item::<Frame>();
        self.emitter_id = reader.read_u32();
        true
    }
}

impl IPackable for CreateParticleHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction.into());
        writer.write_item(&self.emitter_info_id);
        writer.write_u32(self.part_index);
        writer.write_item(&self.offset);
        writer.write_u32(self.emitter_id);
        true
    }
}
