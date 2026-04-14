use crate::{
    Generated::Enums::MediaType::MediaType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{
        MediaDescAlpha::MediaDescAlpha, MediaDescAnimation::MediaDescAnimation,
        MediaDescCursor::MediaDescCursor, MediaDescFade::MediaDescFade,
        MediaDescImage::MediaDescImage, MediaDescJump::MediaDescJump,
        MediaDescMessage::MediaDescMessage, MediaDescMovie::MediaDescMovie,
        MediaDescPause::MediaDescPause, MediaDescSound::MediaDescSound,
        MediaDescState::MediaDescState,
    },
};

pub(crate) fn read_header(reader: &mut DatBinReader<'_>) -> (MediaType, MediaType) {
    (
        MediaType::from(reader.read_i32()),
        MediaType::from(reader.read_i32()),
    )
}

pub(crate) fn write_header(
    writer: &mut DatBinWriter<'_>,
    media_type: MediaType,
    ty: MediaType,
) {
    writer.write_i32(media_type.into());
    writer.write_i32(ty.into());
}

#[derive(Debug, Clone, PartialEq)]
pub enum MediaDesc {
    Movie(MediaDescMovie),
    Alpha(MediaDescAlpha),
    Animation(MediaDescAnimation),
    Cursor(MediaDescCursor),
    Image(MediaDescImage),
    Jump(MediaDescJump),
    Message(MediaDescMessage),
    Pause(MediaDescPause),
    Sound(MediaDescSound),
    State(MediaDescState),
    Fade(MediaDescFade),
}

impl Default for MediaDesc {
    fn default() -> Self {
        Self::Movie(MediaDescMovie::default())
    }
}

impl MediaDesc {
    pub fn media_type(&self) -> MediaType {
        match self {
            Self::Movie(_) => MediaType::Movie,
            Self::Alpha(_) => MediaType::Alpha,
            Self::Animation(_) => MediaType::Animation,
            Self::Cursor(_) => MediaType::Cursor,
            Self::Image(_) => MediaType::Image,
            Self::Jump(_) => MediaType::Jump,
            Self::Message(_) => MediaType::Message,
            Self::Pause(_) => MediaType::Pause,
            Self::Sound(_) => MediaType::Sound,
            Self::State(_) => MediaType::State,
            Self::Fade(_) => MediaType::Fade,
        }
    }
}

impl IUnpackable for MediaDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let media_type = MediaType::from(reader.read_i32());
        reader.rewind(4);

        *self = match media_type {
            t if t == MediaType::Movie => {
                let mut value = MediaDescMovie::default();
                let _ = value.unpack(reader);
                Self::Movie(value)
            }
            t if t == MediaType::Alpha => {
                let mut value = MediaDescAlpha::default();
                let _ = value.unpack(reader);
                Self::Alpha(value)
            }
            t if t == MediaType::Animation => {
                let mut value = MediaDescAnimation::default();
                let _ = value.unpack(reader);
                Self::Animation(value)
            }
            t if t == MediaType::Cursor => {
                let mut value = MediaDescCursor::default();
                let _ = value.unpack(reader);
                Self::Cursor(value)
            }
            t if t == MediaType::Image => {
                let mut value = MediaDescImage::default();
                let _ = value.unpack(reader);
                Self::Image(value)
            }
            t if t == MediaType::Jump => {
                let mut value = MediaDescJump::default();
                let _ = value.unpack(reader);
                Self::Jump(value)
            }
            t if t == MediaType::Message => {
                let mut value = MediaDescMessage::default();
                let _ = value.unpack(reader);
                Self::Message(value)
            }
            t if t == MediaType::Pause => {
                let mut value = MediaDescPause::default();
                let _ = value.unpack(reader);
                Self::Pause(value)
            }
            t if t == MediaType::Sound => {
                let mut value = MediaDescSound::default();
                let _ = value.unpack(reader);
                Self::Sound(value)
            }
            t if t == MediaType::State => {
                let mut value = MediaDescState::default();
                let _ = value.unpack(reader);
                Self::State(value)
            }
            t if t == MediaType::Fade => {
                let mut value = MediaDescFade::default();
                let _ = value.unpack(reader);
                Self::Fade(value)
            }
            _ => return false,
        };
        true
    }
}

impl IPackable for MediaDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        match self {
            Self::Movie(value) => value.pack(writer),
            Self::Alpha(value) => value.pack(writer),
            Self::Animation(value) => value.pack(writer),
            Self::Cursor(value) => value.pack(writer),
            Self::Image(value) => value.pack(writer),
            Self::Jump(value) => value.pack(writer),
            Self::Message(value) => value.pack(writer),
            Self::Pause(value) => value.pack(writer),
            Self::Sound(value) => value.pack(writer),
            Self::State(value) => value.pack(writer),
            Self::Fade(value) => value.pack(writer),
        }
    }
}
