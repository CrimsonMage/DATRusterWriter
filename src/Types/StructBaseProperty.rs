use std::collections::BTreeMap;

use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::BaseProperty::{BaseProperty, BasePropertyHeader},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StructBaseProperty {
    pub header: BasePropertyHeader,
    pub value: BTreeMap<u32, BaseProperty>,
}

impl StructBaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Struct
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Struct {
            header: self.header.clone(),
            value: self.value.clone(),
        }
    }
}

impl IUnpackable for StructBaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let _bucket_size = reader.read_byte();
        let count = reader.read_byte() as usize;
        self.value.clear();
        for _ in 0..count {
            let key = reader.read_u32();
            let Some(value) = BaseProperty::unpack_generic(reader) else {
                return false;
            };
            self.value.insert(key, value);
        }
        true
    }
}

impl IPackable for StructBaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_byte(0);
        writer.write_byte(self.value.len() as u8);
        for (key, property) in &self.value {
            writer.write_u32(*key);
            writer.write_item(property);
        }
        true
    }
}
