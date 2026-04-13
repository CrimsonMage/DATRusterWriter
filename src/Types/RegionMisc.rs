use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegionMisc {
    pub version: u32,
    pub game_map_id: u32,
    pub autotest_map_id: u32,
    pub autotest_map_size: u32,
    pub clear_cell_id: u32,
    pub clear_monster_id: u32,
}

impl IUnpackable for RegionMisc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.version = reader.read_u32();
        self.game_map_id = reader.read_u32();
        self.autotest_map_id = reader.read_u32();
        self.autotest_map_size = reader.read_u32();
        self.clear_cell_id = reader.read_u32();
        self.clear_monster_id = reader.read_u32();
        true
    }
}

impl IPackable for RegionMisc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.version);
        writer.write_u32(self.game_map_id);
        writer.write_u32(self.autotest_map_id);
        writer.write_u32(self.autotest_map_size);
        writer.write_u32(self.clear_cell_id);
        writer.write_u32(self.clear_monster_id);
        true
    }
}
