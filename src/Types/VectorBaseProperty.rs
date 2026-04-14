use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable, Numerics::Vector3,
    },
    Types::BaseProperty::{BaseProperty, BasePropertyHeader},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VectorBaseProperty {
    pub header: BasePropertyHeader,
    pub value: Vector3,
}

impl VectorBaseProperty {
    pub fn property_type(&self) -> BasePropertyType {
        BasePropertyType::Vector
    }

    pub fn as_base_property(&self) -> BaseProperty {
        BaseProperty::Vector {
            header: self.header.clone(),
            value: self.value,
        }
    }
}

impl IUnpackable for VectorBaseProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.value = reader.read_vector3();
        true
    }
}

impl IPackable for VectorBaseProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        if self.header.should_pack_master_property_id {
            writer.write_u32(self.header.master_property_id);
        }
        writer.write_vector3(self.value);
        true
    }
}
