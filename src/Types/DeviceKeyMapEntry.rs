use uuid::Uuid;

use crate::{
    Generated::Enums::DeviceType::DeviceType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DeviceKeyMapEntry {
    pub device_type: DeviceType,
    pub guid: Uuid,
}

impl IUnpackable for DeviceKeyMapEntry {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.device_type = DeviceType::from(reader.read_byte());
        self.guid = reader.read_guid();
        true
    }
}

impl IPackable for DeviceKeyMapEntry {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.device_type.into());
        writer.write_guid(self.guid);
        true
    }
}
