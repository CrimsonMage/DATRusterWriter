use crate::Lib::IO::{
    DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
    IUnpackable::IUnpackable,
};

use super::DatBTreeFile::DatBTreeFile;

#[derive(Debug, Clone)]
pub struct DatBTreeNode {
    pub offset: i32,
    pub branches: [i32; Self::MAX_BRANCHES],
    pub branch_count: usize,
    pub files: [DatBTreeFile; Self::MAX_FILES],
    pub file_count: usize,
}

impl DatBTreeNode {
    pub const MAX_BRANCHES: usize = 62;
    pub const MAX_FILES: usize = 61;
    pub const SIZE: usize = 1720;
    const UNUSED_BRANCH_SENTINEL: i32 = 0xCDCDCDCD_u32 as i32;

    pub fn new(block_offset: i32) -> Self {
        Self {
            offset: block_offset,
            branches: [0; Self::MAX_BRANCHES],
            branch_count: 0,
            files: [DatBTreeFile::default(); Self::MAX_FILES],
            file_count: 0,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.branch_count == 0
    }

    pub fn insert_file(&mut self, index: usize, file: DatBTreeFile) {
        for i in (index..self.file_count).rev() {
            self.files[i + 1] = self.files[i];
        }
        self.files[index] = file;
        self.file_count += 1;
    }

    pub fn add_file(&mut self, file: DatBTreeFile) {
        self.files[self.file_count] = file;
        self.file_count += 1;
    }

    pub fn remove_file_at(&mut self, index: usize) {
        if self.file_count == 0 || index >= self.file_count {
            return;
        }
        self.file_count -= 1;
        for i in index..self.file_count {
            self.files[i] = self.files[i + 1];
        }
        self.files[self.file_count] = DatBTreeFile::default();
    }

    pub fn insert_branch(&mut self, index: usize, branch_offset: i32) {
        for i in (index..self.branch_count).rev() {
            self.branches[i + 1] = self.branches[i];
        }
        self.branches[index] = branch_offset;
        self.branch_count += 1;
    }

    pub fn add_branch(&mut self, branch_offset: i32) {
        self.branches[self.branch_count] = branch_offset;
        self.branch_count += 1;
    }

    pub fn remove_branch_at(&mut self, index: usize) {
        if self.branch_count == 0 || index >= self.branch_count {
            return;
        }
        self.branch_count -= 1;
        for i in index..self.branch_count {
            self.branches[i] = self.branches[i + 1];
        }
        self.branches[self.branch_count] = 0;
    }

    pub fn append_files_from(&mut self, source: &DatBTreeNode, src_index: usize, count: usize) {
        for i in 0..count {
            self.files[self.file_count + i] = source.files[src_index + i];
        }
        self.file_count += count;
    }

    pub fn append_branches_from(&mut self, source: &DatBTreeNode, src_index: usize, count: usize) {
        for i in 0..count {
            self.branches[self.branch_count + i] = source.branches[src_index + i];
        }
        self.branch_count += count;
    }

    pub fn remove_file_range(&mut self, index: usize, count: usize) {
        if count == 0 || index >= self.file_count {
            return;
        }
        let remaining = self.file_count.saturating_sub(index + count);
        if remaining > 0 {
            for i in 0..remaining {
                self.files[index + i] = self.files[index + count + i];
            }
        }
        let old_count = self.file_count;
        self.file_count = self.file_count.saturating_sub(count);
        for i in self.file_count..old_count {
            self.files[i] = DatBTreeFile::default();
        }
    }

    pub fn remove_branch_range(&mut self, index: usize, count: usize) {
        if count == 0 || index >= self.branch_count {
            return;
        }
        let remaining = self.branch_count.saturating_sub(index + count);
        if remaining > 0 {
            for i in 0..remaining {
                self.branches[index + i] = self.branches[index + count + i];
            }
        }
        let old_count = self.branch_count;
        self.branch_count = self.branch_count.saturating_sub(count);
        for i in self.branch_count..old_count {
            self.branches[i] = 0;
        }
    }

    pub fn prepend_files_from(&mut self, source: &DatBTreeNode) {
        let src_count = source.file_count;
        for i in (0..self.file_count).rev() {
            self.files[i + src_count] = self.files[i];
        }
        for i in 0..src_count {
            self.files[i] = source.files[i];
        }
        self.file_count += src_count;
    }

    pub fn prepend_branches_from(&mut self, source: &DatBTreeNode) {
        let src_count = source.branch_count;
        for i in (0..self.branch_count).rev() {
            self.branches[i + src_count] = self.branches[i];
        }
        for i in 0..src_count {
            self.branches[i] = source.branches[i];
        }
        self.branch_count += src_count;
    }
}

impl IUnpackable for DatBTreeNode {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let start_offset = reader.offset();
        reader.skip(Self::MAX_BRANCHES * 4);
        let file_count = reader.read_i32().clamp(0, Self::MAX_FILES as i32) as usize;
        reader.set_offset(start_offset);

        self.branches = [0; Self::MAX_BRANCHES];
        self.files = [DatBTreeFile::default(); Self::MAX_FILES];
        self.branch_count = 0;
        self.file_count = 0;

        let mut did_find_end = false;
        let mut last_branch = 0_i32;
        for _ in 0..Self::MAX_BRANCHES {
            let branch = reader.read_i32();
            if branch == 0 || branch == last_branch || branch == Self::UNUSED_BRANCH_SENTINEL {
                did_find_end = true;
            }

            if !did_find_end {
                last_branch = branch;
                self.branches[self.branch_count] = branch;
                self.branch_count += 1;
            }
        }

        let _stored_file_count = reader.read_i32();
        for i in 0..file_count {
            let mut file = DatBTreeFile::default();
            let _ = file.unpack(reader);
            self.files[i] = file;
        }
        self.file_count = file_count;

        if self.branch_count > 0 && self.file_count > 0 {
            self.branch_count = self.branch_count.min(self.file_count + 1);
        }

        true
    }
}

impl IPackable for DatBTreeNode {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        for i in 0..Self::MAX_BRANCHES {
            if i < self.branch_count {
                writer.write_i32(self.branches[i]);
            } else if self.file_count == 0 {
                writer.write_i32(0);
            } else {
                writer.write_i32(Self::UNUSED_BRANCH_SENTINEL);
            }
        }

        writer.write_i32(self.file_count as i32);
        for i in 0..self.file_count {
            let _ = self.files[i].pack(writer);
        }

        if !self.is_leaf() && self.branch_count != self.file_count + 1 {
            panic!(
                "PACK Branches.Count != Files.Count + 1 ({} != {} + 1)",
                self.branch_count, self.file_count
            );
        }

        true
    }
}
