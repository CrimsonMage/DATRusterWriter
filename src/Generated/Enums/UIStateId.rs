#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct UIStateId(pub u32);

impl UIStateId {
    pub const Undef: Self = Self(0x00000000);
    pub const Normal: Self = Self(0x00000001);
    pub const Highlight: Self = Self(0x00000006);
    pub const Closed: Self = Self(0x0000000B);
    pub const Open: Self = Self(0x0000000C);
    pub const Ghosted: Self = Self(0x0000000D);
    pub const Active: Self = Self(0x1000001B);
    pub const Selected: Self = Self(0x10000017);
    pub const Online: Self = Self(0x10000054);
    pub const Offline: Self = Self(0x10000055);

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
