use std::{
    collections::HashMap,
    future::Future,
    io,
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::Lib::IO::{
    BlockAllocators::IDatBlockAllocator::IDatBlockAllocator,
    DatBTree::{DatBTreeFile::DatBTreeFile, DatBTreeNode::DatBTreeNode},
    DatBinReader::DatBinReader,
    DatBinWriter::DatBinWriter,
    IPackable::IPackable,
    IUnpackable::IUnpackable,
};

pub struct DatBTreeReaderWriter {
    pub block_allocator: Arc<dyn IDatBlockAllocator>,
    node_cache: Mutex<HashMap<i32, DatBTreeNode>>,
    flat_index: Mutex<Option<HashMap<u32, DatBTreeFile>>>,
}

impl DatBTreeReaderWriter {
    pub const DEGREE: usize = 31;
    pub const MIN_ITEMS: usize = Self::DEGREE - 1;
    pub const MAX_ITEMS: usize = Self::DEGREE * 2 - 1;

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

    pub fn try_get_file_async<'a>(
        &'a self,
        file_id: u32,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<DatBTreeFile>>> + Send + 'a>> {
        Box::pin(async move {
            if let Some(index) = &*self.flat_index.lock().unwrap() {
                if let Some(file) = index.get(&file_id) {
                    return Ok(Some(*file));
                }
            }

            self.try_get_file_internal_async(file_id, self.block_allocator.header().root_block)
                .await
        })
    }

    pub fn insert(&self, file: DatBTreeFile) -> io::Result<Option<DatBTreeFile>> {
        if !self.block_allocator.can_write() {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "B-tree insert requires a writable allocator",
            ));
        }

        if self.block_allocator.header().root_block == 0 {
            let mut root = DatBTreeNode::new(self.block_allocator.reserve_block()?);
            root.add_file(file);
            self.set_new_root(root)?;
            if let Some(index) = &mut *self.flat_index.lock().unwrap() {
                index.insert(file.id, file);
            }
            return Ok(None);
        }

        if self.try_get_file(file.id)?.is_some() {
            let root = self.try_get_node(self.block_allocator.header().root_block)?;
            if let Some(root) = root {
                if let Some(replaced) = self.try_find_parent_and_update(root, file)? {
                    if let Some(index) = &mut *self.flat_index.lock().unwrap() {
                        index.insert(file.id, file);
                    }
                    return Ok(Some(replaced));
                }
            }
        }

        let mut root = self
            .try_get_node(self.block_allocator.header().root_block)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "root node not found"))?;

        if root.file_count == Self::MAX_ITEMS {
            let old_root = root.clone();
            let mut new_root = DatBTreeNode::new(self.block_allocator.reserve_block()?);
            self.set_new_root(new_root.clone())?;
            new_root.add_branch(old_root.offset);
            self.split_child(&mut new_root, &mut root)?;
            self.insert_non_full(&mut new_root, file)?;
        } else {
            self.insert_non_full(&mut root, file)?;
        }

        if let Some(index) = &mut *self.flat_index.lock().unwrap() {
            index.insert(file.id, file);
        }
        Ok(None)
    }

    pub fn insert_async<'a>(
        &'a self,
        file: DatBTreeFile,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<DatBTreeFile>>> + Send + 'a>> {
        Box::pin(async move {
            if !self.block_allocator.can_write() {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "B-tree insert requires a writable allocator",
                ));
            }

            if self.block_allocator.header().root_block == 0 {
                let mut root = DatBTreeNode::new(self.block_allocator.reserve_block()?);
                root.add_file(file);
                self.set_new_root_async(root).await?;
                if let Some(index) = &mut *self.flat_index.lock().unwrap() {
                    index.insert(file.id, file);
                }
                return Ok(None);
            }

            if self.try_get_file_async(file.id).await?.is_some() {
                let root = self
                    .try_get_node_async(self.block_allocator.header().root_block)
                    .await?;
                if let Some(root) = root {
                    if let Some(replaced) =
                        self.try_find_parent_and_update_async(root, file).await?
                    {
                        if let Some(index) = &mut *self.flat_index.lock().unwrap() {
                            index.insert(file.id, file);
                        }
                        return Ok(Some(replaced));
                    }
                }
            }

            let mut root = self
                .try_get_node_async(self.block_allocator.header().root_block)
                .await?
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "root node not found"))?;

            if root.file_count == Self::MAX_ITEMS {
                let old_root = root.clone();
                let mut new_root = DatBTreeNode::new(self.block_allocator.reserve_block()?);
                self.set_new_root_async(new_root.clone()).await?;
                new_root.add_branch(old_root.offset);
                self.split_child_async(&mut new_root, &mut root).await?;
                self.insert_non_full_async(&mut new_root, file).await?;
            } else {
                self.insert_non_full_async(&mut root, file).await?;
            }

            if let Some(index) = &mut *self.flat_index.lock().unwrap() {
                index.insert(file.id, file);
            }
            Ok(None)
        })
    }

    pub fn try_delete(&self, file_id: u32) -> io::Result<Option<DatBTreeFile>> {
        if !self.block_allocator.can_write() {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "B-tree delete requires a writable allocator",
            ));
        }

        let Some(root) = self.try_get_node(self.block_allocator.header().root_block)? else {
            return Ok(None);
        };

        let Some(file_entry) = self.try_get_file(file_id)? else {
            return Ok(None);
        };

        self.delete_internal(root, file_id)?;

        if let Some(root_after) = self.try_get_node(self.block_allocator.header().root_block)? {
            if root_after.file_count == 0 && !root_after.is_leaf() {
                if let Some(branch) = self.try_get_node(root_after.branches[0])? {
                    self.set_new_root(branch)?;
                }
            }
        }

        if let Some(index) = &mut *self.flat_index.lock().unwrap() {
            index.remove(&file_id);
        }

        Ok(Some(file_entry))
    }

    pub fn try_delete_async<'a>(
        &'a self,
        file_id: u32,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<DatBTreeFile>>> + Send + 'a>> {
        Box::pin(async move {
            if !self.block_allocator.can_write() {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "B-tree delete requires a writable allocator",
                ));
            }

            let Some(root) = self
                .try_get_node_async(self.block_allocator.header().root_block)
                .await?
            else {
                return Ok(None);
            };

            let Some(file_entry) = self.try_get_file_async(file_id).await? else {
                return Ok(None);
            };

            self.delete_internal_async(root, file_id).await?;

            if let Some(root_after) = self
                .try_get_node_async(self.block_allocator.header().root_block)
                .await?
            {
                if root_after.file_count == 0 && !root_after.is_leaf() {
                    if let Some(branch) = self.try_get_node_async(root_after.branches[0]).await? {
                        self.set_new_root_async(branch).await?;
                    }
                }
            }

            if let Some(index) = &mut *self.flat_index.lock().unwrap() {
                index.remove(&file_id);
            }

            Ok(Some(file_entry))
        })
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

    fn write_node(&self, node: &mut DatBTreeNode) -> io::Result<()> {
        let mut buffer = vec![0_u8; DatBTreeNode::SIZE];
        let mut writer = DatBinWriter::new(&mut buffer);
        let _ = node.pack(&mut writer);
        node.offset = self
            .block_allocator
            .write_block(&buffer, DatBTreeNode::SIZE, node.offset)?;
        self.node_cache
            .lock()
            .unwrap()
            .insert(node.offset, node.clone());
        Ok(())
    }

    fn write_node_async<'a>(
        &'a self,
        node: &'a mut DatBTreeNode,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut buffer = vec![0_u8; DatBTreeNode::SIZE];
            let mut writer = DatBinWriter::new(&mut buffer);
            let _ = node.pack(&mut writer);
            node.offset = self
                .block_allocator
                .write_block_async(&buffer, DatBTreeNode::SIZE, node.offset)
                .await?;
            self.node_cache
                .lock()
                .unwrap()
                .insert(node.offset, node.clone());
            Ok(())
        })
    }

    fn set_new_root(&self, mut root: DatBTreeNode) -> io::Result<()> {
        self.write_node(&mut root)?;
        self.block_allocator.set_root_block(root.offset)?;
        Ok(())
    }

    fn set_new_root_async<'a>(
        &'a self,
        mut root: DatBTreeNode,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            self.write_node_async(&mut root).await?;
            self.block_allocator.set_root_block(root.offset)?;
            Ok(())
        })
    }

    fn split_child(&self, parent: &mut DatBTreeNode, child: &mut DatBTreeNode) -> io::Result<()> {
        let mut new_node = DatBTreeNode::new(self.block_allocator.reserve_block()?);
        let child_idx = parent.branches[..parent.branch_count]
            .iter()
            .position(|offset| *offset == child.offset)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "child not found in parent branches",
                )
            })?;

        parent.insert_file(child_idx, child.files[Self::DEGREE - 1]);
        parent.insert_branch(child_idx + 1, new_node.offset);

        new_node.append_files_from(child, Self::DEGREE, Self::DEGREE - 1);
        child.remove_file_range(Self::DEGREE - 1, Self::DEGREE);

        if !child.is_leaf() {
            new_node.append_branches_from(child, Self::DEGREE, child.branch_count - Self::DEGREE);
            child.remove_branch_range(Self::DEGREE, child.branch_count - Self::DEGREE);
        }

        self.write_node(&mut new_node)?;
        self.write_node(child)?;
        self.write_node(parent)?;
        Ok(())
    }

    fn split_child_async<'a>(
        &'a self,
        parent: &'a mut DatBTreeNode,
        child: &'a mut DatBTreeNode,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut new_node = DatBTreeNode::new(self.block_allocator.reserve_block()?);
            let child_idx = parent.branches[..parent.branch_count]
                .iter()
                .position(|offset| *offset == child.offset)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "child not found in parent branches",
                    )
                })?;

            parent.insert_file(child_idx, child.files[Self::DEGREE - 1]);
            parent.insert_branch(child_idx + 1, new_node.offset);

            new_node.append_files_from(child, Self::DEGREE, Self::DEGREE - 1);
            child.remove_file_range(Self::DEGREE - 1, Self::DEGREE);

            if !child.is_leaf() {
                new_node.append_branches_from(
                    child,
                    Self::DEGREE,
                    child.branch_count - Self::DEGREE,
                );
                child.remove_branch_range(Self::DEGREE, child.branch_count - Self::DEGREE);
            }

            self.write_node_async(&mut new_node).await?;
            self.write_node_async(child).await?;
            self.write_node_async(parent).await?;
            Ok(())
        })
    }

    fn try_find_parent_and_update(
        &self,
        mut node: DatBTreeNode,
        file: DatBTreeFile,
    ) -> io::Result<Option<DatBTreeFile>> {
        let mut left = 0_i32;
        let mut right = node.file_count as i32 - 1;
        let mut i = 0_i32;

        while left <= right {
            i = (left + right) / 2;
            let current_file = node.files[i as usize];
            if file.id == current_file.id {
                let replaced = node.files[i as usize];
                node.files[i as usize] = file;
                self.write_node(&mut node)?;
                return Ok(Some(replaced));
            } else if file.id < current_file.id {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }

        if !node.is_leaf() {
            let branch_index = if node.file_count == 0 {
                0
            } else if file.id > node.files[i as usize].id {
                (i + 1) as usize
            } else {
                i as usize
            };
            if branch_index < node.branch_count {
                if let Some(child) = self.try_get_node(node.branches[branch_index])? {
                    return self.try_find_parent_and_update(child, file);
                }
            }
        }

        Ok(None)
    }

    fn try_find_parent_and_update_async<'a>(
        &'a self,
        mut node: DatBTreeNode,
        file: DatBTreeFile,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<DatBTreeFile>>> + Send + 'a>> {
        Box::pin(async move {
            let mut left = 0_i32;
            let mut right = node.file_count as i32 - 1;
            let mut i = 0_i32;

            while left <= right {
                i = (left + right) / 2;
                let current_file = node.files[i as usize];
                if file.id == current_file.id {
                    let replaced = node.files[i as usize];
                    node.files[i as usize] = file;
                    self.write_node_async(&mut node).await?;
                    return Ok(Some(replaced));
                } else if file.id < current_file.id {
                    right = i - 1;
                } else {
                    left = i + 1;
                }
            }

            if !node.is_leaf() {
                let branch_index = if node.file_count == 0 {
                    0
                } else if file.id > node.files[i as usize].id {
                    (i + 1) as usize
                } else {
                    i as usize
                };
                if branch_index < node.branch_count {
                    if let Some(child) =
                        self.try_get_node_async(node.branches[branch_index]).await?
                    {
                        return self.try_find_parent_and_update_async(child, file).await;
                    }
                }
            }

            Ok(None)
        })
    }

    fn insert_non_full(&self, node: &mut DatBTreeNode, file: DatBTreeFile) -> io::Result<()> {
        let mut position_to_insert = 0_usize;
        while position_to_insert < node.file_count && file.id >= node.files[position_to_insert].id {
            position_to_insert += 1;
        }

        if node.is_leaf() {
            node.insert_file(position_to_insert, file);
            self.write_node(node)?;
            return Ok(());
        }

        let mut child = self
            .try_get_node(node.branches[position_to_insert])?
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "child node not found during insertion",
                )
            })?;

        if child.file_count == Self::MAX_ITEMS {
            self.split_child(node, &mut child)?;
            if file.id > node.files[position_to_insert].id {
                position_to_insert += 1;
                child = self
                    .try_get_node(node.branches[position_to_insert])?
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::NotFound, "child node not found after split")
                    })?;
            }
        }

        self.insert_non_full(&mut child, file)
    }

    fn insert_non_full_async<'a>(
        &'a self,
        node: &'a mut DatBTreeNode,
        file: DatBTreeFile,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut position_to_insert = 0_usize;
            while position_to_insert < node.file_count
                && file.id >= node.files[position_to_insert].id
            {
                position_to_insert += 1;
            }

            if node.is_leaf() {
                node.insert_file(position_to_insert, file);
                self.write_node_async(node).await?;
                return Ok(());
            }

            let mut child = self
                .try_get_node_async(node.branches[position_to_insert])
                .await?
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "child node not found during insertion",
                    )
                })?;

            if child.file_count == Self::MAX_ITEMS {
                self.split_child_async(node, &mut child).await?;
                if file.id > node.files[position_to_insert].id {
                    position_to_insert += 1;
                    child = self
                        .try_get_node_async(node.branches[position_to_insert])
                        .await?
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::NotFound,
                                "child node not found after split",
                            )
                        })?;
                }
            }

            self.insert_non_full_async(&mut child, file).await
        })
    }

    fn delete_internal(&self, node: DatBTreeNode, file_id: u32) -> io::Result<()> {
        let mut i = 0usize;
        while i < node.file_count && file_id > node.files[i].id {
            i += 1;
        }

        if i < node.file_count && node.files[i].id == file_id {
            self.delete_key_from_node(node, file_id, i)
        } else if !node.is_leaf() {
            self.delete_key_from_subtree(node, file_id, i)
        } else {
            Ok(())
        }
    }

    fn delete_internal_async<'a>(
        &'a self,
        node: DatBTreeNode,
        file_id: u32,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut i = 0usize;
            while i < node.file_count && file_id > node.files[i].id {
                i += 1;
            }

            if i < node.file_count && node.files[i].id == file_id {
                self.delete_key_from_node_async(node, file_id, i).await
            } else if !node.is_leaf() {
                self.delete_key_from_subtree_async(node, file_id, i).await
            } else {
                Ok(())
            }
        })
    }

    fn delete_key_from_subtree(
        &self,
        mut parent_node: DatBTreeNode,
        key_to_delete: u32,
        subtree_index_in_node: usize,
    ) -> io::Result<()> {
        let mut child_node = self
            .try_get_node(parent_node.branches[subtree_index_in_node])?
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "unable to lookup child node")
            })?;

        if child_node.file_count == Self::MIN_ITEMS {
            let left_index = subtree_index_in_node as i32 - 1;
            let right_index = subtree_index_in_node + 1;
            let left_sibling = if subtree_index_in_node > 0 {
                self.try_get_node(parent_node.branches[left_index as usize])?
            } else {
                None
            };
            let right_sibling = if subtree_index_in_node < parent_node.branch_count - 1 {
                self.try_get_node(parent_node.branches[right_index])?
            } else {
                None
            };

            if let Some(mut left_sibling) = left_sibling {
                if left_sibling.file_count > Self::DEGREE - 1 {
                    child_node.insert_file(0, parent_node.files[subtree_index_in_node]);
                    parent_node.files[subtree_index_in_node] =
                        left_sibling.files[left_sibling.file_count - 1];
                    left_sibling.remove_file_at(left_sibling.file_count - 1);

                    if !left_sibling.is_leaf() {
                        child_node
                            .insert_branch(0, left_sibling.branches[left_sibling.branch_count - 1]);
                        left_sibling.remove_branch_at(left_sibling.branch_count - 1);
                    }

                    self.write_node(&mut child_node)?;
                    self.write_node(&mut parent_node)?;
                    self.write_node(&mut left_sibling)?;
                } else if let Some(mut right_sibling) = right_sibling {
                    if right_sibling.file_count > Self::DEGREE - 1 {
                        child_node.add_file(parent_node.files[subtree_index_in_node]);
                        parent_node.files[subtree_index_in_node] = right_sibling.files[0];
                        right_sibling.remove_file_at(0);

                        if !right_sibling.is_leaf() {
                            child_node.add_branch(right_sibling.branches[0]);
                            right_sibling.remove_branch_at(0);
                        }

                        self.write_node(&mut child_node)?;
                        self.write_node(&mut parent_node)?;
                        self.write_node(&mut right_sibling)?;
                    } else {
                        child_node.insert_file(0, parent_node.files[subtree_index_in_node]);
                        child_node.prepend_files_from(&left_sibling);
                        if !left_sibling.is_leaf() {
                            child_node.prepend_branches_from(&left_sibling);
                        }

                        parent_node.remove_branch_at(left_index as usize);
                        parent_node.remove_file_at(subtree_index_in_node);

                        self.write_node(&mut child_node)?;
                        self.write_node(&mut parent_node)?;
                    }
                } else {
                    child_node.insert_file(0, parent_node.files[subtree_index_in_node]);
                    child_node.prepend_files_from(&left_sibling);
                    if !left_sibling.is_leaf() {
                        child_node.prepend_branches_from(&left_sibling);
                    }

                    parent_node.remove_branch_at(left_index as usize);
                    parent_node.remove_file_at(subtree_index_in_node);

                    self.write_node(&mut child_node)?;
                    self.write_node(&mut parent_node)?;
                }
            } else if let Some(mut right_sibling) = right_sibling {
                if right_sibling.file_count > Self::DEGREE - 1 {
                    child_node.add_file(parent_node.files[subtree_index_in_node]);
                    parent_node.files[subtree_index_in_node] = right_sibling.files[0];
                    right_sibling.remove_file_at(0);

                    if !right_sibling.is_leaf() {
                        child_node.add_branch(right_sibling.branches[0]);
                        right_sibling.remove_branch_at(0);
                    }

                    self.write_node(&mut child_node)?;
                    self.write_node(&mut parent_node)?;
                    self.write_node(&mut right_sibling)?;
                } else {
                    child_node.add_file(parent_node.files[subtree_index_in_node]);
                    child_node.append_files_from(&right_sibling, 0, right_sibling.file_count);
                    if !right_sibling.is_leaf() {
                        child_node.append_branches_from(
                            &right_sibling,
                            0,
                            right_sibling.branch_count,
                        );
                    }

                    parent_node.remove_branch_at(right_index);
                    parent_node.remove_file_at(subtree_index_in_node);

                    self.write_node(&mut child_node)?;
                    self.write_node(&mut parent_node)?;
                }
            }
        }

        self.delete_internal(child_node, key_to_delete)
    }

    fn delete_key_from_subtree_async<'a>(
        &'a self,
        mut parent_node: DatBTreeNode,
        key_to_delete: u32,
        subtree_index_in_node: usize,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut child_node = self
                .try_get_node_async(parent_node.branches[subtree_index_in_node])
                .await?
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::NotFound, "unable to lookup child node")
                })?;

            if child_node.file_count == Self::MIN_ITEMS {
                let left_index = subtree_index_in_node as i32 - 1;
                let right_index = subtree_index_in_node + 1;
                let left_sibling = if subtree_index_in_node > 0 {
                    self.try_get_node_async(parent_node.branches[left_index as usize])
                        .await?
                } else {
                    None
                };
                let right_sibling = if subtree_index_in_node < parent_node.branch_count - 1 {
                    self.try_get_node_async(parent_node.branches[right_index])
                        .await?
                } else {
                    None
                };

                if let Some(mut left_sibling) = left_sibling {
                    if left_sibling.file_count > Self::DEGREE - 1 {
                        child_node.insert_file(0, parent_node.files[subtree_index_in_node]);
                        parent_node.files[subtree_index_in_node] =
                            left_sibling.files[left_sibling.file_count - 1];
                        left_sibling.remove_file_at(left_sibling.file_count - 1);

                        if !left_sibling.is_leaf() {
                            child_node.insert_branch(
                                0,
                                left_sibling.branches[left_sibling.branch_count - 1],
                            );
                            left_sibling.remove_branch_at(left_sibling.branch_count - 1);
                        }

                        self.write_node_async(&mut child_node).await?;
                        self.write_node_async(&mut parent_node).await?;
                        self.write_node_async(&mut left_sibling).await?;
                    } else if let Some(mut right_sibling) = right_sibling {
                        if right_sibling.file_count > Self::DEGREE - 1 {
                            child_node.add_file(parent_node.files[subtree_index_in_node]);
                            parent_node.files[subtree_index_in_node] = right_sibling.files[0];
                            right_sibling.remove_file_at(0);

                            if !right_sibling.is_leaf() {
                                child_node.add_branch(right_sibling.branches[0]);
                                right_sibling.remove_branch_at(0);
                            }

                            self.write_node_async(&mut child_node).await?;
                            self.write_node_async(&mut parent_node).await?;
                            self.write_node_async(&mut right_sibling).await?;
                        } else {
                            child_node.insert_file(0, parent_node.files[subtree_index_in_node]);
                            child_node.prepend_files_from(&left_sibling);
                            if !left_sibling.is_leaf() {
                                child_node.prepend_branches_from(&left_sibling);
                            }

                            parent_node.remove_branch_at(left_index as usize);
                            parent_node.remove_file_at(subtree_index_in_node);

                            self.write_node_async(&mut child_node).await?;
                            self.write_node_async(&mut parent_node).await?;
                        }
                    } else {
                        child_node.insert_file(0, parent_node.files[subtree_index_in_node]);
                        child_node.prepend_files_from(&left_sibling);
                        if !left_sibling.is_leaf() {
                            child_node.prepend_branches_from(&left_sibling);
                        }

                        parent_node.remove_branch_at(left_index as usize);
                        parent_node.remove_file_at(subtree_index_in_node);

                        self.write_node_async(&mut child_node).await?;
                        self.write_node_async(&mut parent_node).await?;
                    }
                } else if let Some(mut right_sibling) = right_sibling {
                    if right_sibling.file_count > Self::DEGREE - 1 {
                        child_node.add_file(parent_node.files[subtree_index_in_node]);
                        parent_node.files[subtree_index_in_node] = right_sibling.files[0];
                        right_sibling.remove_file_at(0);

                        if !right_sibling.is_leaf() {
                            child_node.add_branch(right_sibling.branches[0]);
                            right_sibling.remove_branch_at(0);
                        }

                        self.write_node_async(&mut child_node).await?;
                        self.write_node_async(&mut parent_node).await?;
                        self.write_node_async(&mut right_sibling).await?;
                    } else {
                        child_node.add_file(parent_node.files[subtree_index_in_node]);
                        child_node.append_files_from(&right_sibling, 0, right_sibling.file_count);
                        if !right_sibling.is_leaf() {
                            child_node.append_branches_from(
                                &right_sibling,
                                0,
                                right_sibling.branch_count,
                            );
                        }

                        parent_node.remove_branch_at(right_index);
                        parent_node.remove_file_at(subtree_index_in_node);

                        self.write_node_async(&mut child_node).await?;
                        self.write_node_async(&mut parent_node).await?;
                    }
                }
            }

            self.delete_internal_async(child_node, key_to_delete).await
        })
    }

    fn delete_key_from_node(
        &self,
        mut node: DatBTreeNode,
        key_to_delete: u32,
        key_index_in_node: usize,
    ) -> io::Result<()> {
        if node.is_leaf() {
            node.remove_file_at(key_index_in_node);
            self.write_node(&mut node)?;
            return Ok(());
        }

        let mut predecessor_child = self
            .try_get_node(node.branches[key_index_in_node])?
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "unable to lookup predecessor child",
                )
            })?;

        if predecessor_child.file_count >= Self::DEGREE {
            let predecessor = self.delete_predecessor(predecessor_child)?;
            node.files[key_index_in_node] = predecessor;
            self.write_node(&mut node)
        } else {
            let successor_child = self
                .try_get_node(node.branches[key_index_in_node + 1])?
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::NotFound, "unable to lookup successor child")
                })?;

            if successor_child.file_count >= Self::DEGREE {
                let successor = self.delete_successor(successor_child)?;
                node.files[key_index_in_node] = successor;
                self.write_node(&mut node)
            } else {
                predecessor_child.add_file(node.files[key_index_in_node]);
                predecessor_child.append_files_from(
                    &successor_child,
                    0,
                    successor_child.file_count,
                );
                predecessor_child.append_branches_from(
                    &successor_child,
                    0,
                    successor_child.branch_count,
                );

                node.remove_file_at(key_index_in_node);
                node.remove_branch_at(key_index_in_node + 1);

                self.write_node(&mut node)?;
                self.write_node(&mut predecessor_child)?;

                self.delete_internal(predecessor_child, key_to_delete)
            }
        }
    }

    fn delete_key_from_node_async<'a>(
        &'a self,
        mut node: DatBTreeNode,
        key_to_delete: u32,
        key_index_in_node: usize,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            if node.is_leaf() {
                node.remove_file_at(key_index_in_node);
                self.write_node_async(&mut node).await?;
                return Ok(());
            }

            let mut predecessor_child = self
                .try_get_node_async(node.branches[key_index_in_node])
                .await?
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "unable to lookup predecessor child",
                    )
                })?;

            if predecessor_child.file_count >= Self::DEGREE {
                let predecessor = self.delete_predecessor_async(predecessor_child).await?;
                node.files[key_index_in_node] = predecessor;
                self.write_node_async(&mut node).await
            } else {
                let successor_child = self
                    .try_get_node_async(node.branches[key_index_in_node + 1])
                    .await?
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::NotFound, "unable to lookup successor child")
                    })?;

                if successor_child.file_count >= Self::DEGREE {
                    let successor = self.delete_successor_async(successor_child).await?;
                    node.files[key_index_in_node] = successor;
                    self.write_node_async(&mut node).await
                } else {
                    predecessor_child.add_file(node.files[key_index_in_node]);
                    predecessor_child.append_files_from(
                        &successor_child,
                        0,
                        successor_child.file_count,
                    );
                    predecessor_child.append_branches_from(
                        &successor_child,
                        0,
                        successor_child.branch_count,
                    );

                    node.remove_file_at(key_index_in_node);
                    node.remove_branch_at(key_index_in_node + 1);

                    self.write_node_async(&mut node).await?;
                    self.write_node_async(&mut predecessor_child).await?;

                    self.delete_internal_async(predecessor_child, key_to_delete)
                        .await
                }
            }
        })
    }

    fn delete_predecessor(&self, mut node: DatBTreeNode) -> io::Result<DatBTreeFile> {
        if node.is_leaf() {
            let result = node.files[node.file_count - 1];
            node.remove_file_at(node.file_count - 1);
            self.write_node(&mut node)?;
            return Ok(result);
        }

        let predecessor = self
            .try_get_node(node.branches[node.branch_count - 1])?
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "failed to look up predecessor")
            })?;
        self.delete_predecessor(predecessor)
    }

    fn delete_predecessor_async<'a>(
        &'a self,
        mut node: DatBTreeNode,
    ) -> Pin<Box<dyn Future<Output = io::Result<DatBTreeFile>> + Send + 'a>> {
        Box::pin(async move {
            if node.is_leaf() {
                let result = node.files[node.file_count - 1];
                node.remove_file_at(node.file_count - 1);
                self.write_node_async(&mut node).await?;
                return Ok(result);
            }

            let predecessor = self
                .try_get_node_async(node.branches[node.branch_count - 1])
                .await?
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::NotFound, "failed to look up predecessor")
                })?;
            self.delete_predecessor_async(predecessor).await
        })
    }

    fn delete_successor(&self, mut node: DatBTreeNode) -> io::Result<DatBTreeFile> {
        if node.is_leaf() {
            let result = node.files[0];
            node.remove_file_at(0);
            self.write_node(&mut node)?;
            return Ok(result);
        }

        let successor = self.try_get_node(node.branches[0])?.ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "failed to look up successor")
        })?;
        self.delete_successor(successor)
    }

    fn delete_successor_async<'a>(
        &'a self,
        mut node: DatBTreeNode,
    ) -> Pin<Box<dyn Future<Output = io::Result<DatBTreeFile>> + Send + 'a>> {
        Box::pin(async move {
            if node.is_leaf() {
                let result = node.files[0];
                node.remove_file_at(0);
                self.write_node_async(&mut node).await?;
                return Ok(result);
            }

            let successor = self
                .try_get_node_async(node.branches[0])
                .await?
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::NotFound, "failed to look up successor")
                })?;
            self.delete_successor_async(successor).await
        })
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

    fn try_get_node_async<'a>(
        &'a self,
        block_offset: i32,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<DatBTreeNode>>> + Send + 'a>> {
        Box::pin(async move {
            if block_offset == 0 || block_offset == 0xCDCDCDCD_u32 as i32 {
                return Ok(None);
            }

            if let Some(node) = self.node_cache.lock().unwrap().get(&block_offset).cloned() {
                return Ok(Some(node));
            }

            let mut buffer = vec![0_u8; DatBTreeNode::SIZE];
            self.block_allocator
                .read_block_async(&mut buffer, block_offset as usize)
                .await?;
            let mut node = DatBTreeNode::new(block_offset);
            let _ = node.unpack(&mut DatBinReader::new(&buffer));
            self.node_cache
                .lock()
                .unwrap()
                .insert(block_offset, node.clone());
            Ok(Some(node))
        })
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

    fn try_get_file_internal_async<'a>(
        &'a self,
        file_id: u32,
        starting_block: i32,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<DatBTreeFile>>> + Send + 'a>> {
        Box::pin(async move {
            let mut current_block = starting_block;
            while current_block != 0 && current_block != 0xCDCDCDCD_u32 as i32 {
                let Some(node) = self.try_get_node_async(current_block).await? else {
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
        })
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
