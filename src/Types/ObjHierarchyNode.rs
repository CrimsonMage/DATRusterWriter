use crate::{
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::ObfuscatedPStringBase::ObfuscatedPStringBase,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjHierarchyNode {
    pub menu_name: ObfuscatedPStringBase,
    pub wcid: u32,
    pub children: Vec<ObjHierarchyNode>,
}

impl IUnpackable for ObjHierarchyNode {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.menu_name = reader.read_item::<ObfuscatedPStringBase>();
        reader.align(4);
        self.wcid = reader.read_u32();
        let count = reader.read_i32().max(0) as usize;
        self.children.clear();
        self.children.reserve(count);
        for _ in 0..count {
            self.children.push(reader.read_item::<ObjHierarchyNode>());
        }
        reader.align(4);
        true
    }
}

impl IPackable for ObjHierarchyNode {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.menu_name);
        writer.align(4);
        writer.write_u32(self.wcid);
        writer.write_i32(self.children.len() as i32);
        for item in &self.children {
            writer.write_item(item);
        }
        writer.align(4);
        true
    }
}
