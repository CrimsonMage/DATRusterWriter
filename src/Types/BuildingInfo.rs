use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{BuildingPortal::BuildingPortal, Frame::Frame},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuildingInfo {
    pub model_id: u32,
    pub frame: Frame,
    pub num_leaves: u32,
    pub portals: Vec<BuildingPortal>,
}

impl IUnpackable for BuildingInfo {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.model_id = reader.read_u32();
        self.frame = reader.read_item::<Frame>();
        self.num_leaves = reader.read_u32();
        let num_portals = reader.read_u32() as usize;
        self.portals = (0..num_portals)
            .map(|_| reader.read_item::<BuildingPortal>())
            .collect();
        true
    }
}

impl IPackable for BuildingInfo {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.model_id);
        writer.write_item(&self.frame);
        writer.write_u32(self.num_leaves);
        writer.write_u32(self.portals.len() as u32);
        for portal in &self.portals {
            writer.write_item(portal);
        }
        true
    }
}
