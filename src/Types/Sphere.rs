use crate::Lib::IO::{
    DatBinReader::DatBinReader,
    DatBinWriter::DatBinWriter,
    IPackable::IPackable,
    IUnpackable::IUnpackable,
    Numerics::Vector3,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Sphere {
    pub origin: Vector3,
    pub radius: f32,
}

impl IUnpackable for Sphere {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.origin = reader.read_vector3();
        self.radius = reader.read_single();
        true
    }
}

impl IPackable for Sphere {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_vector3(self.origin);
        writer.write_single(self.radius);
        true
    }
}
