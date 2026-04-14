use std::{
    fs::{File, OpenOptions},
    future::Future,
    io::{self, Read, Seek, SeekFrom, Write},
    pin::Pin,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::{
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{
        BlockAllocators::IDatBlockAllocator::IDatBlockAllocator, DatBinReader::DatBinReader,
        DatBinWriter::DatBinWriter, DatHeader::DatHeader, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Options::DatDatabaseOptions::DatDatabaseOptions,
};

struct StreamState {
    file: File,
    header: DatHeader,
    has_header_data: bool,
}

pub struct StreamBlockAllocator {
    state: Mutex<StreamState>,
    can_write: bool,
}

impl StreamBlockAllocator {
    pub fn new(options: &DatDatabaseOptions) -> io::Result<Arc<Self>> {
        let can_write = matches!(
            options.access_type,
            crate::Options::DatAccessType::DatAccessType::ReadWrite
        );
        let file = if can_write {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&options.file_path)?
        } else {
            OpenOptions::new().read(true).open(&options.file_path)?
        };

        if can_write && file.metadata()?.len() < DatHeader::SIZE as u64 {
            file.set_len(DatHeader::SIZE as u64)?;
        }

        let mut state = StreamState {
            file,
            header: DatHeader::default(),
            has_header_data: false,
        };
        Self::try_read_header(&mut state)?;

        Ok(Arc::new(Self {
            state: Mutex::new(state),
            can_write,
        }))
    }

    fn unsupported_write() -> io::Error {
        io::Error::new(io::ErrorKind::Unsupported, "allocator is not writable")
    }

    fn try_read_header(state: &mut StreamState) -> io::Result<()> {
        let len = state.file.metadata()?.len();
        if len < DatHeader::SIZE as u64 {
            state.has_header_data = false;
            return Ok(());
        }

        let mut buffer = vec![0u8; DatHeader::SIZE];
        state.file.seek(SeekFrom::Start(0))?;
        state.file.read_exact(&mut buffer)?;
        let mut header = DatHeader::default();
        let has_header_data = header.unpack(&mut DatBinReader::new(&buffer));
        state.header = header;
        state.has_header_data = has_header_data;
        Ok(())
    }

    fn write_header_locked(state: &mut StreamState) -> io::Result<()> {
        let mut buffer = vec![0u8; DatHeader::SIZE];
        state.header.write_empty_transaction();
        let _ = state.header.pack(&mut DatBinWriter::new(&mut buffer));
        state.file.seek(SeekFrom::Start(0))?;
        state.file.write_all(&buffer)?;
        state.file.flush()?;
        state.has_header_data = true;
        Ok(())
    }

    fn expand_locked(state: &mut StreamState, new_size_in_bytes: i32) -> io::Result<()> {
        let current_len = state.file.metadata()?.len() as i32;
        if new_size_in_bytes <= current_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "cannot shrink or keep file the same size with expand({new_size_in_bytes}) when current size is {current_len}"
                ),
            ));
        }
        state.file.set_len(new_size_in_bytes as u64)?;
        state.header.file_size = new_size_in_bytes;
        Ok(())
    }

    fn allocate_empty_blocks_locked(
        state: &mut StreamState,
        num_blocks_to_allocate: i32,
    ) -> io::Result<()> {
        if num_blocks_to_allocate <= 0 {
            return Ok(());
        }

        if state.header.first_free_block == 0 && state.header.last_free_block == 0 {
            let first_block_offset = ((DatHeader::SIZE as i32 + state.header.block_size - 1)
                / state.header.block_size)
                * state.header.block_size;
            state.header.free_block_count = 0;
            state.header.first_free_block = first_block_offset;
            state.header.last_free_block = first_block_offset;
            state.header.file_size = first_block_offset;
        }

        let offset = state.header.file_size;
        Self::expand_locked(
            state,
            state.header.file_size + num_blocks_to_allocate * state.header.block_size,
        )?;

        if state.header.free_block_count == 0 {
            state.header.first_free_block = offset;
        }

        state.header.last_free_block = state.header.file_size - state.header.block_size;
        state.header.free_block_count += num_blocks_to_allocate;
        Self::write_header_locked(state)
    }

    fn reserve_block_locked(state: &mut StreamState) -> io::Result<i32> {
        if state.header.free_block_count > 0 {
            let free_block_offset = state.header.first_free_block;
            state.header.first_free_block += state.header.block_size;
            state.header.free_block_count -= 1;
            Self::write_header_locked(state)?;
            Ok(free_block_offset)
        } else {
            Self::allocate_empty_blocks_locked(state, 50)?;
            Self::reserve_block_locked(state)
        }
    }
}

impl IDatBlockAllocator for StreamBlockAllocator {
    fn can_write(&self) -> bool {
        self.can_write
    }

    fn has_header_data(&self) -> bool {
        self.state.lock().unwrap().has_header_data
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
        if !self.can_write {
            return Err(Self::unsupported_write());
        }
        let mut state = self.state.lock().unwrap();
        let header_block_count = ((DatHeader::SIZE as i32 + block_size - 1) / block_size).max(1);
        state.header = DatHeader::new(file_type, subset, block_size, None, 0, 0, Uuid::nil(), 0);
        state.header.root_block = 0;
        state.header.file_size = header_block_count * block_size;
        if num_blocks_to_allocate > 0 {
            Self::allocate_empty_blocks_locked(&mut state, num_blocks_to_allocate)?;
        } else {
            Self::expand_locked(&mut state, header_block_count * block_size)?;
            Self::write_header_locked(&mut state)?;
        }
        state.has_header_data = true;
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
        if !self.can_write {
            return Err(Self::unsupported_write());
        }
        if version.len() > 255 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "version string can only be 255 characters max",
            ));
        }
        let mut state = self.state.lock().unwrap();
        state.header.version = Some(version.to_string());
        state.header.engine_version = engine_version;
        state.header.game_version = game_version;
        state.header.major_version = major_version;
        state.header.minor_version = minor_version;
        Self::write_header_locked(&mut state)
    }

    fn write_bytes(&self, buffer: &[u8], byte_offset: usize, num_bytes: usize) -> io::Result<()> {
        if !self.can_write {
            return Err(Self::unsupported_write());
        }
        let mut state = self.state.lock().unwrap();
        state.file.seek(SeekFrom::Start(byte_offset as u64))?;
        state.file.write_all(&buffer[..num_bytes])?;
        state.file.flush()
    }

    fn write_block(&self, buffer: &[u8], num_bytes: usize, starting_block: i32) -> io::Result<i32> {
        if !self.can_write {
            return Err(Self::unsupported_write());
        }

        let mut state = self.state.lock().unwrap();
        let mut next_block_buffer = [0u8; 4];
        let mut current_block = starting_block;
        let mut buffer_index = 0usize;

        if current_block <= 0 {
            current_block = Self::reserve_block_locked(&mut state)?;
        }
        let result_start = current_block;

        while buffer_index < num_bytes {
            let size = ((state.header.block_size - 4) as usize).min(num_bytes - buffer_index);
            state
                .file
                .seek(SeekFrom::Start((current_block + 4) as u64))?;
            state
                .file
                .write_all(&buffer[buffer_index..buffer_index + size])?;
            buffer_index += size;

            let old_offset = current_block;
            if buffer_index < num_bytes {
                state.file.seek(SeekFrom::Start(current_block as u64))?;
                state.file.read_exact(&mut next_block_buffer)?;
                let next_block = i32::from_le_bytes(next_block_buffer);
                current_block = if next_block <= 0 {
                    Self::reserve_block_locked(&mut state)?
                } else {
                    next_block
                };
            } else {
                current_block = 0;
            }

            next_block_buffer.copy_from_slice(&current_block.to_le_bytes());
            state.file.seek(SeekFrom::Start(old_offset as u64))?;
            state.file.write_all(&next_block_buffer)?;
        }

        state.file.flush()?;
        Self::write_header_locked(&mut state)?;
        Ok(result_start)
    }

    fn write_block_async<'a>(
        &'a self,
        buffer: &'a [u8],
        num_bytes: usize,
        starting_block: i32,
    ) -> Pin<Box<dyn Future<Output = io::Result<i32>> + Send + 'a>> {
        Box::pin(async move {
            if !self.can_write {
                return Err(Self::unsupported_write());
            }

            let mut state = self.state.lock().unwrap();
            let mut next_block_buffer = [0u8; 4];
            let mut current_block = starting_block;
            let mut buffer_index = 0usize;

            if current_block <= 0 {
                current_block = Self::reserve_block_locked(&mut state)?;
            }
            let result_start = current_block;

            while buffer_index < num_bytes {
                let size = ((state.header.block_size - 4) as usize).min(num_bytes - buffer_index);
                state.file.seek(SeekFrom::Start((current_block + 4) as u64))?;
                state
                    .file
                    .write_all(&buffer[buffer_index..buffer_index + size])?;
                buffer_index += size;

                let old_offset = current_block;
                if buffer_index < num_bytes {
                    state.file.seek(SeekFrom::Start(current_block as u64))?;
                    state.file.read_exact(&mut next_block_buffer)?;
                    let next_block = i32::from_le_bytes(next_block_buffer);
                    current_block = if next_block <= 0 {
                        Self::reserve_block_locked(&mut state)?
                    } else {
                        next_block
                    };
                } else {
                    current_block = 0;
                }

                next_block_buffer.copy_from_slice(&current_block.to_le_bytes());
                state.file.seek(SeekFrom::Start(old_offset as u64))?;
                state.file.write_all(&next_block_buffer)?;
            }

            state.file.flush()?;
            Self::write_header_locked(&mut state)?;
            Ok(result_start)
        })
    }

    fn read_bytes(
        &self,
        buffer: &mut [u8],
        buffer_offset: usize,
        byte_offset: usize,
        num_bytes: usize,
    ) -> io::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.file.seek(SeekFrom::Start(byte_offset as u64))?;
        state
            .file
            .read_exact(&mut buffer[buffer_offset..buffer_offset + num_bytes])
    }

    fn read_block(&self, buffer: &mut [u8], starting_block: usize) -> io::Result<()> {
        let mut state = self.state.lock().unwrap();
        let mut next_block_buffer = [0u8; 4];
        let mut current_block = starting_block as i32;
        let mut total_read = 0usize;

        while current_block != 0 && total_read < buffer.len() {
            let bytes_to_read =
                ((state.header.block_size - 4) as usize).min(buffer.len() - total_read);
            state
                .file
                .seek(SeekFrom::Start((current_block + 4) as u64))?;
            state
                .file
                .read_exact(&mut buffer[total_read..total_read + bytes_to_read])?;
            total_read += bytes_to_read;
            if total_read >= buffer.len() {
                return Ok(());
            }
            state.file.seek(SeekFrom::Start(current_block as u64))?;
            state.file.read_exact(&mut next_block_buffer)?;
            current_block = i32::from_le_bytes(next_block_buffer);
        }

        Ok(())
    }

    fn read_block_async<'a>(
        &'a self,
        buffer: &'a mut [u8],
        starting_block: usize,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut state = self.state.lock().unwrap();
            let mut next_block_buffer = [0u8; 4];
            let mut current_block = starting_block as i32;
            let mut total_read = 0usize;

            while current_block != 0 && total_read < buffer.len() {
                let bytes_to_read =
                    ((state.header.block_size - 4) as usize).min(buffer.len() - total_read);
                state.file.seek(SeekFrom::Start((current_block + 4) as u64))?;
                state
                    .file
                    .read_exact(&mut buffer[total_read..total_read + bytes_to_read])?;
                total_read += bytes_to_read;
                if total_read >= buffer.len() {
                    return Ok(());
                }
                state.file.seek(SeekFrom::Start(current_block as u64))?;
                state.file.read_exact(&mut next_block_buffer)?;
                current_block = i32::from_le_bytes(next_block_buffer);
            }

            Ok(())
        })
    }

    fn try_get_block_offsets(&self, starting_block: i32) -> io::Result<Option<Vec<i32>>> {
        if starting_block <= 0 {
            return Ok(None);
        }
        let mut state = self.state.lock().unwrap();
        let mut next_block_buffer = [0u8; 4];
        let mut current_block = starting_block;
        let mut file_blocks = Vec::new();
        while current_block != 0 {
            file_blocks.push(current_block);
            state.file.seek(SeekFrom::Start(current_block as u64))?;
            state.file.read_exact(&mut next_block_buffer)?;
            current_block = i32::from_le_bytes(next_block_buffer);
        }
        Ok(Some(file_blocks))
    }

    fn allocate_empty_blocks(&self, num_blocks_to_allocate: i32) -> io::Result<()> {
        if !self.can_write {
            return Err(Self::unsupported_write());
        }
        let mut state = self.state.lock().unwrap();
        Self::allocate_empty_blocks_locked(&mut state, num_blocks_to_allocate)
    }

    fn reserve_block(&self) -> io::Result<i32> {
        if !self.can_write {
            return Err(Self::unsupported_write());
        }
        let mut state = self.state.lock().unwrap();
        Self::reserve_block_locked(&mut state)
    }

    fn set_root_block(&self, offset: i32) -> io::Result<()> {
        if !self.can_write {
            return Err(Self::unsupported_write());
        }
        let mut state = self.state.lock().unwrap();
        state.header.root_block = offset;
        Self::write_header_locked(&mut state)
    }
}
