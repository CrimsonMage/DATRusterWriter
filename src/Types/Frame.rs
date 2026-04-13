use crate::Lib::IO::{
    DatBinReader::DatBinReader,
    DatBinWriter::DatBinWriter,
    IPackable::IPackable,
    IUnpackable::IUnpackable,
    Numerics::{Quaternion, Vector3},
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Frame {
    pub origin: Vector3,
    pub orientation: Quaternion,
}

impl IUnpackable for Frame {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.origin = reader.read_vector3();
        self.orientation = reader.read_quaternion();
        true
    }
}

impl IPackable for Frame {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_vector3(self.origin);
        writer.write_quaternion(self.orientation);
        true
    }
}
