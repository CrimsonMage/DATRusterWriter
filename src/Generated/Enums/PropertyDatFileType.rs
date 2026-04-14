#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct PropertyDatFileType(pub u8);

impl PropertyDatFileType {
    pub const ClientOnlyData: Self = Self(0x0);
    pub const ServerOnlyData: Self = Self(0x1);
    pub const SharedData: Self = Self(0x2);

    pub const CLIENT_ONLY_DATA: Self = Self::ClientOnlyData;
    pub const SERVER_ONLY_DATA: Self = Self::ServerOnlyData;
    pub const SHARED_DATA: Self = Self::SharedData;
}

impl From<u8> for PropertyDatFileType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PropertyDatFileType> for u8 {
    fn from(value: PropertyDatFileType) -> Self {
        value.0
    }
}
