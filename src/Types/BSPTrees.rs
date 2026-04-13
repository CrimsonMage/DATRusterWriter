use crate::{
    Generated::Enums::BSPNodeType::BSPNodeType,
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable, Numerics::Plane},
    Types::{PortalRef::PortalRef, Sphere::Sphere},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsBSPTree {
    pub root: PhysicsBSPNode,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrawingBSPTree {
    pub root: DrawingBSPNode,
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

fn reads_pos(node_type: BSPNodeType) -> bool {
    matches!(node_type, BSPNodeType::BPNN | BSPNodeType::BPIN_LOWER | BSPNodeType::BPIN | BSPNodeType::BPNN_UPPER | BSPNodeType::PORTAL)
}

fn reads_neg(node_type: BSPNodeType) -> bool {
    matches!(node_type, BSPNodeType::BPIN_UPPER_ALT | BSPNodeType::BPNN_ALT | BSPNodeType::BPIN | BSPNodeType::BPNN_UPPER | BSPNodeType::PORTAL)
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

impl IUnpackable for PhysicsBSPNode {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.node_type = reader.read_i32().into();
        self.pos_node = None;
        self.neg_node = None;
        self.polygons.clear();

        if self.node_type == BSPNodeType::LEAF {
            self.leaf_index = reader.read_i32();
            self.solid = reader.read_i32();
            self.bounding_sphere = reader.read_item::<Sphere>();
            let count = reader.read_u32() as usize;
            for _ in 0..count {
                self.polygons.push(reader.read_u16());
            }
        } else {
            self.splitting_plane = reader.read_plane();
            if reads_pos(self.node_type) && self.node_type != BSPNodeType::PORTAL {
                self.pos_node = Some(Box::new(reader.read_item::<PhysicsBSPNode>()));
            }
            if reads_neg(self.node_type) && self.node_type != BSPNodeType::PORTAL {
                self.neg_node = Some(Box::new(reader.read_item::<PhysicsBSPNode>()));
            }
            self.bounding_sphere = reader.read_item::<Sphere>();
        }
        true
    }
}

impl IPackable for PhysicsBSPNode {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.node_type.into());
        if self.node_type == BSPNodeType::LEAF {
            writer.write_i32(self.leaf_index);
            writer.write_i32(self.solid);
            writer.write_item(&self.bounding_sphere);
            writer.write_u32(self.polygons.len() as u32);
            for poly in &self.polygons {
                writer.write_u16(*poly);
            }
        } else {
            writer.write_plane(self.splitting_plane);
            if let Some(pos) = &self.pos_node { writer.write_item(&**pos); }
            if let Some(neg) = &self.neg_node { writer.write_item(&**neg); }
            writer.write_item(&self.bounding_sphere);
        }
        true
    }
}

impl IUnpackable for DrawingBSPNode {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.node_type = reader.read_i32().into();
        self.pos_node = None;
        self.neg_node = None;
        self.polygons.clear();
        self.portals.clear();

        if self.node_type == BSPNodeType::PORTAL {
            self.splitting_plane = reader.read_plane();
            self.pos_node = Some(Box::new(reader.read_item::<DrawingBSPNode>()));
            self.neg_node = Some(Box::new(reader.read_item::<DrawingBSPNode>()));
            self.bounding_sphere = reader.read_item::<Sphere>();
            let poly_count = reader.read_u32() as usize;
            let portal_count = reader.read_u32() as usize;
            for _ in 0..poly_count { self.polygons.push(reader.read_u16()); }
            for _ in 0..portal_count { self.portals.push(reader.read_item::<PortalRef>()); }
        } else if self.node_type == BSPNodeType::LEAF {
            self.leaf_index = reader.read_i32();
        } else {
            self.splitting_plane = reader.read_plane();
            if reads_pos(self.node_type) {
                self.pos_node = Some(Box::new(reader.read_item::<DrawingBSPNode>()));
            }
            if reads_neg(self.node_type) {
                self.neg_node = Some(Box::new(reader.read_item::<DrawingBSPNode>()));
            }
            self.bounding_sphere = reader.read_item::<Sphere>();
            let poly_count = reader.read_u32() as usize;
            for _ in 0..poly_count { self.polygons.push(reader.read_u16()); }
        }
        true
    }
}

impl IPackable for DrawingBSPNode {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_i32(self.node_type.into());
        if self.node_type == BSPNodeType::PORTAL {
            writer.write_plane(self.splitting_plane);
            if let Some(pos) = &self.pos_node { writer.write_item(&**pos); }
            if let Some(neg) = &self.neg_node { writer.write_item(&**neg); }
            writer.write_item(&self.bounding_sphere);
            writer.write_u32(self.polygons.len() as u32);
            writer.write_u32(self.portals.len() as u32);
            for poly in &self.polygons { writer.write_u16(*poly); }
            for portal in &self.portals { writer.write_item(portal); }
        } else if self.node_type == BSPNodeType::LEAF {
            writer.write_i32(self.leaf_index);
        } else {
            writer.write_plane(self.splitting_plane);
            if let Some(pos) = &self.pos_node { writer.write_item(&**pos); }
            if let Some(neg) = &self.neg_node { writer.write_item(&**neg); }
            writer.write_item(&self.bounding_sphere);
            writer.write_u32(self.polygons.len() as u32);
            for poly in &self.polygons { writer.write_u16(*poly); }
        }
        true
    }
}
