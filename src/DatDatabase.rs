use std::{collections::BTreeMap, io, io::Read, sync::Arc};

use flate2::read::ZlibDecoder;

use crate::{
    DBObjs::MasterProperty::MasterProperty,
    Generated::Enums::DBObjType::DBObjType,
    Generated::Enums::{BasePropertyType::BasePropertyType, DatFileType::DatFileType},
    Lib::{
        DBObjAttributeCache,
        IO::{
            BlockAllocators::{
                IDatBlockAllocator::IDatBlockAllocator,
                MemoryMappedBlockAllocator::MemoryMappedBlockAllocator,
                StreamBlockAllocator::StreamBlockAllocator,
            },
            DatBTree::{
                DatBTreeFile::DatBTreeFile, DatBTreeFileFlags::DatBTreeFileFlags,
                DatBTreeReaderWriter::DatBTreeReaderWriter,
            },
            DatBinReader::DatBinReader,
            DatHeader::DatHeader,
            IDBObj::IDBObj,
        },
    },
    Options::{
        DatAccessType::DatAccessType, DatDatabaseOptions::DatDatabaseOptions,
        IndexCachingStrategy::IndexCachingStrategy,
    },
};

pub struct DatDatabase {
    pub block_allocator: Arc<dyn IDatBlockAllocator>,
    pub tree: DatBTreeReaderWriter,
    pub options: DatDatabaseOptions,
}

impl DatDatabase {
    pub fn new(options: DatDatabaseOptions) -> io::Result<Self> {
        let block_allocator: Arc<dyn IDatBlockAllocator> = match options.access_type {
            DatAccessType::Read => MemoryMappedBlockAllocator::new(&options)?,
            DatAccessType::ReadWrite => StreamBlockAllocator::new(&options)?,
        };
        let tree = DatBTreeReaderWriter::new(block_allocator.clone());
        if options.index_caching_strategy == IndexCachingStrategy::Upfront {
            tree.build_flat_index()?;
        }

        Ok(Self {
            block_allocator,
            tree,
            options,
        })
    }

    pub fn clear_cache(&mut self) {
        self.tree.clear_cache();
        if self.options.index_caching_strategy == IndexCachingStrategy::Upfront {
            let _ = self.tree.build_flat_index();
        }
    }

    pub fn header(&self) -> DatHeader {
        self.block_allocator.header()
    }

    pub fn type_from_id(&self, id: u32) -> DBObjType {
        DBObjAttributeCache::db_obj_type_from_id(self.header().r#type, id)
    }

    pub fn try_get_file_entry(&self, file_id: u32) -> io::Result<Option<DatBTreeFile>> {
        self.tree.try_get_file(file_id)
    }

    pub fn try_get_file_bytes(
        &self,
        file_id: u32,
        auto_decompress: bool,
    ) -> io::Result<Option<Vec<u8>>> {
        let Some(file_entry) = self.tree.try_get_file(file_id)? else {
            return Ok(None);
        };

        let mut bytes = vec![0_u8; file_entry.size as usize];
        self.block_allocator
            .read_block(&mut bytes, file_entry.offset as usize)?;

        if auto_decompress && file_entry.flags.contains(DatBTreeFileFlags::IsCompressed) {
            return Ok(Some(Self::decompress(&bytes)?));
        }

        Ok(Some(bytes))
    }

    pub fn try_get<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default,
    {
        let base_property_types =
            if self.header().r#type == DatFileType::Portal && file_id != 0x39000001 {
                self.base_property_types()?
            } else {
                None
            };
        self.try_get_with_base_property_types(file_id, base_property_types)
    }

    pub fn try_get_with_base_property_types<T>(
        &self,
        file_id: u32,
        base_property_types: Option<Arc<BTreeMap<u32, BasePropertyType>>>,
    ) -> io::Result<Option<T>>
    where
        T: IDBObj + Default,
    {
        let Some(bytes) = self.try_get_file_bytes(file_id, true)? else {
            return Ok(None);
        };
        let mut value = T::default();
        if !value.unpack(&mut DatBinReader::with_base_property_types(
            &bytes,
            base_property_types,
        )) {
            return Ok(None);
        }
        value.set_id(file_id);
        Ok(Some(value))
    }

    pub fn get<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default,
    {
        self.try_get::<T>(file_id)
    }

    pub fn base_property_types(&self) -> io::Result<Option<Arc<BTreeMap<u32, BasePropertyType>>>> {
        if self.header().r#type != DatFileType::Portal {
            return Ok(None);
        }

        let Some(master_property) =
            self.try_get_with_base_property_types::<MasterProperty>(0x39000001, None)?
        else {
            return Ok(None);
        };

        Ok(Some(Arc::new(
            master_property
                .properties
                .into_iter()
                .map(|(key, property)| (key, property.property_type))
                .collect(),
        )))
    }

    pub fn has_file(&self, file_id: u32) -> io::Result<bool> {
        self.tree.has_file(file_id)
    }

    pub fn get_all_ids_of_type<T>(&self) -> io::Result<Vec<u32>>
    where
        T: IDBObj,
    {
        let attr = T::db_obj_type_attr();
        if attr.is_singular() {
            return Ok(vec![attr.first_id]);
        }

        if !attr.has_range_data() {
            return Ok(self
                .tree
                .all_files()?
                .into_iter()
                .filter(|file| {
                    DBObjAttributeCache::type_from_id(self.header().r#type, file.id)
                        .map(|candidate| candidate.db_obj_type == attr.db_obj_type)
                        .unwrap_or(false)
                })
                .map(|file| file.id)
                .collect());
        }

        Ok(self
            .tree
            .get_files_in_range(attr.first_id, attr.last_id)?
            .into_iter()
            .map(|file| file.id)
            .collect())
    }

    fn decompress(data: &[u8]) -> io::Result<Vec<u8>> {
        if data.len() < 4 {
            return Ok(data.to_vec());
        }

        let uncompressed_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let mut decoder = ZlibDecoder::new(&data[4..]);
        let mut output = Vec::with_capacity(uncompressed_size);
        decoder.read_to_end(&mut output)?;
        Ok(output)
    }
}
