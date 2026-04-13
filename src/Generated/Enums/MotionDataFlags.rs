use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct MotionDataFlags: u8 {
        const None = 0x00;
        const HasVelocity = 0x01;
        const HasOmega = 0x02;
    }
}
