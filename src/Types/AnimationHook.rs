use crate::{
    DBObjs::{GfxObj::GfxObj, ParticleEmitter::ParticleEmitter, Wave::Wave},
    Generated::Enums::{
        AnimationHookDir::AnimationHookDir, AnimationHookType::AnimationHookType, Sound::Sound,
    },
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable, Numerics::Vector3,
    },
    Types::{
        AttackCone::AttackCone, Frame::Frame, PackedQualifiedDataId::PackedQualifiedDataId,
        QualifiedDataId::QualifiedDataId,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationHook {
    Sound {
        direction: AnimationHookDir,
        id: QualifiedDataId<Wave>,
    },
    SoundTable {
        direction: AnimationHookDir,
        sound_type: Sound,
    },
    Attack {
        direction: AnimationHookDir,
        attack_cone: AttackCone,
    },
    AnimationDone {
        direction: AnimationHookDir,
    },
    ReplaceObject {
        direction: AnimationHookDir,
        part_index: u16,
        part_id: PackedQualifiedDataId<GfxObj>,
    },
    Ethereal {
        direction: AnimationHookDir,
        ethereal: bool,
    },
    TransparentPart {
        direction: AnimationHookDir,
        part_index: u32,
        start: f32,
        end: f32,
        time: f32,
    },
    Luminous {
        direction: AnimationHookDir,
        start: f32,
        end: f32,
        time: f32,
    },
    LuminousPart {
        direction: AnimationHookDir,
        part_index: u32,
        start: f32,
        end: f32,
        time: f32,
    },
    Diffuse {
        direction: AnimationHookDir,
        start: f32,
        end: f32,
        time: f32,
    },
    DiffusePart {
        direction: AnimationHookDir,
        part_index: u32,
        start: f32,
        end: f32,
        time: f32,
    },
    Scale {
        direction: AnimationHookDir,
        end: f32,
        time: f32,
    },
    CreateParticle {
        direction: AnimationHookDir,
        emitter_info_id: QualifiedDataId<ParticleEmitter>,
        part_index: u32,
        offset: Frame,
        emitter_id: u32,
    },
    DestroyParticle {
        direction: AnimationHookDir,
        emitter_id: u32,
    },
    StopParticle {
        direction: AnimationHookDir,
        emitter_id: u32,
    },
    NoDraw {
        direction: AnimationHookDir,
        no_draw: bool,
    },
    DefaultScript {
        direction: AnimationHookDir,
    },
    DefaultScriptPart {
        direction: AnimationHookDir,
        part_index: u32,
    },
    CallPES {
        direction: AnimationHookDir,
        pes: u32,
        pause: f32,
    },
    Transparent {
        direction: AnimationHookDir,
        start: f32,
        end: f32,
        time: f32,
    },
    SoundTweaked {
        direction: AnimationHookDir,
        sound_id: QualifiedDataId<Wave>,
        priority: f32,
        probability: f32,
        volume: f32,
    },
    SetOmega {
        direction: AnimationHookDir,
        axis: Vector3,
    },
    TextureVelocity {
        direction: AnimationHookDir,
        u_speed: f32,
        v_speed: f32,
    },
    TextureVelocityPart {
        direction: AnimationHookDir,
        part_index: u32,
        u_speed: f32,
        v_speed: f32,
    },
    SetLight {
        direction: AnimationHookDir,
        lights_on: bool,
    },
    CreateBlockingParticle {
        direction: AnimationHookDir,
    },
    Unknown {
        hook_type: AnimationHookType,
        direction: AnimationHookDir,
        payload: Vec<u8>,
    },
}

impl Default for AnimationHook {
    fn default() -> Self {
        Self::AnimationDone {
            direction: AnimationHookDir::BOTH,
        }
    }
}

impl AnimationHook {
    pub fn hook_type(&self) -> AnimationHookType {
        match self {
            Self::Sound { .. } => AnimationHookType::SOUND,
            Self::SoundTable { .. } => AnimationHookType::SOUND_TABLE,
            Self::Attack { .. } => AnimationHookType::ATTACK,
            Self::AnimationDone { .. } => AnimationHookType::ANIMATION_DONE,
            Self::ReplaceObject { .. } => AnimationHookType::REPLACE_OBJECT,
            Self::Ethereal { .. } => AnimationHookType::ETHEREAL,
            Self::TransparentPart { .. } => AnimationHookType::TRANSPARENT_PART,
            Self::Luminous { .. } => AnimationHookType::LUMINOUS,
            Self::LuminousPart { .. } => AnimationHookType::LUMINOUS_PART,
            Self::Diffuse { .. } => AnimationHookType::DIFFUSE,
            Self::DiffusePart { .. } => AnimationHookType::DIFFUSE_PART,
            Self::Scale { .. } => AnimationHookType::SCALE,
            Self::CreateParticle { .. } => AnimationHookType::CREATE_PARTICLE,
            Self::DestroyParticle { .. } => AnimationHookType::DESTROY_PARTICLE,
            Self::StopParticle { .. } => AnimationHookType::STOP_PARTICLE,
            Self::NoDraw { .. } => AnimationHookType::NO_DRAW,
            Self::DefaultScript { .. } => AnimationHookType::DEFAULT_SCRIPT,
            Self::DefaultScriptPart { .. } => AnimationHookType::DEFAULT_SCRIPT_PART,
            Self::CallPES { .. } => AnimationHookType::CALL_PES,
            Self::Transparent { .. } => AnimationHookType::TRANSPARENT,
            Self::SoundTweaked { .. } => AnimationHookType::SOUND_TWEAKED,
            Self::SetOmega { .. } => AnimationHookType::SET_OMEGA,
            Self::TextureVelocity { .. } => AnimationHookType::TEXTURE_VELOCITY,
            Self::TextureVelocityPart { .. } => AnimationHookType::TEXTURE_VELOCITY_PART,
            Self::SetLight { .. } => AnimationHookType::SET_LIGHT,
            Self::CreateBlockingParticle { .. } => AnimationHookType::CREATE_BLOCKING_PARTICLE,
            Self::Unknown { hook_type, .. } => *hook_type,
        }
    }

    fn direction(&self) -> AnimationHookDir {
        match self {
            Self::Sound { direction, .. }
            | Self::SoundTable { direction, .. }
            | Self::Attack { direction, .. }
            | Self::AnimationDone { direction }
            | Self::ReplaceObject { direction, .. }
            | Self::Ethereal { direction, .. }
            | Self::TransparentPart { direction, .. }
            | Self::Luminous { direction, .. }
            | Self::LuminousPart { direction, .. }
            | Self::Diffuse { direction, .. }
            | Self::DiffusePart { direction, .. }
            | Self::Scale { direction, .. }
            | Self::CreateParticle { direction, .. }
            | Self::DestroyParticle { direction, .. }
            | Self::StopParticle { direction, .. }
            | Self::NoDraw { direction, .. }
            | Self::DefaultScript { direction }
            | Self::DefaultScriptPart { direction, .. }
            | Self::CallPES { direction, .. }
            | Self::Transparent { direction, .. }
            | Self::SoundTweaked { direction, .. }
            | Self::SetOmega { direction, .. }
            | Self::TextureVelocity { direction, .. }
            | Self::TextureVelocityPart { direction, .. }
            | Self::SetLight { direction, .. }
            | Self::CreateBlockingParticle { direction }
            | Self::Unknown { direction, .. } => *direction,
        }
    }
}

impl IUnpackable for AnimationHook {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let hook_type = AnimationHookType::from(reader.read_u32());
        let direction = AnimationHookDir::from(reader.read_u32());
        *self = match hook_type {
            t if t == AnimationHookType::SOUND => Self::Sound {
                direction,
                id: reader.read_item::<QualifiedDataId<Wave>>(),
            },
            t if t == AnimationHookType::SOUND_TABLE => Self::SoundTable {
                direction,
                sound_type: Sound::from(reader.read_u32()),
            },
            t if t == AnimationHookType::ATTACK => Self::Attack {
                direction,
                attack_cone: reader.read_item::<AttackCone>(),
            },
            t if t == AnimationHookType::ANIMATION_DONE => Self::AnimationDone { direction },
            t if t == AnimationHookType::REPLACE_OBJECT => Self::ReplaceObject {
                direction,
                part_index: reader.read_u16(),
                part_id: reader.read_item::<PackedQualifiedDataId<GfxObj>>(),
            },
            t if t == AnimationHookType::ETHEREAL => Self::Ethereal {
                direction,
                ethereal: reader.read_bool(4),
            },
            t if t == AnimationHookType::TRANSPARENT_PART => Self::TransparentPart {
                direction,
                part_index: reader.read_u32(),
                start: reader.read_single(),
                end: reader.read_single(),
                time: reader.read_single(),
            },
            t if t == AnimationHookType::LUMINOUS => Self::Luminous {
                direction,
                start: reader.read_single(),
                end: reader.read_single(),
                time: reader.read_single(),
            },
            t if t == AnimationHookType::LUMINOUS_PART => Self::LuminousPart {
                direction,
                part_index: reader.read_u32(),
                start: reader.read_single(),
                end: reader.read_single(),
                time: reader.read_single(),
            },
            t if t == AnimationHookType::DIFFUSE => Self::Diffuse {
                direction,
                start: reader.read_single(),
                end: reader.read_single(),
                time: reader.read_single(),
            },
            t if t == AnimationHookType::DIFFUSE_PART => Self::DiffusePart {
                direction,
                part_index: reader.read_u32(),
                start: reader.read_single(),
                end: reader.read_single(),
                time: reader.read_single(),
            },
            t if t == AnimationHookType::SCALE => Self::Scale {
                direction,
                end: reader.read_single(),
                time: reader.read_single(),
            },
            t if t == AnimationHookType::CREATE_PARTICLE => Self::CreateParticle {
                direction,
                emitter_info_id: reader.read_item::<QualifiedDataId<ParticleEmitter>>(),
                part_index: reader.read_u32(),
                offset: reader.read_item::<Frame>(),
                emitter_id: reader.read_u32(),
            },
            t if t == AnimationHookType::DESTROY_PARTICLE => Self::DestroyParticle {
                direction,
                emitter_id: reader.read_u32(),
            },
            t if t == AnimationHookType::STOP_PARTICLE => Self::StopParticle {
                direction,
                emitter_id: reader.read_u32(),
            },
            t if t == AnimationHookType::NO_DRAW => Self::NoDraw {
                direction,
                no_draw: reader.read_bool(4),
            },
            t if t == AnimationHookType::DEFAULT_SCRIPT => Self::DefaultScript { direction },
            t if t == AnimationHookType::DEFAULT_SCRIPT_PART => Self::DefaultScriptPart {
                direction,
                part_index: reader.read_u32(),
            },
            t if t == AnimationHookType::CALL_PES => Self::CallPES {
                direction,
                pes: reader.read_u32(),
                pause: reader.read_single(),
            },
            t if t == AnimationHookType::TRANSPARENT => Self::Transparent {
                direction,
                start: reader.read_single(),
                end: reader.read_single(),
                time: reader.read_single(),
            },
            t if t == AnimationHookType::SOUND_TWEAKED => Self::SoundTweaked {
                direction,
                sound_id: reader.read_item::<QualifiedDataId<Wave>>(),
                priority: reader.read_single(),
                probability: reader.read_single(),
                volume: reader.read_single(),
            },
            t if t == AnimationHookType::SET_OMEGA => Self::SetOmega {
                direction,
                axis: reader.read_vector3(),
            },
            t if t == AnimationHookType::TEXTURE_VELOCITY => Self::TextureVelocity {
                direction,
                u_speed: reader.read_single(),
                v_speed: reader.read_single(),
            },
            t if t == AnimationHookType::TEXTURE_VELOCITY_PART => Self::TextureVelocityPart {
                direction,
                part_index: reader.read_u32(),
                u_speed: reader.read_single(),
                v_speed: reader.read_single(),
            },
            t if t == AnimationHookType::SET_LIGHT => Self::SetLight {
                direction,
                lights_on: reader.read_bool(4),
            },
            t if t == AnimationHookType::CREATE_BLOCKING_PARTICLE => {
                Self::CreateBlockingParticle { direction }
            }
            _ => Self::Unknown {
                hook_type,
                direction,
                payload: reader.read_remaining_bytes(),
            },
        };
        true
    }
}

impl IPackable for AnimationHook {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.hook_type().into());
        writer.write_u32(self.direction().into());
        match self {
            Self::Sound { id, .. } => writer.write_item(id),
            Self::SoundTable { sound_type, .. } => writer.write_u32((*sound_type).into()),
            Self::Attack { attack_cone, .. } => writer.write_item(attack_cone),
            Self::AnimationDone { .. } => {}
            Self::ReplaceObject {
                part_index,
                part_id,
                ..
            } => {
                writer.write_u16(*part_index);
                writer.write_item(part_id);
            }
            Self::Ethereal { ethereal, .. } => writer.write_bool(*ethereal, 4),
            Self::TransparentPart {
                part_index,
                start,
                end,
                time,
                ..
            }
            | Self::LuminousPart {
                part_index,
                start,
                end,
                time,
                ..
            }
            | Self::DiffusePart {
                part_index,
                start,
                end,
                time,
                ..
            } => {
                writer.write_u32(*part_index);
                writer.write_single(*start);
                writer.write_single(*end);
                writer.write_single(*time);
            }
            Self::Luminous {
                start, end, time, ..
            }
            | Self::Diffuse {
                start, end, time, ..
            }
            | Self::Transparent {
                start, end, time, ..
            } => {
                writer.write_single(*start);
                writer.write_single(*end);
                writer.write_single(*time);
            }
            Self::Scale { end, time, .. } => {
                writer.write_single(*end);
                writer.write_single(*time);
            }
            Self::CreateParticle {
                emitter_info_id,
                part_index,
                offset,
                emitter_id,
                ..
            } => {
                writer.write_item(emitter_info_id);
                writer.write_u32(*part_index);
                writer.write_item(offset);
                writer.write_u32(*emitter_id);
            }
            Self::DestroyParticle { emitter_id, .. } | Self::StopParticle { emitter_id, .. } => {
                writer.write_u32(*emitter_id)
            }
            Self::NoDraw { no_draw, .. } => writer.write_bool(*no_draw, 4),
            Self::DefaultScript { .. } => {}
            Self::DefaultScriptPart { part_index, .. } => writer.write_u32(*part_index),
            Self::CallPES { pes, pause, .. } => {
                writer.write_u32(*pes);
                writer.write_single(*pause);
            }
            Self::SoundTweaked {
                sound_id,
                priority,
                probability,
                volume,
                ..
            } => {
                writer.write_item(sound_id);
                writer.write_single(*priority);
                writer.write_single(*probability);
                writer.write_single(*volume);
            }
            Self::SetOmega { axis, .. } => writer.write_vector3(*axis),
            Self::TextureVelocity {
                u_speed, v_speed, ..
            } => {
                writer.write_single(*u_speed);
                writer.write_single(*v_speed);
            }
            Self::TextureVelocityPart {
                part_index,
                u_speed,
                v_speed,
                ..
            } => {
                writer.write_u32(*part_index);
                writer.write_single(*u_speed);
                writer.write_single(*v_speed);
            }
            Self::SetLight { lights_on, .. } => writer.write_bool(*lights_on, 4),
            Self::CreateBlockingParticle { .. } => {}
            Self::Unknown { payload, .. } => {
                for byte in payload {
                    writer.write_byte(*byte);
                }
            }
        }
        true
    }
}
