use crate::{
    DBObjs::StringTable::StringTable,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{PStringBase::PStringBase, QualifiedDataId::QualifiedDataId},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StringTableString {
    pub data_id: QualifiedDataId<StringTable>,
    pub strings: Vec<PStringBase<u16>>,
    pub variables: Vec<u32>,
    pub is_var_name_table_worth_packing: bool,
}

impl IUnpackable for StringTableString {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.data_id = reader.read_item::<QualifiedDataId<StringTable>>();

        let strings_count = reader.read_u32() as usize;
        self.strings.clear();
        for _ in 0..strings_count {
            self.strings.push(reader.read_item::<PStringBase<u16>>());
        }

        let variables_count = reader.read_u32() as usize;
        self.variables.clear();
        for _ in 0..variables_count {
            self.variables.push(reader.read_u32());
        }

        self.is_var_name_table_worth_packing = reader.read_bool(1);
        true
    }
}

impl IPackable for StringTableString {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.data_id);
        writer.write_u32(self.strings.len() as u32);
        for value in &self.strings {
            writer.write_item(value);
        }
        writer.write_u32(self.variables.len() as u32);
        for value in &self.variables {
            writer.write_u32(*value);
        }
        writer.write_bool(self.is_var_name_table_worth_packing, 1);
        true
    }
}
