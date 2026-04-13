use uuid::Uuid;

use crate::{
    Generated::Enums::DatFileType::DatFileType,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct DatHeader {
    pub version: Option<String>,
    pub transactions: [u8; 64],
    pub magic: i32,
    pub block_size: i32,
    pub file_size: i32,
    pub r#type: DatFileType,
    pub subset: u32,
    pub first_free_block: i32,
    pub last_free_block: i32,
    pub free_block_count: i32,
    pub root_block: i32,
    pub new_lru: i32,
    pub old_lru: i32,
    pub use_lru: i32,
    pub master_map_id: i32,
    pub engine_version: i32,
    pub game_version: i32,
    pub major_version: Uuid,
    pub minor_version: u32,
}

impl DatHeader {
    pub const SIZE: usize = 400;
    pub const RETAIL_MAGIC: i32 = 0x0000_5442;

    pub fn new(
        file_type: DatFileType,
        subset: u32,
        block_size: i32,
        version: Option<String>,
        engine_version: i32,
        game_version: i32,
        major_version: Uuid,
        minor_version: u32,
    ) -> Self {
        assert!(
            version.as_ref().is_none_or(|value| value.len() <= 255),
            "Version string can be at max 255 characters."
        );

        let mut header = Self {
            version,
            transactions: [0; 64],
            magic: Self::RETAIL_MAGIC,
            block_size,
            file_size: 0,
            r#type: file_type,
            subset,
            first_free_block: 0,
            last_free_block: 0,
            free_block_count: 0,
            root_block: 0,
            new_lru: 0,
            old_lru: 0,
            use_lru: 0,
            master_map_id: 0,
            engine_version,
            game_version,
            major_version,
            minor_version,
        };
        header.write_empty_transaction();
        header
    }

    pub fn write_empty_transaction(&mut self) {
        self.transactions = [0; 64];
        let mut writer = DatBinWriter::new(&mut self.transactions);
        writer.write_bytes(&[0x00, 0x50, 0x4C, 0x00], 4);
    }

    pub fn get_size(&self) -> usize {
        Self::SIZE
    }
}

impl Default for DatHeader {
    fn default() -> Self {
        let mut header = Self {
            version: None,
            transactions: [0; 64],
            magic: Self::RETAIL_MAGIC,
            block_size: 0,
            file_size: 0,
            r#type: DatFileType::Undefined,
            subset: 0,
            first_free_block: 0,
            last_free_block: 0,
            free_block_count: 0,
            root_block: 0,
            new_lru: 0,
            old_lru: 0,
            use_lru: 0,
            master_map_id: 0,
            engine_version: 0,
            game_version: 0,
            major_version: Uuid::nil(),
            minor_version: 0,
        };
        header.write_empty_transaction();
        header
    }
}

impl IUnpackable for DatHeader {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let version_bytes = reader.read_bytes(256);
        let version = String::from_utf8_lossy(&version_bytes)
            .trim_end_matches('\0')
            .to_string();
        self.version = if version.is_empty() {
            None
        } else {
            Some(version)
        };

        reader.read_bytes_into(&mut self.transactions);
        self.magic = reader.read_i32();
        self.block_size = reader.read_i32();
        self.file_size = reader.read_i32();
        self.r#type = DatFileType::from(reader.read_u32());
        self.subset = reader.read_u32();
        self.first_free_block = reader.read_i32();
        self.last_free_block = reader.read_i32();
        self.free_block_count = reader.read_i32();
        self.root_block = reader.read_i32();
        self.new_lru = reader.read_i32();
        self.old_lru = reader.read_i32();
        self.use_lru = reader.read_i32();
        self.master_map_id = reader.read_i32();
        self.engine_version = reader.read_i32();
        self.game_version = reader.read_i32();
        self.major_version = reader.read_guid();
        self.minor_version = reader.read_u32();

        self.magic == Self::RETAIL_MAGIC
    }
}

impl IPackable for DatHeader {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        let mut version_bytes = [0_u8; 256];
        if let Some(version) = &self.version {
            let encoded = version.as_bytes();
            version_bytes[..encoded.len()].copy_from_slice(encoded);
            version_bytes[encoded.len()] = 0;
        }

        writer.write_bytes(&version_bytes, 256);
        writer.write_bytes(&self.transactions, 64);
        writer.write_i32(self.magic);
        writer.write_i32(self.block_size);
        writer.write_i32(self.file_size);
        writer.write_u32(self.r#type.into());
        writer.write_u32(self.subset);
        writer.write_i32(self.first_free_block);
        writer.write_i32(self.last_free_block);
        writer.write_i32(self.free_block_count);
        writer.write_i32(self.root_block);
        writer.write_i32(self.new_lru);
        writer.write_i32(self.old_lru);
        writer.write_i32(self.use_lru);
        writer.write_i32(self.master_map_id);
        writer.write_i32(self.engine_version);
        writer.write_i32(self.game_version);
        writer.write_guid(self.major_version);
        writer.write_u32(self.minor_version);
        true
    }
}
