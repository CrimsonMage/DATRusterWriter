use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use futures::executor::block_on;

use dat_reader_writer::{
    CellDatabase::CellDatabase,
    DBObjs::{Iteration::Iteration, MasterProperty::MasterProperty, Palette::Palette},
    DatCollection::DatCollection,
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{
        DatBTree::{DatBTreeFile::DatBTreeFile, DatBTreeFileFlags::DatBTreeFileFlags},
        DatBinWriter::DatBinWriter,
        DatHeader::DatHeader,
        IPackable::IPackable,
    },
    LocalDatabase::LocalDatabase,
    Options::{DatAccessType::DatAccessType, DatCollectionOptions::DatCollectionOptions},
    PortalDatabase::PortalDatabase,
    Types::{DBObj::DBObjBase, QualifiedDataId::QualifiedDataId},
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

#[test]
fn portal_database_exposes_master_property_and_region_helpers() {
    let dir = unique_temp_dir();
    let master_path = dir.join("portal_master.dat");
    let master_property = MasterProperty {
        base: DBObjBase {
            id: 0x3900_0001,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut master_bytes = vec![0u8; 1024];
    let master_used = {
        let mut master_writer = DatBinWriter::new(&mut master_bytes);
        assert!(master_property.pack(&mut master_writer));
        master_writer.offset()
    };

    fs::write(
        &master_path,
        build_single_block_dat(
            DatFileType::Portal,
            0x3900_0001,
            &master_bytes[..master_used],
        ),
    )
    .unwrap();
    let master_portal = PortalDatabase::from_path(
        master_path.to_string_lossy().to_string(),
        DatAccessType::Read,
    )
    .unwrap();

    assert!(master_portal.master_property().unwrap().is_some());
}

#[test]
fn dat_collection_can_write_portal_types_and_read_them_back() {
    let dir = unique_temp_dir();

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::ReadWrite)
            .unwrap();

    collection
        .portal
        .inner
        .block_allocator
        .init_new(DatFileType::Portal, 0, 1024, 4)
        .unwrap();

    let palette = Palette {
        base: DBObjBase {
            id: 0x0400_0010,
            ..Default::default()
        },
        colors: vec![dat_reader_writer::Types::ColorARGB::ColorARGB {
            blue: 0x11,
            green: 0x22,
            red: 0x33,
            alpha: 0x44,
        }],
    };

    assert!(collection.try_write_file(&palette).unwrap());

    let read_palette = collection.try_get::<Palette>(0x0400_0010).unwrap().unwrap();
    assert_eq!(1, read_palette.colors.len());
    assert_eq!(0x11, read_palette.colors[0].blue);
    assert_eq!(0x44, read_palette.colors[0].alpha);
}

#[test]
fn dat_collection_rejects_iteration_cross_dat_access() {
    let dir = unique_temp_dir();
    write_header_only_dat(&dir.join("client_portal.dat"), DatFileType::Portal);
    write_header_only_dat(&dir.join("client_cell_1.dat"), DatFileType::Cell);
    write_header_only_dat(&dir.join("client_local_English.dat"), DatFileType::Local);
    write_header_only_dat(&dir.join("client_highres.dat"), DatFileType::Portal);

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::ReadWrite)
            .unwrap();

    assert!(collection.try_get::<Iteration>(0xFFFF0001).is_err());
    assert!(collection.get_cached::<Iteration>(0xFFFF0001).is_err());
    assert!(collection.get_all_ids_of_type::<Iteration>().unwrap().is_empty());
    assert!(collection.try_write_file(&Iteration::default()).is_err());
    assert!(collection.try_write_compressed(&Iteration::default()).is_err());
}

#[test]
fn dat_collection_cached_reads_hold_until_clear_cache() {
    let dir = unique_temp_dir();

    let mut collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::ReadWrite)
            .unwrap();

    collection
        .portal
        .inner
        .block_allocator
        .init_new(DatFileType::Portal, 0, 1024, 4)
        .unwrap();

    let first_palette = Palette {
        base: DBObjBase {
            id: 0x0400_0020,
            ..Default::default()
        },
        colors: vec![dat_reader_writer::Types::ColorARGB::ColorARGB {
            blue: 0x01,
            green: 0x02,
            red: 0x03,
            alpha: 0x04,
        }],
    };

    let second_palette = Palette {
        base: DBObjBase {
            id: 0x0400_0020,
            ..Default::default()
        },
        colors: vec![dat_reader_writer::Types::ColorARGB::ColorARGB {
            blue: 0xAA,
            green: 0xBB,
            red: 0xCC,
            alpha: 0xDD,
        }],
    };

    assert!(collection.try_write_file(&first_palette).unwrap());

    let cached_first = collection.get_cached::<Palette>(0x0400_0020).unwrap().unwrap();
    assert_eq!(0x01, cached_first.colors[0].blue);
    assert_eq!(0x04, cached_first.colors[0].alpha);

    assert!(collection.try_write_file(&second_palette).unwrap());

    let still_cached = collection.get_cached::<Palette>(0x0400_0020).unwrap().unwrap();
    assert_eq!(0x01, still_cached.colors[0].blue);
    assert_eq!(0x04, still_cached.colors[0].alpha);

    collection.clear_cache();

    let refreshed = collection.get_cached::<Palette>(0x0400_0020).unwrap().unwrap();
    assert_eq!(0xAA, refreshed.colors[0].blue);
    assert_eq!(0xDD, refreshed.colors[0].alpha);
}

#[test]
fn dat_collection_async_reads_follow_portal_high_res_and_cache_paths() {
    let dir = unique_temp_dir();
    write_header_only_dat(&dir.join("client_portal.dat"), DatFileType::Portal);
    write_header_only_dat(&dir.join("client_cell_1.dat"), DatFileType::Cell);
    write_header_only_dat(&dir.join("client_local_English.dat"), DatFileType::Local);

    let mut palette_payload = [0u8; 12];
    let mut writer = DatBinWriter::new(&mut palette_payload);
    writer.write_u32(0x04000044);
    writer.write_i32(1);
    writer.write_u32(0x88776655);

    fs::write(
        dir.join("client_highres.dat"),
        build_single_block_dat(DatFileType::Portal, 0x04000044, &palette_payload),
    )
    .unwrap();

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::Read)
            .unwrap();

    let palette = block_on(collection.get_async::<Palette>(0x04000044))
        .unwrap()
        .unwrap();
    assert_eq!(1, palette.colors.len());
    assert_eq!(0x55, palette.colors[0].blue);
    assert_eq!(0x88, palette.colors[0].alpha);

    let cached_palette = block_on(collection.get_cached_async::<Palette>(0x04000044))
        .unwrap()
        .unwrap();
    assert_eq!(0x55, cached_palette.colors[0].blue);
    assert_eq!(0x88, cached_palette.colors[0].alpha);
}

#[test]
fn dat_collection_can_write_with_template_metadata() {
    let dir = unique_temp_dir();

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::ReadWrite)
            .unwrap();

    collection
        .portal
        .inner
        .block_allocator
        .init_new(DatFileType::Portal, 0, 1024, 4)
        .unwrap();

    let palette = Palette {
        base: DBObjBase {
            id: 0x0400_0030,
            ..Default::default()
        },
        colors: vec![dat_reader_writer::Types::ColorARGB::ColorARGB {
            blue: 0x10,
            green: 0x20,
            red: 0x30,
            alpha: 0x40,
        }],
    };

    let template = DatBTreeFile {
        flags: DatBTreeFileFlags::None,
        version: 6,
        iteration: 8,
        ..Default::default()
    };

    assert!(collection
        .try_write_file_with_template(&palette, template)
        .unwrap());

    let entry = collection
        .portal
        .try_get_file_entry(0x0400_0030)
        .unwrap()
        .unwrap();
    assert_eq!(6, entry.version);
    assert_eq!(8, entry.iteration);

    let read_palette = collection.get_cached::<Palette>(0x0400_0030).unwrap().unwrap();
    assert_eq!(0x10, read_palette.colors[0].blue);
    assert_eq!(0x40, read_palette.colors[0].alpha);
}

#[test]
fn dat_collection_async_writes_follow_template_and_read_back() {
    let dir = unique_temp_dir();

    let collection =
        DatCollection::from_directory(dir.to_string_lossy().to_string(), DatAccessType::ReadWrite)
            .unwrap();

    collection
        .portal
        .inner
        .block_allocator
        .init_new(DatFileType::Portal, 0, 1024, 4)
        .unwrap();

    let palette = Palette {
        base: DBObjBase {
            id: 0x0400_0060,
            ..Default::default()
        },
        colors: vec![dat_reader_writer::Types::ColorARGB::ColorARGB {
            blue: 0x21,
            green: 0x43,
            red: 0x65,
            alpha: 0x87,
        }],
    };

    let template = DatBTreeFile {
        flags: DatBTreeFileFlags::None,
        version: 12,
        iteration: 14,
        ..Default::default()
    };

    assert!(block_on(collection.try_write_file_with_template_async(&palette, template)).unwrap());

    let entry = collection
        .portal
        .try_get_file_entry(0x0400_0060)
        .unwrap()
        .unwrap();
    assert_eq!(12, entry.version);
    assert_eq!(14, entry.iteration);

    let read_palette = block_on(collection.get_cached_async::<Palette>(0x0400_0060))
        .unwrap()
        .unwrap();
    assert_eq!(0x21, read_palette.colors[0].blue);
    assert_eq!(0x87, read_palette.colors[0].alpha);
}
