use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::PStringBase::PStringBase,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StringTableData {
    pub var_names: Vec<PStringBase<u16>>,
    pub vars: Vec<PStringBase<u16>>,
    pub strings: Vec<PStringBase<u16>>,
    pub comments: Vec<u32>,
    pub unknown: u8,
}

impl IUnpackable for StringTableData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let var_names_count = reader.read_u16() as usize;
        self.var_names.clear();
        for _ in 0..var_names_count {
            self.var_names.push(reader.read_item::<PStringBase<u16>>());
        }

        let vars_count = reader.read_u16() as usize;
        self.vars.clear();
        for _ in 0..vars_count {
            self.vars.push(reader.read_item::<PStringBase<u16>>());
        }

        let strings_count = reader.read_u32() as usize;
        self.strings.clear();
        for _ in 0..strings_count {
            self.strings.push(reader.read_item::<PStringBase<u16>>());
        }

        let comments_count = reader.read_u32() as usize;
        self.comments.clear();
        for _ in 0..comments_count {
            self.comments.push(reader.read_u32());
        }

        self.unknown = reader.read_byte();
        true
    }
}

impl IPackable for StringTableData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.var_names.len() as u16);
        for value in &self.var_names {
            writer.write_item(value);
        }
        writer.write_u16(self.vars.len() as u16);
        for value in &self.vars {
            writer.write_item(value);
        }
        writer.write_u32(self.strings.len() as u32);
        for value in &self.strings {
            writer.write_item(value);
        }
        writer.write_u32(self.comments.len() as u32);
        for value in &self.comments {
            writer.write_u32(*value);
        }
        writer.write_byte(self.unknown);
        true
    }
}
