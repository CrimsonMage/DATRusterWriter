use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct DatBTreeFileFlags: u16 {
        const None = 0x00;
        const IsCompressed = 0x01;
    }
}
