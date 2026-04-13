use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct SurfaceType: u32 {
        const Base1Solid = 0x00000001;
        const Base1Image = 0x00000002;
        const Base1ClipMap = 0x00000004;
        const Translucent = 0x00000010;
        const Diffuse = 0x00000020;
        const Luminous = 0x00000040;
        const Alpha = 0x00000100;
        const InvAlpha = 0x00000200;
        const Additive = 0x00010000;
        const Detail = 0x00020000;
        const Gouraud = 0x10000000;
        const Stippled = 0x40000000;
        const Perspective = 0x80000000;
    }
}
