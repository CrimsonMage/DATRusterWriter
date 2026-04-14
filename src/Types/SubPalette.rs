use crate::{
    DBObjs::Palette::Palette,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::PackedQualifiedDataId::PackedQualifiedDataId,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SubPalette {
    pub sub_id: PackedQualifiedDataId<Palette>,
    pub offset: u8,
    pub num_colors: u8,
}

impl IUnpackable for SubPalette {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.sub_id = reader.read_item::<PackedQualifiedDataId<Palette>>();
        self.offset = reader.read_byte();
        self.num_colors = reader.read_byte();
        true
    }
}

impl IPackable for SubPalette {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.sub_id);
        writer.write_byte(self.offset);
        writer.write_byte(self.num_colors);
        true
    }
}
