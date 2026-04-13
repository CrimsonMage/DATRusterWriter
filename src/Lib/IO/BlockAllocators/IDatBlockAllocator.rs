use std::io;

use crate::Lib::IO::DatHeader::DatHeader;

pub trait IDatBlockAllocator: Send + Sync {
    fn can_write(&self) -> bool;
    fn has_header_data(&self) -> bool;
    fn header(&self) -> &DatHeader;
    fn read_bytes(
        &self,
        buffer: &mut [u8],
        buffer_offset: usize,
        byte_offset: usize,
        num_bytes: usize,
    ) -> io::Result<()>;
    fn read_block(&self, buffer: &mut [u8], starting_block: usize) -> io::Result<()>;
}
