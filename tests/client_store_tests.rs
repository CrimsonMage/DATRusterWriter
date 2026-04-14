use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use dat_ruster_writer::{
    databases::ClientDatStore,
    DBObjs::Palette::Palette,
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{DatBinWriter::DatBinWriter, DatHeader::DatHeader, IPackable::IPackable},
    Options::DatAccessType::DatAccessType,
};
use uuid::Uuid;

fn unique_temp_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("dat_ruster_writer_client_store_{stamp}"));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_header_only_dat(path: &PathBuf, dat_type: DatFileType) {
    let mut header = DatHeader::new(
        dat_type,
        0,
        1024,
        Some("test".to_string()),
        1,
        1,
        Uuid::nil(),
        1,
    );
    header.file_size = DatHeader::SIZE as i32;
    let mut bytes = [0_u8; DatHeader::SIZE];
    assert!(header.pack(&mut DatBinWriter::new(&mut bytes)));
    fs::write(path, bytes).unwrap();
}

fn build_single_block_dat(dat_type: DatFileType, file_id: u32, payload: &[u8]) -> Vec<u8> {
    let block_size = (payload.len() + 256).max(1024);
    let root_offset = block_size;
    let file_offset = root_offset + block_size;

    let mut header = DatHeader::new(
        dat_type,
        0,
        block_size as i32,
        Some("test".to_string()),
        1,
        1,
        Uuid::nil(),
        1,
    );
    header.root_block = root_offset as i32;
    header.file_size = (file_offset + block_size) as i32;

    let mut root_node =
        dat_ruster_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::new(root_offset as i32);
    root_node.file_count = 1;
    root_node.files[0] = dat_ruster_writer::Lib::IO::DatBTree::DatBTreeFile::DatBTreeFile {
        version: 2,
        id: file_id,
        offset: file_offset as i32,
        size: payload.len() as u32,
        iteration: 1,
        ..Default::default()
    };

    let mut bytes = vec![0u8; file_offset + block_size];
    assert!(header.pack(&mut DatBinWriter::new(&mut bytes[..DatHeader::SIZE])));

    let mut node_bytes =
        vec![0u8; dat_ruster_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::SIZE];
    assert!(root_node.pack(&mut DatBinWriter::new(&mut node_bytes)));
    bytes[root_offset + 4..root_offset + 4 + node_bytes.len()].copy_from_slice(&node_bytes);
    bytes[file_offset + 4..file_offset + 4 + payload.len()].copy_from_slice(payload);
    bytes
}

#[test]
fn client_dat_store_loads_typed_assets_through_one_api() {
    let dir = unique_temp_dir();
    write_header_only_dat(&dir.join("client_portal.dat"), DatFileType::Portal);
    write_header_only_dat(&dir.join("client_cell_1.dat"), DatFileType::Cell);
    write_header_only_dat(&dir.join("client_local_English.dat"), DatFileType::Local);

    let mut palette_payload = [0u8; 12];
    let mut writer = DatBinWriter::new(&mut palette_payload);
    writer.write_u32(0x0400_0044);
    writer.write_i32(1);
    writer.write_u32(0x88776655);

    fs::write(
        dir.join("client_highres.dat"),
        build_single_block_dat(DatFileType::Portal, 0x0400_0044, &palette_payload),
    )
    .unwrap();

    let store = ClientDatStore::open(dir.to_string_lossy().to_string(), DatAccessType::Read)
        .unwrap();

    let palette = store.load::<Palette>(0x0400_0044).unwrap().unwrap();
    assert_eq!(1, palette.colors.len());
    assert_eq!(0x55, palette.colors[0].blue);

    let cached = store.load_cached::<Palette>(0x0400_0044).unwrap().unwrap();
    assert_eq!(0x88, cached.colors[0].alpha);

    let ids = store.load_ids::<Palette>().unwrap();
    assert_eq!(vec![0x0400_0044], ids);
}

#[test]
fn client_dat_store_convenience_methods_use_the_same_underlying_collection() {
    let dir = unique_temp_dir();
    write_header_only_dat(&dir.join("client_cell_1.dat"), DatFileType::Cell);
    write_header_only_dat(&dir.join("client_local_English.dat"), DatFileType::Local);

    let mut palette_payload = [0u8; 12];
    let mut writer = DatBinWriter::new(&mut palette_payload);
    writer.write_u32(0x0400_0077);
    writer.write_i32(1);
    writer.write_u32(0x44332211);

    fs::write(
        dir.join("client_portal.dat"),
        build_single_block_dat(DatFileType::Portal, 0x0400_0077, &palette_payload),
    )
    .unwrap();
    write_header_only_dat(&dir.join("client_highres.dat"), DatFileType::Portal);

    let store = ClientDatStore::open(dir.to_string_lossy().to_string(), DatAccessType::Read)
        .unwrap();

    let palette = store.palette(0x0400_0077).unwrap().unwrap();
    assert_eq!(1, palette.colors.len());
    assert_eq!(0x11, palette.colors[0].blue);

    assert!(store.master_property().unwrap().is_none());
    assert!(store.region().unwrap().is_none());
}
