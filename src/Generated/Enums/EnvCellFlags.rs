use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct EnvCellFlags: i32 {
        const SeenOutside = 0x00000001;
        const HasStaticObjs = 0x00000002;
        const HasRestrictionObj = 0x00000008;
    }
}
