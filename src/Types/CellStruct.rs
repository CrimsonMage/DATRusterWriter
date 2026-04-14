use std::collections::BTreeMap;

use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{
        BSPTrees::{CellBSPTree, DrawingBSPTree, PhysicsBSPTree},
        Polygon::Polygon,
        VertexArray::VertexArray,
    },
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CellStruct {
    pub vertex_array: VertexArray,
    pub polygons: BTreeMap<u16, Polygon>,
    pub portals: Vec<u16>,
    pub cell_bsp: CellBSPTree,
    pub physics_polygons: BTreeMap<u16, Polygon>,
    pub physics_bsp: PhysicsBSPTree,
    pub drawing_bsp: Option<DrawingBSPTree>,
}

impl IUnpackable for CellStruct {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let num_polys = reader.read_u32() as usize;
        let num_physics_polys = reader.read_u32() as usize;
        let num_portals = reader.read_u32() as usize;
        self.vertex_array = reader.read_item::<VertexArray>();
        self.polygons.clear();
        for _ in 0..num_polys {
            let key = reader.read_u16();
            let value = reader.read_item::<Polygon>();
            self.polygons.insert(key, value);
        }
        self.portals = (0..num_portals).map(|_| reader.read_u16()).collect();
        reader.align(4);
        self.cell_bsp = reader.read_item::<CellBSPTree>();
        self.physics_polygons.clear();
        for _ in 0..num_physics_polys {
            let key = reader.read_u16();
            let value = reader.read_item::<Polygon>();
            self.physics_polygons.insert(key, value);
        }
        self.physics_bsp = reader.read_item::<PhysicsBSPTree>();
        self.drawing_bsp = if reader.read_bool(4) {
            Some(reader.read_item::<DrawingBSPTree>())
        } else {
            None
        };
        reader.align(4);
        true
    }
}

impl IPackable for CellStruct {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.polygons.len() as u32);
        writer.write_u32(self.physics_polygons.len() as u32);
        writer.write_u32(self.portals.len() as u32);
        writer.write_item(&self.vertex_array);
        for (key, value) in &self.polygons {
            writer.write_u16(*key);
            writer.write_item(value);
        }
        for portal in &self.portals {
            writer.write_u16(*portal);
        }
        writer.align(4);
        writer.write_item(&self.cell_bsp);
        for (key, value) in &self.physics_polygons {
            writer.write_u16(*key);
            writer.write_item(value);
        }
        writer.write_item(&self.physics_bsp);
        writer.write_bool(self.drawing_bsp.is_some(), 4);
        if let Some(drawing_bsp) = &self.drawing_bsp {
            writer.write_item(drawing_bsp);
        }
        writer.align(4);
        true
    }
}
