use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use dat_reader_writer::{
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::BlockAllocators::{
        IDatBlockAllocator::IDatBlockAllocator,
        MemoryMappedBlockAllocator::MemoryMappedBlockAllocator,
        StreamBlockAllocator::StreamBlockAllocator,
    },
    Options::{
        DatAccessType::DatAccessType, DatDatabaseOptions::DatDatabaseOptions,
        FileCachingStrategy::FileCachingStrategy, IndexCachingStrategy::IndexCachingStrategy,
    },
};
use uuid::Uuid;

fn temp_dat_path(name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("dat_reader_writer_{name}_{stamp}.dat"))
}

fn read_write_options(path: &PathBuf) -> DatDatabaseOptions {
    DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        index_caching_strategy: IndexCachingStrategy::OnDemand,
        file_caching_strategy: FileCachingStrategy::OnDemand,
        access_type: DatAccessType::ReadWrite,
    }
}

fn read_only_options(path: &PathBuf) -> DatDatabaseOptions {
    DatDatabaseOptions {
        file_path: path.to_string_lossy().to_string(),
        index_caching_strategy: IndexCachingStrategy::OnDemand,
        file_caching_strategy: FileCachingStrategy::OnDemand,
        access_type: DatAccessType::Read,
    }
}

#[test]
fn stream_allocator_init_new_and_version_updates_header() {
    let path = temp_dat_path("stream_header");
    let allocator = StreamBlockAllocator::new(&read_write_options(&path)).unwrap();

    allocator.init_new(DatFileType::Portal, 7, 64, 4).unwrap();
    allocator
        .set_version("test-version", 1, 2, Uuid::nil(), 3)
        .unwrap();

    let header = allocator.header();
    assert_eq!(DatFileType::Portal, header.r#type);
    assert_eq!(7, header.subset);
    assert_eq!(64, header.block_size);
    assert_eq!(Some("test-version".to_string()), header.version);
    assert!(header.first_free_block >= 448);
    assert_eq!(3, header.minor_version);

    let _ = fs::remove_file(path);
}

#[test]
fn stream_allocator_writes_and_reads_chained_blocks() {
    let path = temp_dat_path("stream_blocks");
    let allocator = StreamBlockAllocator::new(&read_write_options(&path)).unwrap();
    allocator.init_new(DatFileType::Portal, 0, 64, 2).unwrap();

    let payload: Vec<u8> = (0..150).map(|value| (value % 251) as u8).collect();
    let start = allocator.write_block(&payload, payload.len(), 0).unwrap();

    let offsets = allocator.try_get_block_offsets(start).unwrap().unwrap();
    assert!(offsets.len() >= 3);

    let mut read_back = vec![0u8; payload.len()];
    allocator
        .read_block(&mut read_back, start as usize)
        .unwrap();
    assert_eq!(payload, read_back);

    let _ = fs::remove_file(path);
}

#[test]
fn stream_allocator_reuses_existing_start_block_for_rewrite() {
    let path = temp_dat_path("stream_rewrite");
    let allocator = StreamBlockAllocator::new(&read_write_options(&path)).unwrap();
    allocator.init_new(DatFileType::Portal, 0, 64, 3).unwrap();

    let first = vec![1u8; 80];
    let second = vec![2u8; 40];
    let start = allocator.write_block(&first, first.len(), 0).unwrap();
    let rewritten = allocator.write_block(&second, second.len(), start).unwrap();

    assert_eq!(start, rewritten);
    let mut read_back = vec![0u8; second.len()];
    allocator
        .read_block(&mut read_back, start as usize)
        .unwrap();
    assert_eq!(second, read_back);

    let _ = fs::remove_file(path);
}

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use std::{
        pin::pin,
        task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    };

    fn raw_waker() -> RawWaker {
        fn clone(_: *const ()) -> RawWaker {
            raw_waker()
        }
        fn wake(_: *const ()) {}
        fn wake_by_ref(_: *const ()) {}
        fn drop(_: *const ()) {}
        RawWaker::new(
            std::ptr::null(),
            &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
        )
    }

    let waker = unsafe { Waker::from_raw(raw_waker()) };
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut Context::from_waker(&waker)) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[test]
fn stream_allocator_async_wrappers_roundtrip_blocks() {
    let path = temp_dat_path("stream_async");
    let allocator = StreamBlockAllocator::new(&read_write_options(&path)).unwrap();
    allocator.init_new(DatFileType::Portal, 0, 64, 2).unwrap();

    let payload: Vec<u8> = (0..90).map(|value| (value % 251) as u8).collect();
    let start = block_on(allocator.write_block_async(&payload, payload.len(), 0)).unwrap();

    let mut read_back = vec![0u8; payload.len()];
    block_on(allocator.read_block_async(&mut read_back, start as usize)).unwrap();
    assert_eq!(payload, read_back);

    let _ = fs::remove_file(path);
}

#[test]
fn memory_mapped_allocator_reads_existing_blocks_sync_and_async() {
    let path = temp_dat_path("mmap_read");
    let stream_allocator = StreamBlockAllocator::new(&read_write_options(&path)).unwrap();
    stream_allocator
        .init_new(DatFileType::Portal, 0, 64, 2)
        .unwrap();

    let payload: Vec<u8> = (0..90).map(|value| (value % 251) as u8).collect();
    let start = stream_allocator
        .write_block(&payload, payload.len(), 0)
        .unwrap();
    drop(stream_allocator);

    let allocator = MemoryMappedBlockAllocator::new(&read_only_options(&path)).unwrap();
    assert!(!allocator.can_write());
    assert!(allocator.has_header_data());

    let mut sync_read = vec![0u8; payload.len()];
    allocator.read_block(&mut sync_read, start as usize).unwrap();
    assert_eq!(payload, sync_read);

    let mut async_read = vec![0u8; payload.len()];
    block_on(allocator.read_block_async(&mut async_read, start as usize)).unwrap();
    assert_eq!(payload, async_read);

    let _ = fs::remove_file(path);
}
