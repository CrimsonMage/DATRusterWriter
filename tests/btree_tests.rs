use std::{
    collections::HashMap,
    io,
    sync::{Arc, Mutex},
};

use futures::executor::block_on;

use dat_ruster_writer::{
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

struct ReadOnlyMockBlockAllocator {
    header: DatHeader,
    blocks: HashMap<usize, Vec<u8>>,
}

impl ReadOnlyMockBlockAllocator {
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

impl IDatBlockAllocator for ReadOnlyMockBlockAllocator {
    fn can_write(&self) -> bool {
        false
    }
    fn has_header_data(&self) -> bool {
        true
    }
    fn header(&self) -> DatHeader {
        self.header.clone()
    }
    fn init_new(
        &self,
        _file_type: DatFileType,
        _subset: u32,
        _block_size: i32,
        _num_blocks_to_allocate: i32,
    ) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "read-only"))
    }
    fn set_version(
        &self,
        _version: &str,
        _engine_version: i32,
        _game_version: i32,
        _major_version: Uuid,
        _minor_version: u32,
    ) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "read-only"))
    }
    fn write_bytes(
        &self,
        _buffer: &[u8],
        _byte_offset: usize,
        _num_bytes: usize,
    ) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "read-only"))
    }
    fn write_block(
        &self,
        _buffer: &[u8],
        _num_bytes: usize,
        _starting_block: i32,
    ) -> io::Result<i32> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "read-only"))
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
    fn try_get_block_offsets(&self, _starting_block: i32) -> io::Result<Option<Vec<i32>>> {
        Ok(None)
    }
    fn allocate_empty_blocks(&self, _num_blocks_to_allocate: i32) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "read-only"))
    }
    fn reserve_block(&self) -> io::Result<i32> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "read-only"))
    }
    fn set_root_block(&self, _offset: i32) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "read-only"))
    }
}

struct WritableState {
    header: DatHeader,
    blocks: HashMap<i32, Vec<u8>>,
    next_block: i32,
}

struct WritableMockBlockAllocator {
    state: Mutex<WritableState>,
}

impl WritableMockBlockAllocator {
    fn empty() -> Self {
        let header = DatHeader::new(
            DatFileType::Portal,
            0,
            2048,
            Some("test".to_string()),
            1,
            1,
            Uuid::nil(),
            1,
        );
        Self {
            state: Mutex::new(WritableState {
                header,
                blocks: HashMap::new(),
                next_block: 2048,
            }),
        }
    }

    fn with_nodes(root: DatBTreeNode, nodes: Vec<DatBTreeNode>) -> Self {
        let allocator = Self::empty();
        {
            let mut state = allocator.state.lock().unwrap();
            state.header.root_block = root.offset;
            state.next_block = root.offset + state.header.block_size;
            state.blocks.insert(root.offset, encode_node(&root));
            for node in nodes {
                state.next_block = state.next_block.max(node.offset + state.header.block_size);
                state.blocks.insert(node.offset, encode_node(&node));
            }
        }
        allocator
    }

    fn with_root(root: DatBTreeNode) -> Self {
        Self::with_nodes(root, Vec::new())
    }
}

impl IDatBlockAllocator for WritableMockBlockAllocator {
    fn can_write(&self) -> bool {
        true
    }
    fn has_header_data(&self) -> bool {
        true
    }
    fn header(&self) -> DatHeader {
        self.state.lock().unwrap().header.clone()
    }
    fn init_new(
        &self,
        file_type: DatFileType,
        subset: u32,
        block_size: i32,
        num_blocks_to_allocate: i32,
    ) -> io::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.header = DatHeader::new(file_type, subset, block_size, None, 0, 0, Uuid::nil(), 0);
        state.next_block = ((DatHeader::SIZE as i32 + block_size - 1) / block_size) * block_size;
        if num_blocks_to_allocate > 0 {
            state.header.file_size = state.next_block + num_blocks_to_allocate * block_size;
        }
        Ok(())
    }
    fn set_version(
        &self,
        version: &str,
        engine_version: i32,
        game_version: i32,
        major_version: Uuid,
        minor_version: u32,
    ) -> io::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.header.version = Some(version.to_string());
        state.header.engine_version = engine_version;
        state.header.game_version = game_version;
        state.header.major_version = major_version;
        state.header.minor_version = minor_version;
        Ok(())
    }
    fn write_bytes(&self, buffer: &[u8], byte_offset: usize, num_bytes: usize) -> io::Result<()> {
        let mut state = self.state.lock().unwrap();
        state
            .blocks
            .insert(byte_offset as i32, buffer[..num_bytes].to_vec());
        Ok(())
    }
    fn write_block(&self, buffer: &[u8], num_bytes: usize, starting_block: i32) -> io::Result<i32> {
        let mut state = self.state.lock().unwrap();
        let offset = if starting_block > 0 {
            starting_block
        } else {
            let offset = state.next_block;
            state.next_block += state.header.block_size;
            state.header.file_size = state.header.file_size.max(state.next_block);
            offset
        };
        let mut block = vec![0u8; num_bytes];
        block[..num_bytes].copy_from_slice(&buffer[..num_bytes]);
        state.blocks.insert(offset, block);
        Ok(offset)
    }
    fn read_bytes(
        &self,
        buffer: &mut [u8],
        buffer_offset: usize,
        byte_offset: usize,
        num_bytes: usize,
    ) -> io::Result<()> {
        let state = self.state.lock().unwrap();
        let block = state
            .blocks
            .get(&(byte_offset as i32))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing raw block"))?;
        buffer[buffer_offset..buffer_offset + num_bytes].copy_from_slice(&block[..num_bytes]);
        Ok(())
    }
    fn read_block(&self, buffer: &mut [u8], starting_block: usize) -> io::Result<()> {
        let state = self.state.lock().unwrap();
        let block = state
            .blocks
            .get(&(starting_block as i32))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing block"))?;
        buffer[..block.len()].copy_from_slice(block);
        Ok(())
    }
    fn try_get_block_offsets(&self, starting_block: i32) -> io::Result<Option<Vec<i32>>> {
        Ok((starting_block > 0).then_some(vec![starting_block]))
    }
    fn allocate_empty_blocks(&self, num_blocks_to_allocate: i32) -> io::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.header.file_size += num_blocks_to_allocate * state.header.block_size;
        Ok(())
    }
    fn reserve_block(&self) -> io::Result<i32> {
        let mut state = self.state.lock().unwrap();
        let offset = state.next_block;
        state.next_block += state.header.block_size;
        state.header.file_size = state.header.file_size.max(state.next_block);
        Ok(offset)
    }
    fn set_root_block(&self, offset: i32) -> io::Result<()> {
        self.state.lock().unwrap().header.root_block = offset;
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
    let ok = dat_ruster_writer::Lib::IO::IUnpackable::IUnpackable::unpack(
        &mut unpacked,
        &mut dat_ruster_writer::Lib::IO::DatBinReader::DatBinReader::new(&bytes),
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

    let allocator = Arc::new(ReadOnlyMockBlockAllocator::new(500, blocks));
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
fn block_allocator_async_wrappers_match_sync_behavior() {
    let mut root = DatBTreeNode::new(500);
    root.file_count = 1;
    root.files[0] = sample_file(15, 15, 1);

    let mut blocks = HashMap::new();
    blocks.insert(500usize, encode_node(&root));

    let read_allocator = ReadOnlyMockBlockAllocator::new(500, blocks);
    let mut read_buffer = vec![0u8; DatBTreeNode::SIZE];
    block_on(read_allocator.read_block_async(&mut read_buffer, 500)).unwrap();
    assert_eq!(encode_node(&root), read_buffer);

    let write_allocator = WritableMockBlockAllocator::empty();
    let payload = [0x11, 0x22, 0x33, 0x44];
    let written_offset = block_on(write_allocator.write_block_async(&payload, payload.len(), 0)).unwrap();
    assert!(written_offset > 0);

    let mut roundtrip = vec![0u8; payload.len()];
    write_allocator
        .read_block(&mut roundtrip, written_offset as usize)
        .unwrap();
    assert_eq!(payload, roundtrip.as_slice());
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

    let allocator = Arc::new(ReadOnlyMockBlockAllocator::new(500, blocks));
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

#[test]
fn btree_insert_creates_root_for_empty_tree() {
    let allocator = Arc::new(WritableMockBlockAllocator::empty());
    let tree = DatBTreeReaderWriter::new(allocator.clone());

    let inserted = sample_file(0x1000, 0x2000, 64);
    assert!(tree.insert(inserted).unwrap().is_none());

    assert_eq!(inserted, tree.try_get_file(0x1000).unwrap().unwrap());
    assert!(allocator.header().root_block != 0);
}

#[test]
fn btree_insert_replaces_existing_entry() {
    let mut root = DatBTreeNode::new(2048);
    root.add_file(sample_file(0x1000, 0x2000, 64));

    let allocator = Arc::new(WritableMockBlockAllocator::with_root(root));
    let tree = DatBTreeReaderWriter::new(allocator);

    let replacement = sample_file(0x1000, 0x3000, 128);
    let replaced = tree.insert(replacement).unwrap().unwrap();

    assert_eq!(0x2000, replaced.offset);
    assert_eq!(128, tree.try_get_file(0x1000).unwrap().unwrap().size);
}

#[test]
fn btree_insert_splits_full_root() {
    let mut root = DatBTreeNode::new(2048);
    for i in 0..DatBTreeReaderWriter::MAX_ITEMS {
        root.add_file(sample_file((i as u32 + 1) * 10, 1000 + i as i32, 1));
    }

    let allocator = Arc::new(WritableMockBlockAllocator::with_root(root));
    let tree = DatBTreeReaderWriter::new(allocator.clone());
    let inserted = sample_file(9999, 3333, 2);
    assert!(tree.insert(inserted).unwrap().is_none());

    assert_eq!(
        DatBTreeReaderWriter::MAX_ITEMS + 1,
        tree.all_files().unwrap().len()
    );
    assert_eq!(inserted, tree.try_get_file(9999).unwrap().unwrap());
    assert_ne!(2048, allocator.header().root_block);
}

#[test]
fn btree_insert_updates_existing_flat_index() {
    let mut root = DatBTreeNode::new(2048);
    root.add_file(sample_file(10, 1010, 1));
    root.add_file(sample_file(30, 1030, 1));

    let allocator = Arc::new(WritableMockBlockAllocator::with_root(root));
    let tree = DatBTreeReaderWriter::new(allocator);
    tree.build_flat_index().unwrap();

    let inserted = sample_file(20, 1020, 1);
    assert!(tree.insert(inserted).unwrap().is_none());

    assert_eq!(inserted, tree.try_get_file(20).unwrap().unwrap());
    let ids: Vec<u32> = tree
        .get_files_in_range(1, 40)
        .unwrap()
        .into_iter()
        .map(|file| file.id)
        .collect();
    assert_eq!(vec![10, 20, 30], ids);
}

#[test]
fn btree_async_wrappers_cover_lookup_insert_and_delete() {
    let allocator = Arc::new(WritableMockBlockAllocator::empty());
    let tree = DatBTreeReaderWriter::new(allocator.clone());

    let inserted = sample_file(0x1010, 0x2020, 64);
    assert!(block_on(tree.insert_async(inserted)).unwrap().is_none());

    let found = block_on(tree.try_get_file_async(0x1010)).unwrap().unwrap();
    assert_eq!(inserted, found);

    let deleted = block_on(tree.try_delete_async(0x1010)).unwrap().unwrap();
    assert_eq!(0x1010, deleted.id);
    assert!(block_on(tree.try_get_file_async(0x1010)).unwrap().is_none());
}

#[test]
fn btree_delete_removes_leaf_entry() {
    let mut root = DatBTreeNode::new(2048);
    root.add_file(sample_file(10, 1010, 1));
    root.add_file(sample_file(20, 1020, 1));
    root.add_file(sample_file(30, 1030, 1));

    let allocator = Arc::new(WritableMockBlockAllocator::with_root(root));
    let tree = DatBTreeReaderWriter::new(allocator);

    let deleted = tree.try_delete(20).unwrap().unwrap();
    assert_eq!(20, deleted.id);
    assert!(tree.try_get_file(20).unwrap().is_none());
    let ids: Vec<u32> = tree
        .all_files()
        .unwrap()
        .into_iter()
        .map(|file| file.id)
        .collect();
    assert_eq!(vec![10, 30], ids);
}

#[test]
fn btree_delete_can_collapse_empty_root_to_child() {
    let mut left = DatBTreeNode::new(4096);
    for i in 0..DatBTreeReaderWriter::MIN_ITEMS {
        left.add_file(sample_file((i as u32 + 1) * 10, 2000 + i as i32, 1));
    }

    let mut right = DatBTreeNode::new(6144);
    for i in 0..DatBTreeReaderWriter::MIN_ITEMS {
        right.add_file(sample_file(1000 + (i as u32 + 1) * 10, 3000 + i as i32, 1));
    }

    let middle = sample_file(500, 2500, 1);
    let mut root = DatBTreeNode::new(2048);
    root.add_branch(left.offset);
    root.add_file(middle);
    root.add_branch(right.offset);

    let allocator = Arc::new(WritableMockBlockAllocator::with_nodes(
        root,
        vec![left, right],
    ));
    let tree = DatBTreeReaderWriter::new(allocator.clone());

    let deleted = tree.try_delete(500).unwrap().unwrap();
    assert_eq!(500, deleted.id);
    assert!(tree.try_get_file(500).unwrap().is_none());
    assert_ne!(2048, allocator.header().root_block);
    assert_eq!(
        DatBTreeReaderWriter::MIN_ITEMS * 2,
        tree.all_files().unwrap().len()
    );
}

#[test]
fn btree_delete_updates_existing_flat_index() {
    let mut root = DatBTreeNode::new(2048);
    root.add_file(sample_file(10, 1010, 1));
    root.add_file(sample_file(20, 1020, 1));
    root.add_file(sample_file(30, 1030, 1));

    let allocator = Arc::new(WritableMockBlockAllocator::with_root(root));
    let tree = DatBTreeReaderWriter::new(allocator);
    tree.build_flat_index().unwrap();

    let deleted = tree.try_delete(20).unwrap().unwrap();
    assert_eq!(20, deleted.id);
    assert!(tree.try_get_file(20).unwrap().is_none());

    let ids: Vec<u32> = tree
        .get_files_in_range(1, 40)
        .unwrap()
        .into_iter()
        .map(|file| file.id)
        .collect();
    assert_eq!(vec![10, 30], ids);
}
