use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

use super::DatBTreeFileFlags::DatBTreeFileFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DatBTreeFile {
    pub flags: DatBTreeFileFlags,
    pub version: u16,
    pub id: u32,
    pub offset: i32,
    pub size: u32,
    pub raw_date: u32,
    pub iteration: i32,
}

impl DatBTreeFile {
    pub const SIZE: usize = 24;
}

impl IUnpackable for DatBTreeFile {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.flags = DatBTreeFileFlags::from_bits_truncate(reader.read_u16());
        self.version = reader.read_u16();
        self.id = reader.read_u32();
        self.offset = reader.read_i32();
        self.size = reader.read_u32();
        self.raw_date = reader.read_u32();
        self.iteration = reader.read_i32();
        true
    }
}

impl IPackable for DatBTreeFile {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.flags.bits());
        writer.write_u16(self.version);
        writer.write_u32(self.id);
        writer.write_i32(self.offset);
        writer.write_u32(self.size);
        writer.write_u32(self.raw_date);
        writer.write_i32(self.iteration);
        true
    }
}
