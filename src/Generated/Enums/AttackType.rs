use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct AttackType: i32 {
        const Undef = 0x00000000;
        const Punch = 0x00000001;
        const Thrust = 0x00000002;
        const Slash = 0x00000004;
        const Kick = 0x00000008;
        const OffhandPunch = 0x00000010;
        const Punches = 0x00000011;
        const Unarmed = 0x00000019;
        const DoubleSlash = 0x00000020;
        const TripleSlash = 0x00000040;
        const DoubleThrust = 0x00000080;
        const TripleThrust = 0x00000100;
        const OffhandThrust = 0x00000200;
        const OffhandSlash = 0x00000400;
        const OffhandDoubleSlash = 0x00000800;
        const OffhandTripleSlash = 0x00001000;
        const Slashes = 0x00001C64;
        const OffhandDoubleThrust = 0x00002000;
        const DoubleStrike = 0x000028A0;
        const OffhandTripleThrust = 0x00004000;
        const TripleStrike = 0x00005140;
        const Thrusts = 0x00006382;
        const MultiStrike = 0x000079E0;
        const Offhand = 0x00007E00;
    }
}
