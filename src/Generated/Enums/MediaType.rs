#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct MediaType(pub i32);

impl MediaType {
    pub const Undef: Self = Self(0x00000000);
    pub const Movie: Self = Self(0x00000001);
    pub const Alpha: Self = Self(0x00000002);
    pub const Animation: Self = Self(0x00000003);
    pub const Cursor: Self = Self(0x00000004);
    pub const Image: Self = Self(0x00000005);
    pub const Jump: Self = Self(0x00000006);
    pub const Message: Self = Self(0x00000007);
    pub const Pause: Self = Self(0x00000008);
    pub const Sound: Self = Self(0x00000009);
    pub const State: Self = Self(0x0000000A);
    pub const Fade: Self = Self(0x0000000B);
    pub const Stretch: Self = Self(0x0000000C);

    pub const UNDEF: Self = Self::Undef;
    pub const MOVIE: Self = Self::Movie;
    pub const ALPHA: Self = Self::Alpha;
    pub const ANIMATION: Self = Self::Animation;
    pub const CURSOR: Self = Self::Cursor;
    pub const IMAGE: Self = Self::Image;
    pub const JUMP: Self = Self::Jump;
    pub const MESSAGE: Self = Self::Message;
    pub const PAUSE: Self = Self::Pause;
    pub const SOUND: Self = Self::Sound;
    pub const STATE: Self = Self::State;
    pub const FADE: Self = Self::Fade;
    pub const STRETCH: Self = Self::Stretch;
}

impl From<i32> for MediaType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<MediaType> for i32 {
    fn from(value: MediaType) -> Self {
        value.0
    }
}

impl From<u32> for MediaType {
    fn from(value: u32) -> Self {
        Self(value as i32)
    }
}

impl From<MediaType> for u32 {
    fn from(value: MediaType) -> Self {
        value.0 as u32
    }
}
