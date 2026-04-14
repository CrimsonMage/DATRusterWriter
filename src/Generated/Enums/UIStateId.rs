#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct UIStateId(pub u32);

impl UIStateId {
    pub const Undef: Self = Self(0x00000000);
    pub const Normal: Self = Self(0x00000001);
    pub const Normal_rollover: Self = Self(0x00000002);
    pub const Normal_pressed: Self = Self(0x00000003);
    pub const Normal_focussed: Self = Self(0x00000004);
    pub const Normal_activated: Self = Self(0x00000005);
    pub const Highlight: Self = Self(0x00000006);
    pub const Highlight_rollover: Self = Self(0x00000007);
    pub const Highlight_pressed: Self = Self(0x00000008);
    pub const Drag_rollover_accept: Self = Self(0x00000009);
    pub const Drag_rollover_reject: Self = Self(0x0000000A);
    pub const Closed: Self = Self(0x0000000B);
    pub const Open: Self = Self(0x0000000C);
    pub const Ghosted: Self = Self(0x0000000D);
    pub const Unencumbered: Self = Self(0x0000000E);
    pub const Encumbered: Self = Self(0x0000000F);
    pub const Heavily_encumbered: Self = Self(0x00000010);
    pub const Connection_good: Self = Self(0x00000011);
    pub const Connection_uncertain: Self = Self(0x00000012);
    pub const Connection_bad: Self = Self(0x00000013);
    pub const Connection_disconnected: Self = Self(0x00000014);
    pub const Csm_highlight: Self = Self(0x00000015);
    pub const Csm_normal: Self = Self(0x00000016);
    pub const Csm_ghosted: Self = Self(0x00000017);
    pub const Dialog_pending_true: Self = Self(0x00000018);
    pub const Dialog_pending_false: Self = Self(0x00000019);
    pub const Talkfocus_highlight: Self = Self(0x10000001);
    pub const MeleeCombat: Self = Self(0x10000003);
    pub const MissileCombat: Self = Self(0x10000004);
    pub const HideDetail: Self = Self(0x10000006);
    pub const ShowDetail: Self = Self(0x10000007);
    pub const Abuse_PageOne: Self = Self(0x10000008);
    pub const Abuse_PageTwo: Self = Self(0x10000009);
    pub const Abuse_PageThree: Self = Self(0x1000000A);
    pub const ObjectSelected: Self = Self(0x1000000B);
    pub const StackedItemSelected: Self = Self(0x1000000C);
    pub const StackedItem: Self = Self(0x1000000D);
    pub const UrgentAssistance_PageOne: Self = Self(0x1000000E);
    pub const UrgentAssistance_PageTwo: Self = Self(0x1000000F);
    pub const UrgentAssistance_PageThree: Self = Self(0x10000010);
    pub const StatManagement_Footer_Default: Self = Self(0x10000011);
    pub const StatManagement_Footer_Text: Self = Self(0x10000012);
    pub const StatManagement_Footer_Meter: Self = Self(0x10000013);
    pub const Buffed: Self = Self(0x10000014);
    pub const Unselected: Self = Self(0x10000016);
    pub const Selected: Self = Self(0x10000017);
    pub const Unlocked: Self = Self(0x10000018);
    pub const Locked: Self = Self(0x10000019);
    pub const Inactive: Self = Self(0x1000001A);
    pub const Active: Self = Self(0x1000001B);
    pub const ItemSlot_Empty: Self = Self(0x1000001C);
    pub const ItemSlot_Filled: Self = Self(0x1000001D);
    pub const Aluvian: Self = Self(0x10000021);
    pub const Gharundim: Self = Self(0x10000022);
    pub const Sho: Self = Self(0x10000023);
    pub const Viamont: Self = Self(0x10000024);
    pub const Heritage: Self = Self(0x10000025);
    pub const Profession: Self = Self(0x10000026);
    pub const Skills: Self = Self(0x10000027);
    pub const Appearance: Self = Self(0x10000028);
    pub const Town: Self = Self(0x10000029);
    pub const Summary: Self = Self(0x1000002A);
    pub const Custom: Self = Self(0x1000002B);
    pub const Bow_hunter: Self = Self(0x1000002C);
    pub const Life_caster: Self = Self(0x1000002D);
    pub const War_mage: Self = Self(0x1000002E);
    pub const Wayfarer: Self = Self(0x1000002F);
    pub const Soldier: Self = Self(0x10000030);
    pub const Swashbuckler: Self = Self(0x10000031);
    pub const Create_normal: Self = Self(0x10000032);
    pub const Create_admin: Self = Self(0x10000033);
    pub const Holtburg: Self = Self(0x10000034);
    pub const Sanamar: Self = Self(0x10000035);
    pub const Yaraq: Self = Self(0x10000036);
    pub const Shoushi: Self = Self(0x10000037);
    pub const Frame1: Self = Self(0x10000038);
    pub const Frame2: Self = Self(0x10000039);
    pub const Frame3: Self = Self(0x1000003A);
    pub const Inprogress: Self = Self(0x1000003B);
    pub const Done: Self = Self(0x1000003C);
    pub const IntroVideo: Self = Self(0x1000003E);
    pub const ItemSlot_DragOver_Normal: Self = Self(0x1000003F);
    pub const ItemSlot_DragOver_Accept: Self = Self(0x10000040);
    pub const ItemSlot_DragOver_Reject: Self = Self(0x10000041);
    pub const JumpMode: Self = Self(0x10000042);
    pub const MeleeMode: Self = Self(0x10000043);
    pub const MissileMode: Self = Self(0x10000044);
    pub const DDDMode: Self = Self(0x10000045);
    pub const ItemSlot_DragOver_DropIn: Self = Self(0x10000046);
    pub const Maximized: Self = Self(0x10000047);
    pub const Minimized: Self = Self(0x10000048);
    pub const Uninscribed: Self = Self(0x10000050);
    pub const Create_envoy: Self = Self(0x10000053);
    pub const Online: Self = Self(0x10000054);
    pub const Offline: Self = Self(0x10000055);
    pub const IsCharacter: Self = Self(0x10000056);
    pub const IsAccount: Self = Self(0x10000057);
    pub const Shadow: Self = Self(0x10000058);
    pub const Penumbraen: Self = Self(0x10000059);
    pub const Gearknight: Self = Self(0x1000005A);
    pub const Undead: Self = Self(0x1000005B);
    pub const Empyrean: Self = Self(0x1000005C);
    pub const Olthoi: Self = Self(0x1000005D);
    pub const Olthoiacid: Self = Self(0x1000005E);
    pub const Auntumerok: Self = Self(0x1000005F);
    pub const Lugian: Self = Self(0x10000060);
    pub const LockedUI: Self = Self(0x10000063);
    pub const UnlockedUI: Self = Self(0x10000064);

    pub const UNDEF: Self = Self::Undef;
    pub const NORMAL: Self = Self::Normal;
    pub const HIGHLIGHT: Self = Self::Highlight;
    pub const CLOSED: Self = Self::Closed;
    pub const OPEN: Self = Self::Open;
    pub const GHOSTED: Self = Self::Ghosted;
    pub const ACTIVE: Self = Self::Active;
    pub const SELECTED: Self = Self::Selected;
    pub const ONLINE: Self = Self::Online;
    pub const OFFLINE: Self = Self::Offline;
}

impl From<u32> for UIStateId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<UIStateId> for u32 {
    fn from(value: UIStateId) -> Self {
        value.0
    }
}
