#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct EquipmentSet(pub u32);

impl EquipmentSet {
    pub const Invalid: Self = Self(0x00000000);
    pub const Test: Self = Self(0x00000001);
    pub const Test2: Self = Self(0x00000002);
    pub const Unknown3: Self = Self(0x00000003);
    pub const CarraidasBenediction: Self = Self(0x00000004);
    pub const NobleRelic: Self = Self(0x00000005);
    pub const AncientRelic: Self = Self(0x00000006);
    pub const AlduressaRelic: Self = Self(0x00000007);
    pub const Ninja: Self = Self(0x00000008);
    pub const EmpyreanRings: Self = Self(0x00000009);
    pub const ArmMindHeart: Self = Self(0x0000000A);
    pub const ArmorPerfectLight: Self = Self(0x0000000B);
    pub const ArmorPerfectLight2: Self = Self(0x0000000C);
    pub const Soldiers: Self = Self(0x0000000D);
    pub const Adepts: Self = Self(0x0000000E);
    pub const Archers: Self = Self(0x0000000F);
    pub const Defenders: Self = Self(0x00000010);
    pub const Tinkers: Self = Self(0x00000011);
    pub const Crafters: Self = Self(0x00000012);
    pub const Hearty: Self = Self(0x00000013);
    pub const Dexterous: Self = Self(0x00000014);
    pub const Wise: Self = Self(0x00000015);
    pub const Swift: Self = Self(0x00000016);
    pub const Hardened: Self = Self(0x00000017);
    pub const Reinforced: Self = Self(0x00000018);
    pub const Interlocking: Self = Self(0x00000019);
    pub const Flameproof: Self = Self(0x0000001A);
    pub const Acidproof: Self = Self(0x0000001B);
    pub const Coldproof: Self = Self(0x0000001C);
    pub const Lightningproof: Self = Self(0x0000001D);
    pub const SocietyArmor: Self = Self(0x0000001E);
    pub const ColosseumClothing: Self = Self(0x0000001F);
    pub const GraveyardClothing: Self = Self(0x00000020);
    pub const OlthoiClothing: Self = Self(0x00000021);
    pub const NoobieArmor: Self = Self(0x00000022);
    pub const AetheriaDefense: Self = Self(0x00000023);
    pub const AetheriaDestruction: Self = Self(0x00000024);
    pub const AetheriaFury: Self = Self(0x00000025);
    pub const AetheriaGrowth: Self = Self(0x00000026);
    pub const AetheriaVigor: Self = Self(0x00000027);
    pub const RareDamageResistance: Self = Self(0x00000028);
    pub const RareDamageBoost: Self = Self(0x00000029);
    pub const OlthoiArmorDRed: Self = Self(0x0000002A);
    pub const OlthoiArmorCRat: Self = Self(0x0000002B);
    pub const OlthoiArmorCRed: Self = Self(0x0000002C);
    pub const OlthoiArmorDRat: Self = Self(0x0000002D);
    pub const AlduressaRelicUpgrade: Self = Self(0x0000002E);
    pub const AncientRelicUpgrade: Self = Self(0x0000002F);
    pub const NobleRelicUpgrade: Self = Self(0x00000030);
    pub const CloakAlchemy: Self = Self(0x00000031);
    pub const CloakArcaneLore: Self = Self(0x00000032);
    pub const CloakArmorTinkering: Self = Self(0x00000033);
    pub const CloakAssessPerson: Self = Self(0x00000034);
    pub const CloakLightWeapons: Self = Self(0x00000035);
    pub const CloakMissileWeapons: Self = Self(0x00000036);
    pub const CloakCooking: Self = Self(0x00000037);
    pub const CloakCreatureEnchantment: Self = Self(0x00000038);
    pub const CloakCrossbow: Self = Self(0x00000039);
    pub const CloakFinesseWeapons: Self = Self(0x0000003A);
    pub const CloakDeception: Self = Self(0x0000003B);
    pub const CloakFletching: Self = Self(0x0000003C);
    pub const CloakHealing: Self = Self(0x0000003D);
    pub const CloakItemEnchantment: Self = Self(0x0000003E);
    pub const CloakItemTinkering: Self = Self(0x0000003F);
    pub const CloakLeadership: Self = Self(0x00000040);
    pub const CloakLifeMagic: Self = Self(0x00000041);
    pub const CloakLoyalty: Self = Self(0x00000042);
    pub const CloakMace: Self = Self(0x00000043);
    pub const CloakMagicDefense: Self = Self(0x00000044);
    pub const CloakMagicItemTinkering: Self = Self(0x00000045);
    pub const CloakManaConversion: Self = Self(0x00000046);
    pub const CloakMeleeDefense: Self = Self(0x00000047);
    pub const CloakMissileDefense: Self = Self(0x00000048);
    pub const CloakSalvaging: Self = Self(0x00000049);
    pub const CloakSpear: Self = Self(0x0000004A);
    pub const CloakStaff: Self = Self(0x0000004B);
    pub const CloakHeavyWeapons: Self = Self(0x0000004C);
    pub const CloakThrownWeapon: Self = Self(0x0000004D);
    pub const CloakTwoHandedCombat: Self = Self(0x0000004E);
    pub const CloakUnarmedCombat: Self = Self(0x0000004F);
    pub const CloakVoidMagic: Self = Self(0x00000050);
    pub const CloakWarMagic: Self = Self(0x00000051);
    pub const CloakWeaponTinkering: Self = Self(0x00000052);
    pub const CloakAssessCreature: Self = Self(0x00000053);
    pub const CloakDirtyFighting: Self = Self(0x00000054);
    pub const CloakDualWield: Self = Self(0x00000055);
    pub const CloakRecklessness: Self = Self(0x00000056);
    pub const CloakShield: Self = Self(0x00000057);
    pub const CloakSneakAttack: Self = Self(0x00000058);
    pub const Ninja_New: Self = Self(0x00000059);
    pub const CloakSummoning: Self = Self(0x0000005A);
    pub const ShroudedSoul: Self = Self(0x0000005B);
    pub const DarkenedMind: Self = Self(0x0000005C);
    pub const CloudedSpirit: Self = Self(0x0000005D);
    pub const MinorStingingShroudedSoul: Self = Self(0x0000005E);
    pub const MinorSparkingShroudedSoul: Self = Self(0x0000005F);
    pub const MinorSmolderingShroudedSoul: Self = Self(0x00000060);
    pub const MinorShiveringShroudedSoul: Self = Self(0x00000061);
    pub const MinorStingingDarkenedMind: Self = Self(0x00000062);
    pub const MinorSparkingDarkenedMind: Self = Self(0x00000063);
    pub const MinorSmolderingDarkenedMind: Self = Self(0x00000064);
    pub const MinorShiveringDarkenedMind: Self = Self(0x00000065);
    pub const MinorStingingCloudedSpirit: Self = Self(0x00000066);
    pub const MinorSparkingCloudedSpirit: Self = Self(0x00000067);
    pub const MinorSmolderingCloudedSpirit: Self = Self(0x00000068);
    pub const MinorShiveringCloudedSpirit: Self = Self(0x00000069);
    pub const MajorStingingShroudedSoul: Self = Self(0x0000006A);
    pub const MajorSparkingShroudedSoul: Self = Self(0x0000006B);
    pub const MajorSmolderingShroudedSoul: Self = Self(0x0000006C);
    pub const MajorShiveringShroudedSoul: Self = Self(0x0000006D);
    pub const MajorStingingDarkenedMind: Self = Self(0x0000006E);
    pub const MajorSparkingDarkenedMind: Self = Self(0x0000006F);
    pub const MajorSmolderingDarkenedMind: Self = Self(0x00000070);
    pub const MajorShiveringDarkenedMind: Self = Self(0x00000071);
    pub const MajorStingingCloudedSpirit: Self = Self(0x00000072);
    pub const MajorSparkingCloudedSpirit: Self = Self(0x00000073);
    pub const MajorSmolderingCloudedSpirit: Self = Self(0x00000074);
    pub const MajorShiveringCloudedSpirit: Self = Self(0x00000075);
    pub const BlackfireStingingShroudedSoul: Self = Self(0x00000076);
    pub const BlackfireSparkingShroudedSoul: Self = Self(0x00000077);
    pub const BlackfireSmolderingShroudedSoul: Self = Self(0x00000078);
    pub const BlackfireShiveringShroudedSoul: Self = Self(0x00000079);
    pub const BlackfireStingingDarkenedMind: Self = Self(0x0000007A);
    pub const BlackfireSparkingDarkenedMind: Self = Self(0x0000007B);
    pub const BlackfireSmolderingDarkenedMind: Self = Self(0x0000007C);
    pub const BlackfireShiveringDarkenedMind: Self = Self(0x0000007D);
    pub const BlackfireStingingCloudedSpirit: Self = Self(0x0000007E);
    pub const BlackfireSparkingCloudedSpirit: Self = Self(0x0000007F);
    pub const BlackfireSmolderingCloudedSpirit: Self = Self(0x00000080);
    pub const BlackfireShiveringCloudedSpirit: Self = Self(0x00000081);
    pub const ShimmeringShadowsSet: Self = Self(0x00000082);
    pub const BrownSocietyLocket: Self = Self(0x00000083);
    pub const YellowSocietyLocket: Self = Self(0x00000084);
    pub const RedSocietyBand: Self = Self(0x00000085);
    pub const GreenSocietyBand: Self = Self(0x00000086);
    pub const PurpleSocietyBand: Self = Self(0x00000087);
    pub const BlueSocietyBand: Self = Self(0x00000088);
    pub const GauntletGarb: Self = Self(0x00000089);
    pub const ParagonMissile: Self = Self(0x0000008A);
    pub const ParagonCaster: Self = Self(0x0000008B);
    pub const ParagonMelee: Self = Self(0x0000008C);
}

impl From<u32> for EquipmentSet {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<EquipmentSet> for u32 {
    fn from(value: EquipmentSet) -> Self {
        value.0
    }
}
