#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct DeviceType(pub u8);

impl DeviceType {
    pub const Invalid: Self = Self(0x00);
    pub const Keyboard: Self = Self(0x01);
    pub const Mouse: Self = Self(0x02);
    pub const Joystick: Self = Self(0x03);
    pub const Virtual: Self = Self(0x04);

    pub const INVALID: Self = Self::Invalid;
    pub const KEYBOARD: Self = Self::Keyboard;
    pub const MOUSE: Self = Self::Mouse;
    pub const JOYSTICK: Self = Self::Joystick;
    pub const VIRTUAL: Self = Self::Virtual;
}

impl From<u8> for DeviceType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<DeviceType> for u8 {
    fn from(value: DeviceType) -> Self {
        value.0
    }
}
