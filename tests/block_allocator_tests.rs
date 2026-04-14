use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

use dat_reader_writer::{
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::BlockAllocators::{
        IDatBlockAllocator::IDatBlockAllocator,
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
    allocator.read_block(&mut read_back, start as usize).unwrap();
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
    allocator.read_block(&mut read_back, start as usize).unwrap();
    assert_eq!(second, read_back);

    let _ = fs::remove_file(path);
}
