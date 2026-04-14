use crate::{
    DBObjs::StringTable::StringTable,
    Generated::Enums::StringInfoOverrideFlag::StringInfoOverrideFlag,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::QualifiedDataId::QualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StringInfo {
    pub token: u8,
    pub string_id: u32,
    pub table_id: QualifiedDataId<StringTable>,
    pub override_flag: StringInfoOverrideFlag,
    pub english: u8,
    pub comment: u8,
}

impl IUnpackable for StringInfo {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.token = reader.read_byte();
        self.string_id = reader.read_u32();
        self.table_id = reader.read_item::<QualifiedDataId<StringTable>>();
        self.override_flag = StringInfoOverrideFlag::from_bits_truncate(reader.read_byte());
        self.english = reader.read_byte();
        self.comment = reader.read_byte();
        true
    }
}

impl IPackable for StringInfo {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.token);
        writer.write_u32(self.string_id);
        writer.write_item(&self.table_id);
        writer.write_byte(self.override_flag.bits());
        writer.write_byte(self.english);
        writer.write_byte(self.comment);
        true
    }
}
