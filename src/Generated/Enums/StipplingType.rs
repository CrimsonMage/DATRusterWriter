use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct StipplingType: u8 {
        const None = 0x00;
        const Positive = 0x01;
        const Negative = 0x02;
        const Both = 0x03;
        const NoPos = 0x04;
        const NoNeg = 0x08;
        const NoUVS = 0x14;
    }
}
