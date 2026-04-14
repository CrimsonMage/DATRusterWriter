use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{PStringBase::PStringBase, Position::Position},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StartingArea {
    pub name: PStringBase<u8>,
    pub locations: Vec<Position>,
}

impl IUnpackable for StartingArea {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.name = reader.read_item::<PStringBase<u8>>();
        let count = reader.read_compressed_uint() as usize;
        self.locations.clear();
        for _ in 0..count {
            self.locations.push(reader.read_item::<Position>());
        }
        true
    }
}

impl IPackable for StartingArea {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.name);
        writer.write_compressed_uint(self.locations.len() as u32);
        for location in &self.locations {
            writer.write_item(location);
        }
        true
    }
}

