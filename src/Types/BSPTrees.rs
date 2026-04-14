use crate::{
    Generated::Enums::BSPNodeType::BSPNodeType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable, Numerics::Plane,
    },
    Types::{PortalRef::PortalRef, Sphere::Sphere},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CellBSPTree {
    pub root: CellBSPNode,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsBSPTree {
    pub root: PhysicsBSPNode,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrawingBSPTree {
    pub root: DrawingBSPNode,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CellBSPNode {
    pub node_type: BSPNodeType,
    pub splitting_plane: Plane,
    pub pos_node: Option<Box<CellBSPNode>>,
    pub neg_node: Option<Box<CellBSPNode>>,
    pub leaf_index: i32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsBSPNode {
    pub node_type: BSPNodeType,
    pub splitting_plane: Plane,
    pub pos_node: Option<Box<PhysicsBSPNode>>,
    pub neg_node: Option<Box<PhysicsBSPNode>>,
    pub leaf_index: i32,
    pub solid: i32,
    pub bounding_sphere: Sphere,
    pub polygons: Vec<u16>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrawingBSPNode {
    pub node_type: BSPNodeType,
    pub splitting_plane: Plane,
    pub pos_node: Option<Box<DrawingBSPNode>>,
    pub neg_node: Option<Box<DrawingBSPNode>>,
    pub leaf_index: i32,
    pub bounding_sphere: Sphere,
    pub polygons: Vec<u16>,
    pub portals: Vec<PortalRef>,
}

fn read_pos_physics(node_type: BSPNodeType) -> bool {
    matches!(
        node_type,
        BSPNodeType::BPNN | BSPNodeType::BPIN_LOWER | BSPNodeType::BPIN | BSPNodeType::BPnN
    )
}

fn read_neg_physics(node_type: BSPNodeType) -> bool {
    matches!(
        node_type,
        BSPNodeType::BpIN | BSPNodeType::BpnN | BSPNodeType::BPIN | BSPNodeType::BPnN
    )
}

fn read_pos_drawing(node_type: BSPNodeType) -> bool {
    matches!(
        node_type,
        BSPNodeType::BPNN | BSPNodeType::BPIN_LOWER | BSPNodeType::BPIN | BSPNodeType::BPnN
    )
}

fn read_neg_drawing(node_type: BSPNodeType) -> bool {
    matches!(
        node_type,
        BSPNodeType::BpIN | BSPNodeType::BpnN | BSPNodeType::BPIN | BSPNodeType::BPnN
    )
}

impl IUnpackable for CellBSPTree {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.root = reader.read_item::<CellBSPNode>();
        true
    }
}

impl IPackable for CellBSPTree {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.root);
        true
    }
}

impl IUnpackable for PhysicsBSPTree {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.root = reader.read_item::<PhysicsBSPNode>();
        true
    }
}

impl IPackable for PhysicsBSPTree {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.root);
        true
    }
}

impl IUnpackable for DrawingBSPTree {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.root = reader.read_item::<DrawingBSPNode>();
        true
    }
}

impl IPackable for DrawingBSPTree {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.root);
        true
    }
}

impl IUnpackable for CellBSPNode {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.node_type = BSPNodeType::from(reader.read_u32() as i32);
        self.pos_node = None;
        self.neg_node = None;

        match self.node_type {
            BSPNodeType::PORTAL => false,
            BSPNodeType::LEAF => {
                self.leaf_index = reader.read_i32();
                true
            }
            _ => {
                self.splitting_plane = reader.read_plane();
                if read_pos_drawing(self.node_type) {
                    self.pos_node = Some(Box::new(reader.read_item::<CellBSPNode>()));
                }
                if read_neg_drawing(self.node_type) {
                    self.neg_node = Some(Box::new(reader.read_item::<CellBSPNode>()));
                }
                true
            }
        }
    }
}

impl IPackable for CellBSPNode {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(i32::from(self.node_type) as u32);
        match self.node_type {
            BSPNodeType::PORTAL => false,
            BSPNodeType::LEAF => {
                writer.write_i32(self.leaf_index);
                true
            }
            _ => {
                writer.write_plane(self.splitting_plane);
                if let Some(pos) = &self.pos_node {
                    writer.write_item(&**pos);
                }
                if let Some(neg) = &self.neg_node {
                    writer.write_item(&**neg);
                }
                true
            }
        }
    }
}

impl IUnpackable for PhysicsBSPNode {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.node_type = reader.read_i32().into();
        self.pos_node = None;
        self.neg_node = None;
        self.polygons.clear();

        match self.node_type {
            BSPNodeType::PORTAL => false,
            BSPNodeType::LEAF => {
                self.leaf_index = reader.read_i32();
                self.solid = reader.read_i32();
                self.bounding_sphere = reader.read_item::<Sphere>();
                let count = reader.read_u32() as usize;
                for _ in 0..count {
                    self.polygons.push(reader.read_u16());
                }
                true
            }
            _ => {
                self.splitting_plane = reader.read_plane();
                if read_pos_physics(self.node_type) {
                    self.pos_node = Some(Box::new(reader.read_item::<PhysicsBSPNode>()));
                }
                if read_neg_physics(self.node_type) {
                    self.neg_node = Some(Box::new(reader.read_item::<PhysicsBSPNode>()));
                }
                self.bounding_sphere = reader.read_item::<Sphere>();
                true
            }
        }
    }
}

impl IPackable for PhysicsBSPNode {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.node_type.into());

        match self.node_type {
            BSPNodeType::PORTAL => false,
            BSPNodeType::LEAF => {
                writer.write_i32(self.leaf_index);
                writer.write_i32(self.solid);
                writer.write_item(&self.bounding_sphere);
                writer.write_u32(self.polygons.len() as u32);
                for poly in &self.polygons {
                    writer.write_u16(*poly);
                }
                true
            }
            _ => {
                writer.write_plane(self.splitting_plane);
                if let Some(pos) = &self.pos_node {
                    writer.write_item(&**pos);
                }
                if let Some(neg) = &self.neg_node {
                    writer.write_item(&**neg);
                }
                writer.write_item(&self.bounding_sphere);
                true
            }
        }
    }
}

impl IUnpackable for DrawingBSPNode {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.node_type = reader.read_i32().into();
        self.pos_node = None;
        self.neg_node = None;
        self.polygons.clear();
        self.portals.clear();

        match self.node_type {
            BSPNodeType::PORTAL => {
                self.splitting_plane = reader.read_plane();
                self.pos_node = Some(Box::new(reader.read_item::<DrawingBSPNode>()));
                self.neg_node = Some(Box::new(reader.read_item::<DrawingBSPNode>()));
                self.bounding_sphere = reader.read_item::<Sphere>();
                let poly_count = reader.read_u32() as usize;
                let portal_count = reader.read_u32() as usize;
                for _ in 0..poly_count {
                    self.polygons.push(reader.read_u16());
                }
                for _ in 0..portal_count {
                    self.portals.push(reader.read_item::<PortalRef>());
                }
                true
            }
            BSPNodeType::LEAF => {
                self.leaf_index = reader.read_i32();
                true
            }
            _ => {
                self.splitting_plane = reader.read_plane();
                if read_pos_drawing(self.node_type) {
                    self.pos_node = Some(Box::new(reader.read_item::<DrawingBSPNode>()));
                }
                if read_neg_drawing(self.node_type) {
                    self.neg_node = Some(Box::new(reader.read_item::<DrawingBSPNode>()));
                }
                self.bounding_sphere = reader.read_item::<Sphere>();
                let poly_count = reader.read_u32() as usize;
                for _ in 0..poly_count {
                    self.polygons.push(reader.read_u16());
                }
                true
            }
        }
    }
}

impl IPackable for DrawingBSPNode {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.node_type.into());
        match self.node_type {
            BSPNodeType::PORTAL => {
                writer.write_plane(self.splitting_plane);
                if let Some(pos) = &self.pos_node {
                    writer.write_item(&**pos);
                }
                if let Some(neg) = &self.neg_node {
                    writer.write_item(&**neg);
                }
                writer.write_item(&self.bounding_sphere);
                writer.write_u32(self.polygons.len() as u32);
                writer.write_u32(self.portals.len() as u32);
                for poly in &self.polygons {
                    writer.write_u16(*poly);
                }
                for portal in &self.portals {
                    writer.write_item(portal);
                }
                true
            }
            BSPNodeType::LEAF => {
                writer.write_i32(self.leaf_index);
                true
            }
            _ => {
                writer.write_plane(self.splitting_plane);
                if let Some(pos) = &self.pos_node {
                    writer.write_item(&**pos);
                }
                if let Some(neg) = &self.neg_node {
                    writer.write_item(&**neg);
                }
                writer.write_item(&self.bounding_sphere);
                writer.write_u32(self.polygons.len() as u32);
                for poly in &self.polygons {
                    writer.write_u16(*poly);
                }
                true
            }
        }
    }
}
