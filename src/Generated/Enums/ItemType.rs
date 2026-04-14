#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ItemType(pub u32);

impl ItemType {
    pub const None: Self = Self(0x00000000);
    pub const MeleeWeapon: Self = Self(0x00000001);
    pub const Armor: Self = Self(0x00000002);
    pub const Clothing: Self = Self(0x00000004);
    pub const Vestements: Self = Self(0x00000006);
    pub const Jewelry: Self = Self(0x00000008);
    pub const Creature: Self = Self(0x00000010);
    pub const Food: Self = Self(0x00000020);
    pub const Money: Self = Self(0x00000040);
    pub const Misc: Self = Self(0x00000080);
    pub const MissileWeapon: Self = Self(0x00000100);
    pub const Weapon: Self = Self(0x00000101);
    pub const Container: Self = Self(0x00000200);
    pub const LockableMagicTarget: Self = Self(0x00000280);
    pub const Useless: Self = Self(0x00000400);
    pub const Gem: Self = Self(0x00000800);
    pub const SpellComponents: Self = Self(0x00001000);
    pub const Writable: Self = Self(0x00002000);
    pub const Key: Self = Self(0x00004000);
    pub const Caster: Self = Self(0x00008000);
    pub const WeaponOrCaster: Self = Self(0x00008101);
    pub const RedirectableItemEnchantmentTarget: Self = Self(0x00008107);
    pub const Portal: Self = Self(0x00010000);
    pub const Lockable: Self = Self(0x00020000);
    pub const PromissoryNote: Self = Self(0x00040000);
    pub const ManaStone: Self = Self(0x00080000);
    pub const ItemEnchantableTarget: Self = Self(0x00088B8F);
    pub const Service: Self = Self(0x00100000);
    pub const MagicWieldable: Self = Self(0x00200000);
    pub const Item: Self = Self(0x002DFBEF);
    pub const CraftCookingBase: Self = Self(0x00400000);
    pub const VendorGrocer: Self = Self(0x00446220);
    pub const CraftAlchemyBase: Self = Self(0x00800000);
    pub const CraftFletchingBase: Self = Self(0x02000000);
    pub const CraftAlchemyIntermediate: Self = Self(0x04000000);
    pub const CraftFletchingIntermediate: Self = Self(0x08000000);
    pub const LifeStone: Self = Self(0x10000000);
    pub const PortalMagicTarget: Self = Self(0x10010000);
    pub const TinkeringTool: Self = Self(0x20000000);
    pub const TinkeringMaterial: Self = Self(0x40000000);
    pub const VendorShopKeep: Self = Self(0x480467A7);
    pub const Gameboard: Self = Self(0x80000000);
}

impl From<u32> for ItemType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<ItemType> for u32 {
    fn from(value: ItemType) -> Self {
        value.0
    }
}
