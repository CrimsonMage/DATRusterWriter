use std::io;

use uuid::Uuid;

use crate::{
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::DatHeader::DatHeader,
};

pub struct BaseBlockAllocator;

impl BaseBlockAllocator {
    pub fn unsupported_write() -> io::Error {
        io::Error::new(io::ErrorKind::Unsupported, "allocator is not writable")
    }

    pub fn header_block_count(block_size: i32) -> i32 {
        ((DatHeader::SIZE as i32 + block_size - 1) / block_size).max(1)
    }

    pub fn first_aligned_block_offset(block_size: i32) -> i32 {
        Self::header_block_count(block_size) * block_size
    }

    pub fn init_new_header(file_type: DatFileType, subset: u32, block_size: i32) -> DatHeader {
        let mut header = DatHeader::new(file_type, subset, block_size, None, 0, 0, Uuid::nil(), 0);
        header.root_block = 0;
        header.file_size = Self::header_block_count(block_size) * block_size;
        header
    }

    pub fn set_version(
        header: &mut DatHeader,
        version: &str,
        engine_version: i32,
        game_version: i32,
        major_version: Uuid,
        minor_version: u32,
    ) -> io::Result<()> {
        if version.len() > 255 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "version string can only be 255 characters max",
            ));
        }

        header.version = Some(version.to_string());
        header.engine_version = engine_version;
        header.game_version = game_version;
        header.major_version = major_version;
        header.minor_version = minor_version;
        Ok(())
    }
}
