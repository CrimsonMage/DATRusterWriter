use crate::{
    Generated::Enums::PortalFlags::PortalFlags,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CellPortal {
    pub flags: PortalFlags,
    pub polygon_id: u16,
    pub other_cell_id: u16,
    pub other_portal_id: u16,
}

impl IUnpackable for CellPortal {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.flags = PortalFlags::from_bits_truncate(reader.read_u16());
        self.polygon_id = reader.read_u16();
        self.other_cell_id = reader.read_u16();
        self.other_portal_id = reader.read_u16();
        true
    }
}

impl IPackable for CellPortal {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.flags.bits());
        writer.write_u16(self.polygon_id);
        writer.write_u16(self.other_cell_id);
        writer.write_u16(self.other_portal_id);
        true
    }
}
