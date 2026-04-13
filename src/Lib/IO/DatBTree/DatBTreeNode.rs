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
        for i in 0..Self::MAX_BRANCHES {
            let branch = reader.read_i32();
            if branch == 0 || branch == last_branch || branch == Self::UNUSED_BRANCH_SENTINEL {
                did_find_end = true;
            }

            if !did_find_end {
                last_branch = branch;
                self.branches[self.branch_count] = branch;
                self.branch_count += 1;
            }

            if i + 1 >= Self::MAX_BRANCHES {
                break;
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
