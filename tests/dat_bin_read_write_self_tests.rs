use dat_ruster_writer::{
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{
        DatBinReader::DatBinReader,
        DatBinWriter::DatBinWriter,
        DatHeader::DatHeader,
        IPackable::IPackable,
        IUnpackable::IUnpackable,
        Numerics::{Plane, Quaternion, Vector3},
    },
};
use uuid::Uuid;

#[test]
fn can_write_read_multiple_values() {
    let mut bytes = [0_u8; 12];
    let mut writer = DatBinWriter::new(&mut bytes);
    let random_bytes = [0x12_u8, 0x34, 0x56, 0x78];

    writer.write_u32(1);
    writer.write_i32(-1);
    writer.write_bytes(&random_bytes, 4);

    let mut reader = DatBinReader::new(&bytes);
    assert_eq!(1_u32, reader.read_u32());
    assert_eq!(-1, reader.read_i32());
    assert_eq!(random_bytes, reader.read_bytes(4).as_slice());
}

#[test]
fn can_skip_and_write_read() {
    let mut bytes = [0_u8; 12];
    let mut writer = DatBinWriter::new(&mut bytes);
    writer.skip(4);
    writer.write_u32(1);
    writer.write_i32(-1);

    let mut reader = DatBinReader::new(&bytes);
    reader.skip(4);
    assert_eq!(1_u32, reader.read_u32());
    assert_eq!(-1, reader.read_i32());
}

#[test]
fn can_read_write_compressed_uint_values() {
    for value in [1234_u32, 5678_u32, 0_u32, 1_u32] {
        let mut bytes = [0_u8; 4];
        let mut writer = DatBinWriter::new(&mut bytes);
        writer.write_compressed_uint(value);

        let mut reader = DatBinReader::new(&bytes);
        assert_eq!(value, reader.read_compressed_uint());
    }
}

#[test]
fn can_write_read_vector3_quaternion_and_plane() {
    let vector = Vector3::new(1.0, 2.0, 3.0);
    let quat = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let plane = Plane::new(vector, 4.0);

    let mut vector_bytes = [0_u8; 12];
    DatBinWriter::new(&mut vector_bytes).write_vector3(vector);
    assert_eq!(vector, DatBinReader::new(&vector_bytes).read_vector3());

    let mut quat_bytes = [0_u8; 16];
    DatBinWriter::new(&mut quat_bytes).write_quaternion(quat);
    assert_eq!(quat, DatBinReader::new(&quat_bytes).read_quaternion());

    let mut plane_bytes = [0_u8; 16];
    DatBinWriter::new(&mut plane_bytes).write_plane(plane);
    assert_eq!(plane, DatBinReader::new(&plane_bytes).read_plane());
}

#[test]
fn can_pack_unpack_dat_header() {
    let major_version = Uuid::parse_str("00112233-4455-6677-8899-aabbccddeeff").unwrap();
    let mut header = DatHeader::new(
        DatFileType::Portal,
        7,
        1024,
        Some("retail".to_string()),
        1,
        2,
        major_version,
        3,
    );
    header.file_size = 4096;
    header.first_free_block = 128;
    header.last_free_block = 512;
    header.free_block_count = 4;
    header.root_block = 1024;

    let mut bytes = [0_u8; DatHeader::SIZE];
    let packed = header.pack(&mut DatBinWriter::new(&mut bytes));
    assert!(packed);

    let mut unpacked = DatHeader::default();
    let unpacked_ok = unpacked.unpack(&mut DatBinReader::new(&bytes));
    assert!(unpacked_ok);
    assert_eq!(header, unpacked);
}
