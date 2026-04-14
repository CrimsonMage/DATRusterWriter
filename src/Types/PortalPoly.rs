use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PortalPoly {
    pub portal_index: i16,
    pub polygon_id: i16,
}

impl IUnpackable for PortalPoly {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.portal_index = reader.read_i16();
        self.polygon_id = reader.read_i16();
        true
    }
}

impl IPackable for PortalPoly {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i16(self.portal_index);
        writer.write_i16(self.polygon_id);
        true
    }
}
