use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct DBObjHeaderFlags: u32 {
        const None = 0x00;
        const HasId = 0x01;
        const HasDataCategory = 0x02;
    }
}
