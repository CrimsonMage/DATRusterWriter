#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct BasePropertyType(pub i32);

impl BasePropertyType {
    pub const Invalid: Self = Self(0x00000000);
    pub const Bool: Self = Self(0x00000001);
    pub const Integer: Self = Self(0x00000002);
    pub const LongInteger: Self = Self(0x00000003);
    pub const Float: Self = Self(0x00000004);
    pub const Vector: Self = Self(0x00000005);
    pub const Color: Self = Self(0x00000006);
    pub const String: Self = Self(0x00000007);
    pub const StringInfo: Self = Self(0x00000008);
    pub const Enum: Self = Self(0x00000009);
    pub const DataId: Self = Self(0x0000000A);
    pub const Waveform: Self = Self(0x0000000B);
    pub const InstanceId: Self = Self(0x0000000C);
    pub const Position: Self = Self(0x0000000D);
    pub const TimeStamp: Self = Self(0x0000000E);
    pub const Bitfield32: Self = Self(0x0000000F);
    pub const Bitfield64: Self = Self(0x00000010);
    pub const Array: Self = Self(0x00000011);
    pub const Struct: Self = Self(0x00000012);
    pub const StringToken: Self = Self(0x00000013);
    pub const PropertyName: Self = Self(0x00000014);
    pub const TriState: Self = Self(0x00000015);

    pub const INVALID: Self = Self::Invalid;
    pub const BOOL: Self = Self::Bool;
    pub const INTEGER: Self = Self::Integer;
    pub const LONG_INTEGER: Self = Self::LongInteger;
    pub const FLOAT: Self = Self::Float;
    pub const VECTOR: Self = Self::Vector;
    pub const COLOR: Self = Self::Color;
    pub const STRING: Self = Self::String;
    pub const STRING_INFO: Self = Self::StringInfo;
    pub const ENUM: Self = Self::Enum;
    pub const DATA_ID: Self = Self::DataId;
    pub const WAVEFORM: Self = Self::Waveform;
    pub const INSTANCE_ID: Self = Self::InstanceId;
    pub const POSITION: Self = Self::Position;
    pub const TIME_STAMP: Self = Self::TimeStamp;
    pub const BITFIELD32: Self = Self::Bitfield32;
    pub const BITFIELD64: Self = Self::Bitfield64;
    pub const ARRAY: Self = Self::Array;
    pub const STRUCT: Self = Self::Struct;
    pub const STRING_TOKEN: Self = Self::StringToken;
    pub const PROPERTY_NAME: Self = Self::PropertyName;
    pub const TRI_STATE: Self = Self::TriState;
}

impl From<i32> for BasePropertyType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<u32> for BasePropertyType {
    fn from(value: u32) -> Self {
        Self(value as i32)
    }
}

impl From<BasePropertyType> for i32 {
    fn from(value: BasePropertyType) -> Self {
        value.0
    }
}

impl From<BasePropertyType> for u32 {
    fn from(value: BasePropertyType) -> Self {
        value.0 as u32
    }
}
