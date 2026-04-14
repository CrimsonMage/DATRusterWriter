use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct PortalFlags: u16 {
        const ExactMatch = 0x0001;
        const PortalSide = 0x0002;
    }
}
