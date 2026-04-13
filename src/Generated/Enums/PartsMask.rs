use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct PartsMask: u32 {
        const HasSoundInfo = 0x01;
        const HasSceneInfo = 0x02;
        const HasSkyInfo = 0x10;
        const HasRegionMisc = 0x200;
    }
}
