use std::collections::BTreeMap;

use crate::{
    Generated::Enums::VertexType::VertexType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::SWVertex::SWVertex,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VertexArray {
    pub vertex_type: VertexType,
    pub vertices: BTreeMap<u16, SWVertex>,
}

impl IUnpackable for VertexArray {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.vertex_type = reader.read_i32().into();
        let count = reader.read_u32() as usize;
        self.vertices.clear();
        for _ in 0..count {
            self.vertices
                .insert(reader.read_u16(), reader.read_item::<SWVertex>());
        }
        true
    }
}

impl IPackable for VertexArray {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.vertex_type.into());
        writer.write_u32(self.vertices.len() as u32);
        for (key, value) in &self.vertices {
            writer.write_u16(*key);
            writer.write_item(value);
        }
        true
    }
}
