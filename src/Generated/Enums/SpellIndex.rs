use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct SpellIndex: i32 {
        const Undef = 0x00000000;
        const Resistable = 0x00000001;
        const PKSensitive = 0x00000002;
        const Beneficial = 0x00000004;
        const SelfTargeted = 0x00000008;
        const Reversed = 0x00000010;
        const NotIndoor = 0x00000020;
        const NotOutdoor = 0x00000040;
        const NotResearchable = 0x00000080;
        const Projectile = 0x00000100;
        const CreatureSpell = 0x00000200;
        const ExcludedFromItemDescriptions = 0x00000400;
        const IgnoresManaConversion = 0x00000800;
        const NonTrackingProjectile = 0x00001000;
        const FellowshipSpell = 0x00002000;
        const FastCast = 0x00004000;
        const IndoorLongRange = 0x00008000;
        const DamageOverTime = 0x00010000;
    }
}
