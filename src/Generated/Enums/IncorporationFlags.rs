use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct IncorporationFlags: u32 {
        const None = 0x0000_0000;
        const PassToChildren = 0x0000_0001;
        const X = 0x0000_0002;
        const Y = 0x0000_0004;
        const Width = 0x0000_0008;
        const Height = 0x0000_0010;
        const ZLevel = 0x0000_0020;
    }
}
