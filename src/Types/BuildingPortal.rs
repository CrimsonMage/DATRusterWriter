use crate::{
    Generated::Enums::PortalFlags::PortalFlags,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuildingPortal {
    pub flags: PortalFlags,
    pub other_cell_id: u16,
    pub other_portal_id: u16,
    pub stab_list: Vec<u16>,
}

impl IUnpackable for BuildingPortal {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.flags = PortalFlags::from_bits_truncate(reader.read_u16());
        self.other_cell_id = reader.read_u16();
        self.other_portal_id = reader.read_u16();
        let num_stabs = reader.read_u16() as usize;
        self.stab_list = (0..num_stabs).map(|_| reader.read_u16()).collect();
        reader.align(4);
        true
    }
}

impl IPackable for BuildingPortal {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.flags.bits());
        writer.write_u16(self.other_cell_id);
        writer.write_u16(self.other_portal_id);
        writer.write_u16(self.stab_list.len() as u16);
        for value in &self.stab_list {
            writer.write_u16(*value);
        }
        writer.align(4);
        true
    }
}
