use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use dat_reader_writer::{
    CellDatabase::CellDatabase,
    DBObjs::Palette::Palette,
    DatCollection::DatCollection,
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{DatBinWriter::DatBinWriter, DatHeader::DatHeader, IPackable::IPackable},
    LocalDatabase::LocalDatabase,
    Options::{DatAccessType::DatAccessType, DatCollectionOptions::DatCollectionOptions},
    PortalDatabase::PortalDatabase,
    Types::QualifiedDataId::QualifiedDataId,
};
use uuid::Uuid;

fn unique_temp_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("dat_reader_writer_tests_{stamp}"));
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
    let block_size = 1024usize;
    let root_offset = 1024usize;
    let file_offset = 2048usize;

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
        dat_reader_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::new(root_offset as i32);
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

    let mut node_bytes =
        vec![0u8; dat_reader_writer::Lib::IO::DatBTree::DatBTreeNode::DatBTreeNode::SIZE];
    assert!(root_node.pack(&mut DatBinWriter::new(&mut node_bytes)));
    bytes[root_offset + 4..root_offset + 4 + node_bytes.len()].copy_from_slice(&node_bytes);
    bytes[file_offset + 4..file_offset + 4 + payload.len()].copy_from_slice(payload);
    bytes
}

#[test]
fn specialized_databases_validate_header_type() {
    let dir = unique_temp_dir();
    let portal_path = dir.join("client_portal.dat");
    let cell_path = dir.join("client_cell_1.dat");
    let local_path = dir.join("client_local_English.dat");

    write_header_only_dat(&portal_path, DatFileType::Portal);
    write_header_only_dat(&cell_path, DatFileType::Cell);
    write_header_only_dat(&local_path, DatFileType::Local);

    assert!(
        PortalDatabase::from_path(
            portal_path.to_string_lossy().to_string(),
            DatAccessType::Read
        )
        .is_ok()
    );
    assert!(
        CellDatabase::from_path(cell_path.to_string_lossy().to_string(), DatAccessType::Read)
            .is_ok()
    );
    assert!(
        LocalDatabase::from_path(
            local_path.to_string_lossy().to_string(),
            DatAccessType::Read
        )
        .is_ok()
    );

    assert!(
        CellDatabase::from_path(
            portal_path.to_string_lossy().to_string(),
            DatAccessType::Read
        )
        .is_err()
    );
}

#[test]
fn dat_collection_opens_all_expected_dats() {
    let dir = unique_temp_dir();
    write_header_only_dat(&dir.join("client_portal.dat"), DatFileType::Portal);
    write_header_only_dat(&dir.join("client_cell_1.dat"), DatFileType::Cell);
    write_header_only_dat(&dir.join("client_local_English.dat"), DatFileType::Local);
    write_header_only_dat(&dir.join("client_highres.dat"), DatFileType::Portal);

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::Read)
            .unwrap();
    assert_eq!(DatFileType::Cell, collection.cell.inner.header().r#type);
    assert_eq!(DatFileType::Portal, collection.portal.inner.header().r#type);
    assert_eq!(DatFileType::Local, collection.local.inner.header().r#type);
    assert_eq!(
        DatFileType::Portal,
        collection.high_res.inner.header().r#type
    );
}

#[test]
fn dat_collection_uses_path_overrides() {
    let dir = unique_temp_dir();
    let portal_path = dir.join("portal_override.dat");
    let cell_path = dir.join("cell_override.dat");
    let local_path = dir.join("local_override.dat");
    let high_res_path = dir.join("highres_override.dat");

    write_header_only_dat(&portal_path, DatFileType::Portal);
    write_header_only_dat(&cell_path, DatFileType::Cell);
    write_header_only_dat(&local_path, DatFileType::Local);
    write_header_only_dat(&high_res_path, DatFileType::Portal);

    let mut options = DatCollectionOptions::default();
    options.set_portal_dat_path(portal_path.to_string_lossy().to_string());
    options.set_cell_dat_path(cell_path.to_string_lossy().to_string());
    options.set_local_dat_path(local_path.to_string_lossy().to_string());
    options.set_high_res_dat_path(high_res_path.to_string_lossy().to_string());

    let collection = DatCollection::new(options).unwrap();
    assert_eq!(DatFileType::Portal, collection.portal.inner.header().r#type);
    assert_eq!(DatFileType::Cell, collection.cell.inner.header().r#type);
    assert_eq!(DatFileType::Local, collection.local.inner.header().r#type);
    assert_eq!(
        DatFileType::Portal,
        collection.high_res.inner.header().r#type
    );
}

#[test]
fn dat_collection_can_read_typed_asset_from_high_res_fallback() {
    let dir = unique_temp_dir();
    write_header_only_dat(&dir.join("client_portal.dat"), DatFileType::Portal);
    write_header_only_dat(&dir.join("client_cell_1.dat"), DatFileType::Cell);
    write_header_only_dat(&dir.join("client_local_English.dat"), DatFileType::Local);

    let mut palette_payload = [0u8; 12];
    let mut writer = DatBinWriter::new(&mut palette_payload);
    writer.write_u32(0x04000010);
    writer.write_i32(1);
    writer.write_u32(0x04030201);

    fs::write(
        dir.join("client_highres.dat"),
        build_single_block_dat(DatFileType::Portal, 0x04000010, &palette_payload),
    )
    .unwrap();

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::Read)
            .unwrap();
    let palette = collection.try_get::<Palette>(0x04000010).unwrap().unwrap();
    assert_eq!(1, palette.colors.len());
    assert_eq!(4, palette.colors[0].alpha);
}

#[test]
fn dat_collection_can_enumerate_ids_across_portal_and_high_res() {
    let dir = unique_temp_dir();
    write_header_only_dat(&dir.join("client_cell_1.dat"), DatFileType::Cell);
    write_header_only_dat(&dir.join("client_local_English.dat"), DatFileType::Local);

    let mut portal_payload = [0u8; 8];
    let mut portal_writer = DatBinWriter::new(&mut portal_payload);
    portal_writer.write_u32(0x04000001);
    portal_writer.write_i32(0);

    let mut high_res_payload = [0u8; 8];
    let mut high_res_writer = DatBinWriter::new(&mut high_res_payload);
    high_res_writer.write_u32(0x04000002);
    high_res_writer.write_i32(0);

    fs::write(
        dir.join("client_portal.dat"),
        build_single_block_dat(DatFileType::Portal, 0x04000001, &portal_payload),
    )
    .unwrap();
    fs::write(
        dir.join("client_highres.dat"),
        build_single_block_dat(DatFileType::Portal, 0x04000002, &high_res_payload),
    )
    .unwrap();

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::Read)
            .unwrap();
    let ids = collection.get_all_ids_of_type::<Palette>().unwrap();
    assert_eq!(vec![0x04000001, 0x04000002], ids);
}

#[test]
fn qualified_data_id_can_resolve_through_collection() {
    let dir = unique_temp_dir();
    write_header_only_dat(&dir.join("client_portal.dat"), DatFileType::Portal);
    write_header_only_dat(&dir.join("client_cell_1.dat"), DatFileType::Cell);
    write_header_only_dat(&dir.join("client_local_English.dat"), DatFileType::Local);

    let mut palette_payload = [0u8; 12];
    let mut writer = DatBinWriter::new(&mut palette_payload);
    writer.write_u32(0x04000022);
    writer.write_i32(1);
    writer.write_u32(0xDDCCBBAA);

    fs::write(
        dir.join("client_highres.dat"),
        build_single_block_dat(DatFileType::Portal, 0x04000022, &palette_payload),
    )
    .unwrap();

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::Read)
            .unwrap();
    let qualified = QualifiedDataId::<Palette>::new(0x04000022);
    let palette = qualified.get(&collection).unwrap().unwrap();
    assert_eq!(1, palette.colors.len());
    assert_eq!(0xAA, palette.colors[0].blue);
    assert_eq!(0xDD, palette.colors[0].alpha);
}
