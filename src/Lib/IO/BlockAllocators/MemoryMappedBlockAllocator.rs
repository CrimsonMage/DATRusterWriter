use std::{fs::File, io, sync::Arc};

use memmap2::Mmap;

use crate::{
    Lib::IO::{
        BlockAllocators::IDatBlockAllocator::IDatBlockAllocator, DatBinReader::DatBinReader,
        DatHeader::DatHeader, IUnpackable::IUnpackable,
    },
    Options::DatDatabaseOptions::DatDatabaseOptions,
};

pub struct MemoryMappedBlockAllocator {
    mmap: Mmap,
    header: DatHeader,
    has_header_data: bool,
    can_write: bool,
}

impl MemoryMappedBlockAllocator {
    pub fn new(options: &DatDatabaseOptions) -> io::Result<Arc<Self>> {
        let file = File::open(&options.file_path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < DatHeader::SIZE {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "DAT file is smaller than the header size",
            ));
        }

        let mut header = DatHeader::default();
        let mut reader = DatBinReader::new(&mmap[..DatHeader::SIZE]);
        let has_header_data = header.unpack(&mut reader);

        Ok(Arc::new(Self {
            mmap,
            header,
            has_header_data,
            can_write: false,
        }))
    }
}

impl IDatBlockAllocator for MemoryMappedBlockAllocator {
    fn can_write(&self) -> bool {
        self.can_write
    }

    fn has_header_data(&self) -> bool {
        self.has_header_data
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
        let source_end = byte_offset + num_bytes;
        let dest_end = buffer_offset + num_bytes;
        if source_end > self.mmap.len() || dest_end > buffer.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Attempted to read beyond the DAT bounds",
            ));
        }

        buffer[buffer_offset..dest_end].copy_from_slice(&self.mmap[byte_offset..source_end]);
        Ok(())
    }

    fn read_block(&self, buffer: &mut [u8], starting_block: usize) -> io::Result<()> {
        let block_size = self.header.block_size as usize;
        let block_data_size = block_size.saturating_sub(4);
        let mut current_block = starting_block;
        let mut total_read = 0_usize;

        while current_block != 0 && total_read < buffer.len() {
            let bytes_to_read = block_data_size.min(buffer.len() - total_read);
            self.read_bytes(buffer, total_read, current_block + 4, bytes_to_read)?;
            total_read += bytes_to_read;

            if total_read >= buffer.len() {
                return Ok(());
            }

            let mut next_block_bytes = [0_u8; 4];
            self.read_bytes(&mut next_block_bytes, 0, current_block, 4)?;
            current_block = i32::from_le_bytes(next_block_bytes).max(0) as usize;
        }

        Ok(())
    }
}
