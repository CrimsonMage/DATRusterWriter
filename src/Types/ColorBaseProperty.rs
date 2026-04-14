use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{
        BaseProperty::{BaseProperty, BasePropertyHeader},
        ColorARGB::ColorARGB,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ColorBaseProperty {
    pub header: BasePropertyHeader,
    pub value: ColorARGB,
}

impl ColorBaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Color
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Color {
            header: self.header.clone(),
            value: self.value,
        }
    }
}

impl IUnpackable for ColorBaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_item::<ColorARGB>();
        true
    }
}

impl IPackable for ColorBaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_item(&self.value);
        true
    }
}
