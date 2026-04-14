use crate::{
    Generated::Enums::RMDataType::RMDataType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MaterialProperty {
    pub name_id: u32,
    pub data_type: RMDataType,
    pub data_length: u32,
    pub data_length2: u32,
    pub data_length3: u16,
    pub data_length4: u8,
}

impl IUnpackable for MaterialProperty {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.name_id = reader.read_u32();
        self.data_type = RMDataType::from(reader.read_u16());
        reader.align(4);
        self.data_length = reader.read_u32();
        self.data_length2 = reader.read_u32();
        self.data_length3 = reader.read_u16();
        self.data_length4 = reader.read_byte();
        true
    }
}

impl IPackable for MaterialProperty {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.name_id);
        writer.write_u16(self.data_type.into());
        writer.align(4);
        writer.write_u32(self.data_length);
        writer.write_u32(self.data_length2);
        writer.write_u16(self.data_length3);
        writer.write_byte(self.data_length4);
        true
    }
}
