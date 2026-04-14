use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::BaseProperty::{BaseProperty, BasePropertyHeader},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BoolBaseProperty {
    pub header: BasePropertyHeader,
    pub value: bool,
}

impl BoolBaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Bool
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Bool {
            header: self.header.clone(),
            value: self.value,
        }
    }
}

impl IUnpackable for BoolBaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_bool(1);
        true
    }
}

impl IPackable for BoolBaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_bool(self.value, 1);
        true
    }
}
