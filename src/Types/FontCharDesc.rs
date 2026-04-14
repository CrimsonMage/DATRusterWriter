use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FontCharDesc {
    pub unicode: u16,
    pub offset_x: u16,
    pub offset_y: u16,
    pub width: u8,
    pub height: u8,
    pub horizontal_offset_before: i8,
    pub horizontal_offset_after: i8,
    pub vertical_offset_before: i8,
}

impl IUnpackable for FontCharDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.unicode = reader.read_u16();
        self.offset_x = reader.read_u16();
        self.offset_y = reader.read_u16();
        self.width = reader.read_byte();
        self.height = reader.read_byte();
        self.horizontal_offset_before = reader.read_sbyte();
        self.horizontal_offset_after = reader.read_sbyte();
        self.vertical_offset_before = reader.read_sbyte();
        true
    }
}

impl IPackable for FontCharDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.unicode);
        writer.write_u16(self.offset_x);
        writer.write_u16(self.offset_y);
        writer.write_byte(self.width);
        writer.write_byte(self.height);
        writer.write_sbyte(self.horizontal_offset_before);
        writer.write_sbyte(self.horizontal_offset_after);
        writer.write_sbyte(self.vertical_offset_before);
        true
    }
}
