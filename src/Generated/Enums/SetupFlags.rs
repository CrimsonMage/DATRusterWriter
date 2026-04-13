use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct SetupFlags: u32 {
        const HasParent = 0x00000001;
        const HasDefaultScale = 0x00000002;
        const AllowFreeHeading = 0x00000004;
        const HasPhysicsBSP = 0x00000008;
    }
}
