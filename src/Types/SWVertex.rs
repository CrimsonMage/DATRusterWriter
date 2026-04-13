use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable, Numerics::Vector3,
    },
    Types::Vec2Duv::Vec2Duv,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SWVertex {
    pub origin: Vector3,
    pub normal: Vector3,
    pub uvs: Vec<Vec2Duv>,
}

impl IUnpackable for SWVertex {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let num_uvs = reader.read_u16() as usize;
        self.origin = reader.read_vector3();
        self.normal = reader.read_vector3();
        self.uvs.clear();
        for _ in 0..num_uvs {
            self.uvs.push(reader.read_item::<Vec2Duv>());
        }
        true
    }
}

impl IPackable for SWVertex {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u16(self.uvs.len() as u16);
        writer.write_vector3(self.origin);
        writer.write_vector3(self.normal);
        for uv in &self.uvs {
            writer.write_item(uv);
        }
        true
    }
}
