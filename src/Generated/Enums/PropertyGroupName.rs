#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct PropertyGroupName(pub i32);

impl PropertyGroupName {
    pub const Invalid: Self = Self(0x00000000);
    pub const UICoreButton: Self = Self(0x00000001);
    pub const UICoreBrowser: Self = Self(0x00000002);
    pub const UICoreColorPicker: Self = Self(0x00000003);
    pub const UICoreComboBox: Self = Self(0x00000004);
    pub const UICoreContextMenu: Self = Self(0x00000005);
    pub const UICoreDialog: Self = Self(0x00000006);
    pub const UICoreDragbar: Self = Self(0x00000007);
    pub const UICoreElement: Self = Self(0x00000008);
    pub const UICoreField: Self = Self(0x00000009);
    pub const UICoreFrame: Self = Self(0x0000000A);
    pub const UICoreGroupBox: Self = Self(0x0000000B);
    pub const UICoreListBox: Self = Self(0x0000000C);
    pub const UICoreMenu: Self = Self(0x0000000D);
    pub const UICoreMeter: Self = Self(0x0000000E);
    pub const UICorePanel: Self = Self(0x0000000F);
    pub const UICoreResizebar: Self = Self(0x00000010);
    pub const UICoreScrollable: Self = Self(0x00000011);
    pub const UICoreScrollbar: Self = Self(0x00000012);
    pub const UICoreText: Self = Self(0x00000013);
    pub const UICoreViewport: Self = Self(0x00000014);
    pub const GameplayOptions: Self = Self(0x00000016);
    pub const Wb_AllWorkspaces: Self = Self(0x00000017);
    pub const Wb_Avatar: Self = Self(0x00000018);
    pub const Wb_Camera: Self = Self(0x00000019);
    pub const Wb_CursorSelection: Self = Self(0x0000001A);
    pub const Wb_DungeonWorkspace: Self = Self(0x0000001B);
    pub const Wb_EntityWorkspace: Self = Self(0x0000001C);
    pub const Wb_Grid: Self = Self(0x0000001D);
    pub const Wb_Misc: Self = Self(0x0000001E);
    pub const Wb_RenderOptions: Self = Self(0x0000001F);
    pub const Wb_WorldWorkspace: Self = Self(0x00000020);
    pub const Tools: Self = Self(0x00000021);
    pub const Physics: Self = Self(0x00000022);
    pub const Ethereal: Self = Self(0x00000023);
    pub const Link: Self = Self(0x00000024);
    pub const PregameUI: Self = Self(0x10000001);
    pub const GameUI: Self = Self(0x10000002);
    pub const SmartBoxWrapper: Self = Self(0x10000003);

    pub const INVALID: Self = Self::Invalid;
    pub const UI_CORE_BUTTON: Self = Self::UICoreButton;
    pub const UI_CORE_BROWSER: Self = Self::UICoreBrowser;
    pub const GAMEPLAY_OPTIONS: Self = Self::GameplayOptions;
    pub const PREGAME_UI: Self = Self::PregameUI;
    pub const GAME_UI: Self = Self::GameUI;
}

impl From<i32> for PropertyGroupName {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<u32> for PropertyGroupName {
    fn from(value: u32) -> Self {
        Self(value as i32)
    }
}

impl From<PropertyGroupName> for u32 {
    fn from(value: PropertyGroupName) -> Self {
        value.0 as u32
    }
}
