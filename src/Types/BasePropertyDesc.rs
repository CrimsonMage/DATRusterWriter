use std::collections::BTreeMap;

use crate::{
    Generated::Enums::{
        BasePropertyType::BasePropertyType, PatchFlags::PatchFlags,
        PropertyCachingType::PropertyCachingType, PropertyDatFileType::PropertyDatFileType,
        PropertyGroupName::PropertyGroupName,
        PropertyInheritanceType::PropertyInheritanceType,
        PropertyPropagationType::PropertyPropagationType,
    },
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::BaseProperty::BaseProperty,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BasePropertyDesc {
    pub name: u32,
    pub property_type: BasePropertyType,
    pub group: PropertyGroupName,
    pub provider: u32,
    pub data: u32,
    pub patch_flags: PatchFlags,
    pub default_value: Option<BaseProperty>,
    pub max_value: Option<BaseProperty>,
    pub min_value: Option<BaseProperty>,
    pub prediction_timeout: f32,
    pub inheritance_type: PropertyInheritanceType,
    pub dat_file_type: PropertyDatFileType,
    pub propagation_type: PropertyPropagationType,
    pub caching_type: PropertyCachingType,
    pub required: bool,
    pub read_only: bool,
    pub no_checkpoint: bool,
    pub recorded: bool,
    pub do_not_replay: bool,
    pub absolute_time_stamp: bool,
    pub groupable: bool,
    pub propagate_to_children: bool,
    pub available_properties: BTreeMap<u32, u32>,
}

impl IUnpackable for BasePropertyDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.name = reader.read_u32();
        self.property_type = BasePropertyType::from(reader.read_u32());
        self.group = PropertyGroupName::from(reader.read_u32());
        self.provider = reader.read_u32();
        self.data = reader.read_u32();
        self.patch_flags = PatchFlags::from(reader.read_i32());

        let has_default_value = reader.read_bool(1);
        self.default_value = if has_default_value {
            BaseProperty::unpack_generic_master_property(reader, self.property_type)
        } else {
            None
        };

        let has_max_value = reader.read_bool(1);
        self.max_value = if has_max_value {
            BaseProperty::unpack_generic_master_property(reader, self.property_type)
        } else {
            None
        };

        let has_min_value = reader.read_bool(1);
        self.min_value = if has_min_value {
            BaseProperty::unpack_generic_master_property(reader, self.property_type)
        } else {
            None
        };

        self.prediction_timeout = reader.read_single();
        self.inheritance_type = PropertyInheritanceType::from(reader.read_byte());
        self.dat_file_type = PropertyDatFileType::from(reader.read_byte());
        self.propagation_type = PropertyPropagationType::from(reader.read_byte());
        self.caching_type = PropertyCachingType::from(reader.read_byte());

        self.required = reader.read_bool(1);
        self.read_only = reader.read_bool(1);
        self.no_checkpoint = reader.read_bool(1);
        self.recorded = reader.read_bool(1);
        self.do_not_replay = reader.read_bool(1);
        self.absolute_time_stamp = reader.read_bool(1);
        self.groupable = reader.read_bool(1);
        self.propagate_to_children = reader.read_bool(1);

        let _bucket_size = reader.read_byte();
        let count = reader.read_byte() as usize;
        self.available_properties.clear();
        for _ in 0..count {
            let key = reader.read_u32();
            let value = reader.read_u32();
            self.available_properties.insert(key, value);
        }

        true
    }
}

impl IPackable for BasePropertyDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.name);
        writer.write_u32(self.property_type.into());
        writer.write_u32(self.group.into());
        writer.write_u32(self.provider);
        writer.write_u32(self.data);
        writer.write_i32(self.patch_flags.into());

        writer.write_bool(self.default_value.is_some(), 1);
        if let Some(value) = &self.default_value {
            writer.write_item(value);
        }

        writer.write_bool(self.max_value.is_some(), 1);
        if let Some(value) = &self.max_value {
            writer.write_item(value);
        }

        writer.write_bool(self.min_value.is_some(), 1);
        if let Some(value) = &self.min_value {
            writer.write_item(value);
        }

        writer.write_single(self.prediction_timeout);
        writer.write_byte(self.inheritance_type.into());
        writer.write_byte(self.dat_file_type.into());
        writer.write_byte(self.propagation_type.into());
        writer.write_byte(self.caching_type.into());
        writer.write_bool(self.required, 1);
        writer.write_bool(self.read_only, 1);
        writer.write_bool(self.no_checkpoint, 1);
        writer.write_bool(self.recorded, 1);
        writer.write_bool(self.do_not_replay, 1);
        writer.write_bool(self.absolute_time_stamp, 1);
        writer.write_bool(self.groupable, 1);
        writer.write_bool(self.propagate_to_children, 1);
        writer.write_byte(1);
        writer.write_byte(self.available_properties.len() as u8);
        for (key, value) in &self.available_properties {
            writer.write_u32(*key);
            writer.write_u32(*value);
        }
        true
    }
}
