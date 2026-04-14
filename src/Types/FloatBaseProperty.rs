use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::BaseProperty::{BaseProperty, BasePropertyHeader},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloatBaseProperty {
    pub header: BasePropertyHeader,
    pub value: f32,
}

impl FloatBaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Float
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Float {
            header: self.header.clone(),
            value: self.value,
        }
    }
}

impl IUnpackable for FloatBaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_single();
        true
    }
}

impl IPackable for FloatBaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_single(self.value);
        true
    }
}
