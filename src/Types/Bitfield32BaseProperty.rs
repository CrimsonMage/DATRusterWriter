use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::BaseProperty::{BaseProperty, BasePropertyHeader},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bitfield32BaseProperty {
    pub header: BasePropertyHeader,
    pub value: u32,
}

impl Bitfield32BaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Bitfield32
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Bitfield32 {
            header: self.header.clone(),
            value: self.value,
        }
    }
}

impl IUnpackable for Bitfield32BaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_u32();
        true
    }
}

impl IPackable for Bitfield32BaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_u32(self.value);
        true
    }
}
