use crate::Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PortalRef {
    pub poly_id: u16,
    pub portal_index: u16,
}

impl IUnpackable for PortalRef {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.poly_id = reader.read_u16();
        self.portal_index = reader.read_u16();
        true
    }
}

impl IPackable for PortalRef {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.poly_id);
        writer.write_u16(self.portal_index);
        true
    }
}
