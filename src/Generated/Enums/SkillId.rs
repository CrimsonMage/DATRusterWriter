#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SkillId(pub i32);

impl SkillId {
    pub const AXE: Self = Self(0x01);
    pub const BOW: Self = Self(0x02);
    pub const CROSSBOW: Self = Self(0x03);
    pub const DAGGER: Self = Self(0x04);
    pub const MACE: Self = Self(0x05);
    pub const MELEE_DEFENSE: Self = Self(0x06);
    pub const MISSILE_DEFENSE: Self = Self(0x07);
    pub const SLING: Self = Self(0x08);
    pub const SPEAR: Self = Self(0x09);
    pub const STAFF: Self = Self(0x0A);
    pub const SWORD: Self = Self(0x0B);
    pub const THROWN_WEAPONS: Self = Self(0x0C);
    pub const UNARMED_COMBAT: Self = Self(0x0D);
    pub const ARCANE_LORE: Self = Self(0x0E);
    pub const MAGIC_DEFENSE: Self = Self(0x0F);
    pub const MANA_CONVERSION: Self = Self(0x10);
    pub const SPELLCRAFT: Self = Self(0x11);
    pub const ITEM_TINKERING: Self = Self(0x12);
    pub const ASSESS_PERSON: Self = Self(0x13);
    pub const DECEPTION: Self = Self(0x14);
    pub const HEALING: Self = Self(0x15);
    pub const JUMP: Self = Self(0x16);
    pub const LOCKPICK: Self = Self(0x17);
    pub const RUN: Self = Self(0x18);
    pub const AWARENESS: Self = Self(0x19);
    pub const ARMOR_REPAIR: Self = Self(0x1A);
    pub const ASSESS_CREATURE: Self = Self(0x1B);
    pub const WEAPON_TINKERING: Self = Self(0x1C);
    pub const ARMOR_TINKERING: Self = Self(0x1D);
    pub const MAGIC_ITEM_TINKERING: Self = Self(0x1E);
    pub const CREATURE_ENCHANTMENT: Self = Self(0x1F);
    pub const ITEM_ENCHANTMENT: Self = Self(0x20);
    pub const LIFE_MAGIC: Self = Self(0x21);
    pub const WAR_MAGIC: Self = Self(0x22);
    pub const LEADERSHIP: Self = Self(0x23);
    pub const LOYALTY: Self = Self(0x24);
    pub const FLETCHING: Self = Self(0x25);
    pub const ALCHEMY: Self = Self(0x26);
    pub const COOKING: Self = Self(0x27);
    pub const SALVAGING: Self = Self(0x28);
    pub const TWO_HANDED_COMBAT: Self = Self(0x29);
    pub const GEARCRAFT: Self = Self(0x2A);
    pub const VOID_MAGIC: Self = Self(0x2B);
    pub const HEAVY_WEAPONS: Self = Self(0x2C);
    pub const LIGHT_WEAPONS: Self = Self(0x2D);
    pub const FINESSE_WEAPONS: Self = Self(0x2E);
    pub const MISSLE_WEAPONS: Self = Self(0x2F);
    pub const SHIELD: Self = Self(0x30);
    pub const DUAL_WIELD: Self = Self(0x31);
    pub const RECKLESSNESS: Self = Self(0x32);
    pub const SNEAK_ATTACK: Self = Self(0x33);
    pub const DIRTY_FIGHTING: Self = Self(0x34);
    pub const CHALLENGE: Self = Self(0x35);
    pub const SUMMONING: Self = Self(0x36);
}

impl From<i32> for SkillId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<SkillId> for i32 {
    fn from(value: SkillId) -> Self {
        value.0
    }
}
