use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct AnimationFlags: u32 {
        const PosFrames = 0x00000001;
    }
}
