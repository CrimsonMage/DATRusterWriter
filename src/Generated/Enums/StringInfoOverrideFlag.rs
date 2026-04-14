use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct StringInfoOverrideFlag: u8 {
        const None = 0x0;
        const Literal = 0x1;
        const AutoGen = 0x2;
    }
}
