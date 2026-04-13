use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

use dat_reader_writer::{
    DBObjs::{Iteration::Iteration, Palette::Palette},
    DatDatabase::DatDatabase,
    Generated::Enums::{DBObjType::DBObjType, DatFileType::DatFileType},
    Lib::{
        DBObjAttributeCache,
        IO::{DatBinWriter::DatBinWriter, DatHeader::DatHeader, IPackable::IPackable},
    },
    Options::DatDatabaseOptions::DatDatabaseOptions,
};
use uuid::Uuid;

fn unique_temp_file() -> PathBuf {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::temp_dir();
    dir.join(format!("dat_reader_writer_typed_{stamp}.dat"))
}

fn build_single_block_dat(file_id: u32, payload: &[u8]) -> Vec<u8> {
    let block_size = 1024usize;
    let root_offset = 1024usize;
    let file_offset = 2048usize;

    let mut header = DatHeader::new(DatFileType::Portal, 0, block_size as i32, Some("test".to_string()), 1, 1, Uuid::nil(), 1);
    header.root_block = root_offset as i32;
    header.file_size = (file_offset + block_size) as i32;

    let mut root_node = dat_reader_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::new(root_offset as i32);
    root_node.file_count = 1;
    root_node.files[0] = dat_reader_writer::Lib::IO::DatBTree::DatBTreeFile::DatBTreeFile {
        version: 2,
        id: file_id,
        offset: file_offset as i32,
        size: payload.len() as u32,
        iteration: 1,
        ..Default::default()
    };

    let mut bytes = vec![0u8; file_offset + block_size];
    assert!(header.pack(&mut DatBinWriter::new(&mut bytes[..DatHeader::SIZE])));

    let mut node_bytes = vec![0u8; dat_reader_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::SIZE];
    assert!(root_node.pack(&mut DatBinWriter::new(&mut node_bytes)));
    bytes[root_offset + 4..root_offset + 4 + node_bytes.len()].copy_from_slice(&node_bytes);
    bytes[file_offset + 4..file_offset + 4 + payload.len()].copy_from_slice(payload);
    bytes
}

#[test]
fn db_obj_attribute_cache_resolves_iteration() {
    let attr = DBObjAttributeCache::type_from_id(DatFileType::Portal, 0xFFFF0001).unwrap();
    assert_eq!(DBObjType::Iteration, attr.db_obj_type);
}

#[test]
fn dat_database_can_read_typed_iteration() {
    let mut payload = [0u8; 12];
    let mut writer = DatBinWriter::new(&mut payload);
    writer.write_i32(1);
    writer.write_i32(-1);
    writer.write_i32(1);

    let bytes = build_single_block_dat(0xFFFF0001, &payload);
    let path = unique_temp_file();
    fs::write(&path, bytes).unwrap();

    let db = DatDatabase::new(DatDatabaseOptions { file_path: path.to_string_lossy().to_string(), ..DatDatabaseOptions::default() }).unwrap();
    assert_eq!(DBObjType::Iteration, db.type_from_id(0xFFFF0001));

    let iteration = db.try_get::<Iteration>(0xFFFF0001).unwrap().unwrap();
    assert_eq!(1, iteration.current_iteration);
    assert_eq!(Some(&-1), iteration.iterations.get(&1));
}

#[test]
fn dat_database_can_enumerate_ids_of_type() {
    let mut payload = [0u8; 8];
    let mut writer = DatBinWriter::new(&mut payload);
    writer.write_u32(0x00000000);
    writer.write_i32(0);

    let bytes = build_single_block_dat(0x04000010, &payload);
    let path = unique_temp_file();
    fs::write(&path, bytes).unwrap();

    let db = DatDatabase::new(DatDatabaseOptions { file_path: path.to_string_lossy().to_string(), ..DatDatabaseOptions::default() }).unwrap();
    let ids = db.get_all_ids_of_type::<Palette>().unwrap();
    assert_eq!(vec![0x04000010], ids);
}
