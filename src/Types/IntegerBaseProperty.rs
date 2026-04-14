use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::BaseProperty::{BaseProperty, BasePropertyHeader},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IntegerBaseProperty {
    pub header: BasePropertyHeader,
    pub value: i32,
}

impl IntegerBaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Integer
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Integer {
            header: self.header.clone(),
            value: self.value,
        }
    }
}

impl IUnpackable for IntegerBaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_i32();
        true
    }
}

impl IPackable for IntegerBaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_i32(self.value);
        true
    }
}
