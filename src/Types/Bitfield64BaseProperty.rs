use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::BaseProperty::{BaseProperty, BasePropertyHeader},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bitfield64BaseProperty {
    pub header: BasePropertyHeader,
    pub value: u64,
}

impl Bitfield64BaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Bitfield64
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Bitfield64 {
            header: self.header.clone(),
            value: self.value,
        }
    }
}

impl IUnpackable for Bitfield64BaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_u64();
        true
    }
}

impl IPackable for Bitfield64BaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_u64(self.value);
        true
    }
}
