use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::BaseProperty::{BaseProperty, BasePropertyHeader},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArrayBaseProperty {
    pub header: BasePropertyHeader,
    pub value: Vec<BaseProperty>,
}

impl ArrayBaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Array
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Array {
            header: self.header.clone(),
            value: self.value.clone(),
        }
    }
}

impl IUnpackable for ArrayBaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let count = reader.read_u32() as usize;
        self.value.clear();
        self.value.reserve(count);
        for _ in 0..count {
            let Some(value) = BaseProperty::unpack_generic(reader) else {
                return false;
            };
            self.value.push(value);
        }
        true
    }
}

impl IPackable for ArrayBaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_u32(self.value.len() as u32);
        for item in &self.value {
            writer.write_item(item);
        }
        true
    }
}
