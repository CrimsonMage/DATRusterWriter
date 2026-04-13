use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct GfxObjFlags: u32 {
        const HasPhysics = 0x00000001;
        const HasDrawing = 0x00000002;
        const Unknown = 0x00000004;
        const HasDIDDegrade = 0x00000008;
    }
}
