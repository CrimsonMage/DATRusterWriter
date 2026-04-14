use std::{
    any::Any,
    collections::BTreeMap,
    future::Future,
    io,
    io::{Read, Write},
    pin::Pin,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use flate2::read::ZlibDecoder;
use flate2::{write::ZlibEncoder, Compression};

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
            DatBinWriter::DatBinWriter,
            DatHeader::DatHeader,
            IDBObj::IDBObj,
            IPackable::IPackable,
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
    file_cache: Mutex<BTreeMap<u32, Box<dyn Any + Send>>>,
    base_property_types_cache: Mutex<Option<Option<Arc<BTreeMap<u32, BasePropertyType>>>>>,
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
            file_cache: Mutex::new(BTreeMap::new()),
            base_property_types_cache: Mutex::new(None),
        })
    }

    pub fn clear_cache(&mut self) {
        self.file_cache.lock().unwrap().clear();
        *self.base_property_types_cache.lock().unwrap() = None;
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

    pub fn try_get_file_bytes_async<'a>(
        &'a self,
        file_id: u32,
        auto_decompress: bool,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<Vec<u8>>>> + Send + 'a>> {
        Box::pin(async move { self.try_get_file_bytes(file_id, auto_decompress) })
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
        let mut reader = DatBinReader::with_base_property_types(
            &bytes,
            base_property_types,
        );
        if !value.unpack(&mut reader) || reader.failed() {
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

    pub fn try_get_async<'a, T>(
        &'a self,
        file_id: u32,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<T>>> + Send + 'a>>
    where
        T: IDBObj + Default + Send + 'a,
    {
        Box::pin(async move { self.try_get::<T>(file_id) })
    }

    pub fn get_async<'a, T>(
        &'a self,
        file_id: u32,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<T>>> + Send + 'a>>
    where
        T: IDBObj + Default + Send + 'a,
    {
        Box::pin(async move { self.get::<T>(file_id) })
    }

    pub fn get_cached<T>(&self, file_id: u32) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        let base_property_types =
            if self.header().r#type == DatFileType::Portal && file_id != 0x39000001 {
                self.base_property_types()?
            } else {
                None
            };
        self.get_cached_with_base_property_types(file_id, base_property_types)
    }

    pub fn get_cached_with_base_property_types<T>(
        &self,
        file_id: u32,
        base_property_types: Option<Arc<BTreeMap<u32, BasePropertyType>>>,
    ) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        if let Some(cached) = self
            .file_cache
            .lock()
            .unwrap()
            .get(&file_id)
            .and_then(|value| value.downcast_ref::<T>())
        {
            return Ok(Some(cached.clone()));
        }

        let value: Option<T> =
            self.try_get_with_base_property_types(file_id, base_property_types)?;
        if let Some(value) = &value {
            self.file_cache
                .lock()
                .unwrap()
                .insert(file_id, Box::new(value.clone()));
        }
        Ok(value)
    }

    pub fn get_cached_async<'a, T>(
        &'a self,
        file_id: u32,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<T>>> + Send + 'a>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        Box::pin(async move { self.get_cached::<T>(file_id) })
    }

    pub fn base_property_types(&self) -> io::Result<Option<Arc<BTreeMap<u32, BasePropertyType>>>> {
        if self.header().r#type != DatFileType::Portal {
            return Ok(None);
        }

        if let Some(cached) = self.base_property_types_cache.lock().unwrap().clone() {
            return Ok(cached);
        }

        let Some(master_property) =
            self.try_get_with_base_property_types::<MasterProperty>(0x39000001, None)?
        else {
            *self.base_property_types_cache.lock().unwrap() = Some(None);
            return Ok(None);
        };

        let cached = Some(Arc::new(
            master_property
                .properties
                .into_iter()
                .map(|(key, property)| (key, property.property_type))
                .collect(),
        ));
        *self.base_property_types_cache.lock().unwrap() = Some(cached.clone());
        Ok(cached)
    }

    pub fn has_file(&self, file_id: u32) -> io::Result<bool> {
        self.tree.has_file(file_id)
    }

    pub fn try_write_file<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        let mut buffer = vec![0u8; 1024 * 1024 * 5];
        let bytes_to_write = {
            let mut writer = DatBinWriter::new(&mut buffer);
            writer.write_item(value);
            writer.offset()
        };

        self.try_write_bytes_core(value.id(), &buffer, bytes_to_write, false, |entry| {
            if entry.iteration == 0 {
                entry.iteration = 1;
            }
        })
    }

    pub async fn try_write_file_async<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        self.try_write_file(value)
    }

    pub fn try_write_file_with_template<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        let mut buffer = vec![0u8; 1024 * 1024 * 5];
        let bytes_to_write = {
            let mut writer = DatBinWriter::new(&mut buffer);
            writer.write_item(value);
            writer.offset()
        };

        self.try_write_bytes_core(value.id(), &buffer, bytes_to_write, false, |entry| {
            entry.flags =
                (template.flags & !DatBTreeFileFlags::IsCompressed)
                    | (entry.flags & DatBTreeFileFlags::IsCompressed);
            entry.version = template.version;
            entry.iteration = template.iteration;
        })
    }

    pub async fn try_write_file_with_template_async<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        self.try_write_file_with_template(value, template)
    }

    pub fn try_write_compressed<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        let mut buffer = vec![0u8; 1024 * 1024 * 5];
        let bytes_to_write = {
            let mut writer = DatBinWriter::new(&mut buffer);
            writer.write_item(value);
            writer.offset()
        };

        self.try_write_bytes_core(value.id(), &buffer, bytes_to_write, true, |entry| {
            if entry.iteration == 0 {
                entry.iteration = 1;
            }
        })
    }

    pub async fn try_write_compressed_async<T>(&self, value: &T) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        self.try_write_compressed(value)
    }

    pub fn try_write_compressed_with_template<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable,
    {
        let mut buffer = vec![0u8; 1024 * 1024 * 5];
        let bytes_to_write = {
            let mut writer = DatBinWriter::new(&mut buffer);
            writer.write_item(value);
            writer.offset()
        };

        self.try_write_bytes_core(value.id(), &buffer, bytes_to_write, true, |entry| {
            entry.flags =
                (template.flags & !DatBTreeFileFlags::IsCompressed)
                    | (entry.flags & DatBTreeFileFlags::IsCompressed);
            entry.version = template.version;
            entry.iteration = template.iteration;
        })
    }

    pub async fn try_write_compressed_with_template_async<T>(
        &self,
        value: &T,
        template: DatBTreeFile,
    ) -> io::Result<bool>
    where
        T: IDBObj + IPackable + Sync,
    {
        self.try_write_compressed_with_template(value, template)
    }

    pub fn try_write_file_bytes(
        &self,
        id: u32,
        buffer: &[u8],
        bytes_to_write: usize,
        iteration: i32,
    ) -> io::Result<bool> {
        self.try_write_bytes_core(id, buffer, bytes_to_write, false, |entry| {
            entry.iteration = iteration;
        })
    }

    pub async fn try_write_file_bytes_async(
        &self,
        id: u32,
        buffer: &[u8],
        bytes_to_write: usize,
        iteration: i32,
    ) -> io::Result<bool> {
        self.try_write_file_bytes(id, buffer, bytes_to_write, iteration)
    }

    pub fn try_write_compressed_bytes(
        &self,
        id: u32,
        buffer: &[u8],
        bytes_to_write: usize,
        iteration: i32,
    ) -> io::Result<bool> {
        self.try_write_bytes_core(id, buffer, bytes_to_write, true, |entry| {
            entry.iteration = iteration;
        })
    }

    pub async fn try_write_compressed_bytes_async(
        &self,
        id: u32,
        buffer: &[u8],
        bytes_to_write: usize,
        iteration: i32,
    ) -> io::Result<bool> {
        self.try_write_compressed_bytes(id, buffer, bytes_to_write, iteration)
    }

    pub fn try_write_file_bytes_with_template(
        &self,
        id: u32,
        buffer: &[u8],
        bytes_to_write: usize,
        template: DatBTreeFile,
    ) -> io::Result<bool> {
        self.try_write_bytes_core(id, buffer, bytes_to_write, false, |entry| {
            entry.flags =
                (template.flags & !DatBTreeFileFlags::IsCompressed)
                    | (entry.flags & DatBTreeFileFlags::IsCompressed);
            entry.version = template.version;
            entry.iteration = template.iteration;
        })
    }

    pub async fn try_write_file_bytes_with_template_async(
        &self,
        id: u32,
        buffer: &[u8],
        bytes_to_write: usize,
        template: DatBTreeFile,
    ) -> io::Result<bool> {
        self.try_write_file_bytes_with_template(id, buffer, bytes_to_write, template)
    }

    pub fn try_write_compressed_bytes_with_template(
        &self,
        id: u32,
        buffer: &[u8],
        bytes_to_write: usize,
        template: DatBTreeFile,
    ) -> io::Result<bool> {
        self.try_write_bytes_core(id, buffer, bytes_to_write, true, |entry| {
            entry.flags =
                (template.flags & !DatBTreeFileFlags::IsCompressed)
                    | (entry.flags & DatBTreeFileFlags::IsCompressed);
            entry.version = template.version;
            entry.iteration = template.iteration;
        })
    }

    pub async fn try_write_compressed_bytes_with_template_async(
        &self,
        id: u32,
        buffer: &[u8],
        bytes_to_write: usize,
        template: DatBTreeFile,
    ) -> io::Result<bool> {
        self.try_write_compressed_bytes_with_template(id, buffer, bytes_to_write, template)
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

    fn try_write_bytes_core<F>(
        &self,
        id: u32,
        buffer: &[u8],
        bytes_to_write: usize,
        compress: bool,
        configure_entry: F,
    ) -> io::Result<bool>
    where
        F: FnOnce(&mut DatBTreeFile),
    {
        if !self.block_allocator.can_write() {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "block allocator was opened as read only",
            ));
        }

        let existing_file = self.tree.try_get_file(id)?;
        let mut starting_block = existing_file.map(|file| file.offset).unwrap_or_default();
        let mut flags = existing_file.map(|file| file.flags).unwrap_or_default();
        let version = existing_file.map(|file| file.version).unwrap_or(2);
        let existing_iteration = existing_file.map(|file| file.iteration).unwrap_or_default();

        let write_data = if compress {
            if let Some(compressed) = Self::attempt_to_compress(&buffer[..bytes_to_write])? {
                flags |= DatBTreeFileFlags::IsCompressed;
                compressed
            } else {
                flags.remove(DatBTreeFileFlags::IsCompressed);
                buffer[..bytes_to_write].to_vec()
            }
        } else {
            flags.remove(DatBTreeFileFlags::IsCompressed);
            buffer[..bytes_to_write].to_vec()
        };

        starting_block =
            self.block_allocator
                .write_block(&write_data, write_data.len(), starting_block)?;

        let raw_date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as u32;

        let mut entry = DatBTreeFile {
            flags,
            version,
            id,
            offset: starting_block,
            size: write_data.len() as u32,
            raw_date,
            iteration: existing_iteration,
        };
        configure_entry(&mut entry);
        let _ = self.tree.insert(entry)?;
        *self.base_property_types_cache.lock().unwrap() = None;
        Ok(true)
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

    fn attempt_to_compress(data: &[u8]) -> io::Result<Option<Vec<u8>>> {
        if data.len() < 16 {
            return Ok(None);
        }

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(data)?;
        let compressed = encoder.finish()?;

        if compressed.len() + 4 >= data.len() {
            return Ok(None);
        }

        let mut output = Vec::with_capacity(compressed.len() + 4);
        output.extend_from_slice(&(data.len() as u32).to_le_bytes());
        output.extend_from_slice(&compressed);
        Ok(Some(output))
    }
}
