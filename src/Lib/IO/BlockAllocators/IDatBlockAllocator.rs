use std::io;

use crate::{
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::DatHeader::DatHeader,
};

pub trait IDatBlockAllocator: Send + Sync {
    fn can_write(&self) -> bool;
    fn has_header_data(&self) -> bool;
    fn header(&self) -> DatHeader;
    fn init_new(
        &self,
        file_type: DatFileType,
        subset: u32,
        block_size: i32,
        num_blocks_to_allocate: i32,
    ) -> io::Result<()>;
    fn set_version(
        &self,
        version: &str,
        engine_version: i32,
        game_version: i32,
        major_version: uuid::Uuid,
        minor_version: u32,
    ) -> io::Result<()>;
    fn write_bytes(&self, buffer: &[u8], byte_offset: usize, num_bytes: usize) -> io::Result<()>;
    fn write_block(
        &self,
        buffer: &[u8],
        num_bytes: usize,
        starting_block: i32,
    ) -> io::Result<i32>;
    fn read_bytes(
        &self,
        buffer: &mut [u8],
        buffer_offset: usize,
        byte_offset: usize,
        num_bytes: usize,
    ) -> io::Result<()>;
    fn read_block(&self, buffer: &mut [u8], starting_block: usize) -> io::Result<()>;
    fn try_get_block_offsets(&self, starting_block: i32) -> io::Result<Option<Vec<i32>>>;
    fn allocate_empty_blocks(&self, num_blocks_to_allocate: i32) -> io::Result<()>;
    fn reserve_block(&self) -> io::Result<i32>;
    fn set_root_block(&self, offset: i32) -> io::Result<()>;
}
