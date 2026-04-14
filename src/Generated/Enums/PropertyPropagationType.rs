#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct PropertyPropagationType(pub u8);

impl PropertyPropagationType {
    pub const NetPredictedSharedVisually: Self = Self(0x0);
    pub const NetPredictedSharedPrivately: Self = Self(0x1);
    pub const NetSharedVisually: Self = Self(0x2);
    pub const NetSharedPrivately: Self = Self(0x3);
    pub const NetNotShared: Self = Self(0x4);
    pub const WorldSharedWithServers: Self = Self(0x5);
    pub const WorldSharedWithServersAndClients: Self = Self(0x6);

    pub const NET_PREDICTED_SHARED_VISUALLY: Self = Self::NetPredictedSharedVisually;
    pub const NET_PREDICTED_SHARED_PRIVATELY: Self = Self::NetPredictedSharedPrivately;
    pub const NET_SHARED_VISUALLY: Self = Self::NetSharedVisually;
    pub const NET_SHARED_PRIVATELY: Self = Self::NetSharedPrivately;
    pub const NET_NOT_SHARED: Self = Self::NetNotShared;
    pub const WORLD_SHARED_WITH_SERVERS: Self = Self::WorldSharedWithServers;
    pub const WORLD_SHARED_WITH_SERVERS_AND_CLIENTS: Self = Self::WorldSharedWithServersAndClients;
}

impl From<u8> for PropertyPropagationType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PropertyPropagationType> for u8 {
    fn from(value: PropertyPropagationType) -> Self {
        value.0
    }
}
