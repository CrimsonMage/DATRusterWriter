use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{
        BaseProperty::{BaseProperty, BasePropertyHeader},
        StringInfo::StringInfo,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StringInfoBaseProperty {
    pub header: BasePropertyHeader,
    pub value: StringInfo,
}

impl StringInfoBaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::StringInfo
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::StringInfo {
            header: self.header.clone(),
            value: self.value.clone(),
        }
    }
}

impl IUnpackable for StringInfoBaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_item::<StringInfo>();
        true
    }
}

impl IPackable for StringInfoBaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_item(&self.value);
        true
    }
}
