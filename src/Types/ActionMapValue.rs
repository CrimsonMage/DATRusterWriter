use crate::{
    Generated::Enums::ToggleType::ToggleType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::UserBindingData::UserBindingData,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActionMapValue {
    pub magic: u32,
    pub unknown: u8,
    pub toggle_type: ToggleType,
    pub dummy_list_length: u32,
    pub user_binding: UserBindingData,
}

impl IUnpackable for ActionMapValue {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.magic = reader.read_u32();
        self.unknown = reader.read_byte();
        self.toggle_type = ToggleType::from(reader.read_u32());
        self.dummy_list_length = reader.read_u32();
        self.user_binding = reader.read_item::<UserBindingData>();
        true
    }
}

impl IPackable for ActionMapValue {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.magic);
        writer.write_byte(self.unknown);
        writer.write_u32(self.toggle_type.into());
        writer.write_u32(self.dummy_list_length);
        writer.write_item(&self.user_binding);
        true
    }
}
