use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::BaseProperty::{BaseProperty, BasePropertyHeader},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnumBaseProperty {
    pub header: BasePropertyHeader,
    pub value: u32,
}

impl EnumBaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Enum
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Enum {
            header: self.header.clone(),
            value: self.value,
        }
    }
}

impl IUnpackable for EnumBaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_u32();
        true
    }
}

impl IPackable for EnumBaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_u32(self.value);
        true
    }
}
