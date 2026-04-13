use std::{
    collections::HashMap,
    io,
    sync::{Arc, Mutex},
};

use crate::Lib::IO::{
    BlockAllocators::IDatBlockAllocator::IDatBlockAllocator,
    DatBTree::{DatBTreeFile::DatBTreeFile, DatBTreeNode::DatBTreeNode},
    DatBinReader::DatBinReader,
    IUnpackable::IUnpackable,
};

pub struct DatBTreeReaderWriter {
    pub block_allocator: Arc<dyn IDatBlockAllocator>,
    node_cache: Mutex<HashMap<i32, DatBTreeNode>>,
    flat_index: Mutex<Option<HashMap<u32, DatBTreeFile>>>,
}

impl DatBTreeReaderWriter {
    pub fn new(block_allocator: Arc<dyn IDatBlockAllocator>) -> Self {
        Self {
            block_allocator,
            node_cache: Mutex::new(HashMap::new()),
            flat_index: Mutex::new(None),
        }
    }

    pub fn clear_cache(&self) {
        self.node_cache.lock().unwrap().clear();
        *self.flat_index.lock().unwrap() = None;
    }

    pub fn build_flat_index(&self) -> io::Result<()> {
        let mut map = HashMap::new();
        let root_block = self.block_allocator.header().root_block;
        if root_block != 0 {
            self.walk_collect(root_block, &mut map)?;
        }
        *self.flat_index.lock().unwrap() = Some(map);
        Ok(())
    }

    pub fn try_get_file(&self, file_id: u32) -> io::Result<Option<DatBTreeFile>> {
        if let Some(index) = &*self.flat_index.lock().unwrap() {
            if let Some(file) = index.get(&file_id) {
                return Ok(Some(*file));
            }
        }

        self.try_get_file_internal(file_id, self.block_allocator.header().root_block)
    }

    pub fn has_file(&self, file_id: u32) -> io::Result<bool> {
        Ok(self.try_get_file(file_id)?.is_some())
    }

    pub fn all_files(&self) -> io::Result<Vec<DatBTreeFile>> {
        let mut files = Vec::new();
        let root_block = self.block_allocator.header().root_block;
        if root_block != 0 {
            self.walk_all(root_block, &mut files)?;
        }
        Ok(files)
    }

    pub fn get_files_in_range(&self, first_id: u32, last_id: u32) -> io::Result<Vec<DatBTreeFile>> {
        if let Some(index) = &*self.flat_index.lock().unwrap() {
            let mut files: Vec<_> = index
                .iter()
                .filter_map(|(id, file)| (*id >= first_id && *id <= last_id).then_some(*file))
                .collect();
            files.sort_by_key(|file| file.id);
            return Ok(files);
        }

        let mut files = Vec::new();
        let root_block = self.block_allocator.header().root_block;
        if root_block != 0 {
            self.walk_range(root_block, first_id, last_id, &mut files)?;
        }
        Ok(files)
    }

    fn try_get_node(&self, block_offset: i32) -> io::Result<Option<DatBTreeNode>> {
        if block_offset == 0 || block_offset == 0xCDCDCDCD_u32 as i32 {
            return Ok(None);
        }

        if let Some(node) = self.node_cache.lock().unwrap().get(&block_offset).cloned() {
            return Ok(Some(node));
        }

        let mut buffer = vec![0_u8; DatBTreeNode::SIZE];
        self.block_allocator
            .read_block(&mut buffer, block_offset as usize)?;
        let mut node = DatBTreeNode::new(block_offset);
        let _ = node.unpack(&mut DatBinReader::new(&buffer));
        self.node_cache
            .lock()
            .unwrap()
            .insert(block_offset, node.clone());
        Ok(Some(node))
    }

    fn try_get_file_internal(
        &self,
        file_id: u32,
        starting_block: i32,
    ) -> io::Result<Option<DatBTreeFile>> {
        let mut current_block = starting_block;
        while current_block != 0 && current_block != 0xCDCDCDCD_u32 as i32 {
            let Some(node) = self.try_get_node(current_block)? else {
                break;
            };

            if node.file_count == 0 {
                break;
            }

            let mut left = 0_i32;
            let mut right = node.file_count as i32 - 1;
            let mut i = 0_i32;

            while left <= right {
                i = (left + right) / 2;
                let current_file = node.files[i as usize];
                if file_id == current_file.id {
                    return Ok(Some(current_file));
                } else if file_id < current_file.id {
                    right = i - 1;
                } else {
                    left = i + 1;
                }
            }

            if node.is_leaf() {
                break;
            }

            let branch_index = if file_id > node.files[i as usize].id {
                (i + 1) as usize
            } else {
                i as usize
            };

            if branch_index >= node.branch_count {
                break;
            }
            current_block = node.branches[branch_index];
        }

        Ok(None)
    }

    fn walk_collect(
        &self,
        starting_block: i32,
        map: &mut HashMap<u32, DatBTreeFile>,
    ) -> io::Result<()> {
        let Some(node) = self.try_get_node(starting_block)? else {
            return Ok(());
        };

        if node.is_leaf() {
            for i in 0..node.file_count {
                map.insert(node.files[i].id, node.files[i]);
            }
            return Ok(());
        }

        for i in 0..node.file_count {
            self.walk_collect(node.branches[i], map)?;
            map.insert(node.files[i].id, node.files[i]);
        }
        self.walk_collect(node.branches[node.file_count], map)
    }

    fn walk_all(&self, starting_block: i32, out: &mut Vec<DatBTreeFile>) -> io::Result<()> {
        let Some(node) = self.try_get_node(starting_block)? else {
            return Ok(());
        };

        if node.is_leaf() {
            for i in 0..node.file_count {
                out.push(node.files[i]);
            }
            return Ok(());
        }

        for i in 0..node.file_count {
            self.walk_all(node.branches[i], out)?;
            out.push(node.files[i]);
        }
        self.walk_all(node.branches[node.file_count], out)
    }

    fn walk_range(
        &self,
        starting_block: i32,
        first_id: u32,
        last_id: u32,
        out: &mut Vec<DatBTreeFile>,
    ) -> io::Result<()> {
        let Some(node) = self.try_get_node(starting_block)? else {
            return Ok(());
        };

        let mut i = 0_usize;
        while i < node.file_count && node.files[i].id < first_id {
            i += 1;
        }

        if !node.is_leaf() && i < node.branch_count {
            self.walk_range(node.branches[i], first_id, last_id, out)?;
        }

        while i < node.file_count && node.files[i].id <= last_id {
            out.push(node.files[i]);
            if !node.is_leaf() && i + 1 < node.branch_count {
                self.walk_range(node.branches[i + 1], first_id, last_id, out)?;
            }
            i += 1;
        }

        Ok(())
    }
}
