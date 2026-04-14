use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InputsConflictsValue {
    pub input_map: u32,
    pub conflicting_input_maps: Vec<u32>,
}

impl IUnpackable for InputsConflictsValue {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.input_map = reader.read_u32();
        let count = reader.read_u32() as usize;
        self.conflicting_input_maps = (0..count).map(|_| reader.read_u32()).collect();
        true
    }
}

impl IPackable for InputsConflictsValue {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.input_map);
        writer.write_u32(self.conflicting_input_maps.len() as u32);
        for item in &self.conflicting_input_maps {
            writer.write_u32(*item);
        }
        true
    }
}
