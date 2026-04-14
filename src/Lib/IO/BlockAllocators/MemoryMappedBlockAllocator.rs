use std::{fs::File, io, sync::Arc};

use memmap2::Mmap;

use crate::{
    Generated::Enums::DatFileType::DatFileType,
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

    fn write_unsupported() -> io::Error {
        io::Error::new(
            io::ErrorKind::Unsupported,
            "memory-mapped allocator is currently read-only",
        )
    }
}

impl IDatBlockAllocator for MemoryMappedBlockAllocator {
    fn can_write(&self) -> bool {
        self.can_write
    }

    fn has_header_data(&self) -> bool {
        self.has_header_data
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
        Err(Self::write_unsupported())
    }

    fn set_version(
        &self,
        _version: &str,
        _engine_version: i32,
        _game_version: i32,
        _major_version: uuid::Uuid,
        _minor_version: u32,
    ) -> io::Result<()> {
        Err(Self::write_unsupported())
    }

    fn write_bytes(
        &self,
        _buffer: &[u8],
        _byte_offset: usize,
        _num_bytes: usize,
    ) -> io::Result<()> {
        Err(Self::write_unsupported())
    }

    fn write_block(
        &self,
        _buffer: &[u8],
        _num_bytes: usize,
        _starting_block: i32,
    ) -> io::Result<i32> {
        Err(Self::write_unsupported())
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

    fn try_get_block_offsets(&self, starting_block: i32) -> io::Result<Option<Vec<i32>>> {
        if starting_block <= 0 {
            return Ok(None);
        }

        let mut offsets = Vec::new();
        let mut current_block = starting_block as usize;
        while current_block != 0 {
            offsets.push(current_block as i32);
            let mut next_block_bytes = [0_u8; 4];
            self.read_bytes(&mut next_block_bytes, 0, current_block, 4)?;
            current_block = i32::from_le_bytes(next_block_bytes).max(0) as usize;
        }

        Ok(Some(offsets))
    }

    fn allocate_empty_blocks(&self, _num_blocks_to_allocate: i32) -> io::Result<()> {
        Err(Self::write_unsupported())
    }

    fn reserve_block(&self) -> io::Result<i32> {
        Err(Self::write_unsupported())
    }

    fn set_root_block(&self, _offset: i32) -> io::Result<()> {
        Err(Self::write_unsupported())
    }
}
