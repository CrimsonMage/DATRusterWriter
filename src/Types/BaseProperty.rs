use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader,
        DatBinWriter::DatBinWriter,
        IUnpackable::IUnpackable,
        IPackable::IPackable,
        Numerics::Vector3,
    },
    Types::{ColorARGB::ColorARGB, StringInfo::StringInfo},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BasePropertyHeader {
    pub master_property_id: u32,
    pub should_pack_master_property_id: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseProperty {
    Bool {
        header: BasePropertyHeader,
        value: bool,
    },
    Integer {
        header: BasePropertyHeader,
        value: i32,
    },
    Float {
        header: BasePropertyHeader,
        value: f32,
    },
    Vector {
        header: BasePropertyHeader,
        value: Vector3,
    },
    Color {
        header: BasePropertyHeader,
        value: ColorARGB,
    },
    StringInfo {
        header: BasePropertyHeader,
        value: StringInfo,
    },
    Enum {
        header: BasePropertyHeader,
        value: u32,
    },
    DataId {
        header: BasePropertyHeader,
        value: u32,
    },
    InstanceId {
        header: BasePropertyHeader,
        value: u32,
    },
    Bitfield32 {
        header: BasePropertyHeader,
        value: u32,
    },
    Bitfield64 {
        header: BasePropertyHeader,
        value: u64,
    },
}

impl Default for BaseProperty {
    fn default() -> Self {
        Self::Integer {
            header: BasePropertyHeader::default(),
            value: 0,
        }
    }
}

impl BaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        match self {
            Self::Bool { .. } => BasePropertyType::Bool,
            Self::Integer { .. } => BasePropertyType::Integer,
            Self::Float { .. } => BasePropertyType::Float,
            Self::Vector { .. } => BasePropertyType::Vector,
            Self::Color { .. } => BasePropertyType::Color,
            Self::StringInfo { .. } => BasePropertyType::StringInfo,
            Self::Enum { .. } => BasePropertyType::Enum,
            Self::DataId { .. } => BasePropertyType::DataId,
            Self::InstanceId { .. } => BasePropertyType::InstanceId,
            Self::Bitfield32 { .. } => BasePropertyType::Bitfield32,
            Self::Bitfield64 { .. } => BasePropertyType::Bitfield64,
        }
    }

    fn header(&self) -> &BasePropertyHeader {
        match self {
            Self::Bool { header, .. }
            | Self::Integer { header, .. }
            | Self::Float { header, .. }
            | Self::Vector { header, .. }
            | Self::Color { header, .. }
            | Self::StringInfo { header, .. }
            | Self::Enum { header, .. }
            | Self::DataId { header, .. }
            | Self::InstanceId { header, .. }
            | Self::Bitfield32 { header, .. }
            | Self::Bitfield64 { header, .. } => header,
        }
    }

    pub fn unpack_generic_master_property(
        reader: &mut DatBinReader<'_>,
        property_type: BasePropertyType,
    ) -> Option<Self> {
        Self::unpack_instance_from_type(reader, property_type, false, 0)
    }

    pub fn unpack_instance_from_type(
        reader: &mut DatBinReader<'_>,
        property_type: BasePropertyType,
        should_pack_master_property_id: bool,
        master_property_id: u32,
    ) -> Option<Self> {
        let header = BasePropertyHeader {
            master_property_id,
            should_pack_master_property_id,
        };

        Some(match property_type {
            t if t == BasePropertyType::Bool => Self::Bool {
                header,
                value: reader.read_bool(1),
            },
            t if t == BasePropertyType::Integer => Self::Integer {
                header,
                value: reader.read_i32(),
            },
            t if t == BasePropertyType::Float => Self::Float {
                header,
                value: reader.read_single(),
            },
            t if t == BasePropertyType::Vector => Self::Vector {
                header,
                value: reader.read_vector3(),
            },
            t if t == BasePropertyType::Color => Self::Color {
                header,
                value: reader.read_item::<ColorARGB>(),
            },
            t if t == BasePropertyType::StringInfo => Self::StringInfo {
                header,
                value: reader.read_item::<StringInfo>(),
            },
            t if t == BasePropertyType::Enum => Self::Enum {
                header,
                value: reader.read_u32(),
            },
            t if t == BasePropertyType::DataId => Self::DataId {
                header,
                value: reader.read_u32(),
            },
            t if t == BasePropertyType::InstanceId => Self::InstanceId {
                header,
                value: reader.read_u32(),
            },
            t if t == BasePropertyType::Bitfield32 => Self::Bitfield32 {
                header,
                value: reader.read_u32(),
            },
            t if t == BasePropertyType::Bitfield64 => Self::Bitfield64 {
                header,
                value: reader.read_u64(),
            },
            _ => return None,
        })
    }
}

impl IUnpackable for BaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let property_type = self.property_type();
        if let Some(value) =
            Self::unpack_instance_from_type(reader, property_type, false, 0)
        {
            *self = value;
            true
        } else {
            false
        }
    }
}

impl IPackable for BaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let header = self.header();
        if header.should_pack_master_property_id {
            writer.write_u32(header.master_property_id);
        }

        match self {
            Self::Bool { value, .. } => writer.write_bool(*value, 1),
            Self::Integer { value, .. } => writer.write_i32(*value),
            Self::Float { value, .. } => writer.write_single(*value),
            Self::Vector { value, .. } => writer.write_vector3(*value),
            Self::Color { value, .. } => writer.write_item(value),
            Self::StringInfo { value, .. } => writer.write_item(value),
            Self::Enum { value, .. }
            | Self::DataId { value, .. }
            | Self::InstanceId { value, .. }
            | Self::Bitfield32 { value, .. } => writer.write_u32(*value),
            Self::Bitfield64 { value, .. } => writer.write_u64(*value),
        }
        true
    }
}
