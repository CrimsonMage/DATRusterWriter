use std::{collections::HashMap, io, sync::Arc};

use dat_reader_writer::{
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{
        BlockAllocators::IDatBlockAllocator::IDatBlockAllocator,
        DatBTree::{
            DatBTreeFile::DatBTreeFile, DatBTreeNode::DatBTreeNode,
            DatBTreeReaderWriter::DatBTreeReaderWriter,
        },
        DatBinWriter::DatBinWriter,
        DatHeader::DatHeader,
        IPackable::IPackable,
    },
};
use uuid::Uuid;

struct MockBlockAllocator {
    header: DatHeader,
    blocks: HashMap<usize, Vec<u8>>,
}

impl MockBlockAllocator {
    fn new(root_block: i32, blocks: HashMap<usize, Vec<u8>>) -> Self {
        let mut header = DatHeader::new(
            DatFileType::Portal,
            0,
            2048,
            Some("test".to_string()),
            1,
            1,
            Uuid::nil(),
            1,
        );
        header.root_block = root_block;
        Self { header, blocks }
    }
}

impl IDatBlockAllocator for MockBlockAllocator {
    fn can_write(&self) -> bool {
        false
    }
    fn has_header_data(&self) -> bool {
        true
    }
    fn header(&self) -> &DatHeader {
        &self.header
    }
    fn read_bytes(
        &self,
        buffer: &mut [u8],
        buffer_offset: usize,
        byte_offset: usize,
        num_bytes: usize,
    ) -> io::Result<()> {
        let block = self
            .blocks
            .get(&byte_offset)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing raw block"))?;
        buffer[buffer_offset..buffer_offset + num_bytes].copy_from_slice(&block[..num_bytes]);
        Ok(())
    }
    fn read_block(&self, buffer: &mut [u8], starting_block: usize) -> io::Result<()> {
        let block = self
            .blocks
            .get(&starting_block)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing block"))?;
        buffer[..block.len()].copy_from_slice(block);
        Ok(())
    }
}

fn encode_node(node: &DatBTreeNode) -> Vec<u8> {
    let mut bytes = vec![0_u8; DatBTreeNode::SIZE];
    assert!(node.pack(&mut DatBinWriter::new(&mut bytes)));
    bytes
}

fn sample_file(id: u32, offset: i32, size: u32) -> DatBTreeFile {
    DatBTreeFile {
        version: 2,
        id,
        offset,
        size,
        iteration: 1,
        ..DatBTreeFile::default()
    }
}

#[test]
fn btree_node_pack_unpack_roundtrip_for_leaf_and_branching_metadata() {
    let mut node = DatBTreeNode::new(100);
    node.branch_count = 2;
    node.branches[0] = 200;
    node.branches[1] = 300;
    node.file_count = 1;
    node.files[0] = sample_file(0x1000, 400, 12);

    let bytes = encode_node(&node);
    let mut unpacked = DatBTreeNode::new(100);
    let ok = dat_reader_writer::Lib::IO::IUnpackable::IUnpackable::unpack(
        &mut unpacked,
        &mut dat_reader_writer::Lib::IO::DatBinReader::DatBinReader::new(&bytes),
    );
    assert!(ok);
    assert_eq!(1, unpacked.file_count);
    assert_eq!(2, unpacked.branch_count);
    assert_eq!(200, unpacked.branches[0]);
    assert_eq!(300, unpacked.branches[1]);
    assert_eq!(0x1000, unpacked.files[0].id);
}

#[test]
fn btree_try_get_file_and_range_walk_across_branches() {
    let mut left = DatBTreeNode::new(1000);
    left.file_count = 2;
    left.files[0] = sample_file(10, 10, 1);
    left.files[1] = sample_file(20, 20, 1);

    let mut right = DatBTreeNode::new(2000);
    right.file_count = 2;
    right.files[0] = sample_file(40, 40, 1);
    right.files[1] = sample_file(50, 50, 1);

    let mut root = DatBTreeNode::new(500);
    root.branch_count = 2;
    root.branches[0] = 1000;
    root.branches[1] = 2000;
    root.file_count = 1;
    root.files[0] = sample_file(30, 30, 1);

    let mut blocks = HashMap::new();
    blocks.insert(500usize, encode_node(&root));
    blocks.insert(1000usize, encode_node(&left));
    blocks.insert(2000usize, encode_node(&right));

    let allocator = Arc::new(MockBlockAllocator::new(500, blocks));
    let tree = DatBTreeReaderWriter::new(allocator);

    let found = tree.try_get_file(40).unwrap().unwrap();
    assert_eq!(40, found.id);
    assert!(tree.has_file(20).unwrap());
    assert!(!tree.has_file(999).unwrap());

    let all_ids: Vec<u32> = tree
        .all_files()
        .unwrap()
        .into_iter()
        .map(|f| f.id)
        .collect();
    assert_eq!(vec![10, 20, 30, 40, 50], all_ids);

    let range_ids: Vec<u32> = tree
        .get_files_in_range(15, 45)
        .unwrap()
        .into_iter()
        .map(|f| f.id)
        .collect();
    assert_eq!(vec![20, 30, 40], range_ids);
}

#[test]
fn btree_flat_index_preserves_sorted_range_results() {
    let mut root = DatBTreeNode::new(500);
    root.file_count = 3;
    root.files[0] = sample_file(5, 5, 1);
    root.files[1] = sample_file(15, 15, 1);
    root.files[2] = sample_file(25, 25, 1);

    let mut blocks = HashMap::new();
    blocks.insert(500usize, encode_node(&root));

    let allocator = Arc::new(MockBlockAllocator::new(500, blocks));
    let tree = DatBTreeReaderWriter::new(allocator);
    tree.build_flat_index().unwrap();

    let ids: Vec<u32> = tree
        .get_files_in_range(1, 20)
        .unwrap()
        .into_iter()
        .map(|f| f.id)
        .collect();
    assert_eq!(vec![5, 15], ids);
}
