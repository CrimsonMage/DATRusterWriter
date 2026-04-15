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
    decoded_file_cache: Mutex<DecodedFileCache>,
    base_property_types_cache: Mutex<Option<Option<Arc<BTreeMap<u32, BasePropertyType>>>>>,
}

struct CachedDecodedValue {
    value: Arc<dyn Any + Send + Sync>,
    approx_bytes: usize,
    last_access: u64,
}

#[derive(Default)]
struct DecodedFileCache {
    entries: BTreeMap<u32, CachedDecodedValue>,
    current_bytes: usize,
    access_clock: u64,
}

impl DecodedFileCache {
    fn next_access(&mut self) -> u64 {
        self.access_clock = self.access_clock.saturating_add(1);
        self.access_clock
    }

    fn insert(
        &mut self,
        file_id: u32,
        value: Arc<dyn Any + Send + Sync>,
        approx_bytes: usize,
        budget_bytes: usize,
    ) {
        if let Some(previous) = self.entries.remove(&file_id) {
            self.current_bytes = self.current_bytes.saturating_sub(previous.approx_bytes);
        }

        let last_access = self.next_access();
        self.current_bytes = self.current_bytes.saturating_add(approx_bytes);
        self.entries.insert(
            file_id,
            CachedDecodedValue {
                value,
                approx_bytes,
                last_access,
            },
        );
        self.evict_to_budget(budget_bytes);
    }

    fn get(&mut self, file_id: u32) -> Option<Arc<dyn Any + Send + Sync>> {
        let last_access = self.next_access();
        let entry = self.entries.get_mut(&file_id)?;
        entry.last_access = last_access;
        Some(entry.value.clone())
    }

    fn evict_to_budget(&mut self, budget_bytes: usize) {
        while self.current_bytes > budget_bytes {
            let Some((&evict_id, _)) = self.entries.iter().min_by_key(|(_, entry)| entry.last_access)
            else {
                break;
            };
            if let Some(evicted) = self.entries.remove(&evict_id) {
                self.current_bytes = self.current_bytes.saturating_sub(evicted.approx_bytes);
            }
        }
    }
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
            decoded_file_cache: Mutex::new(DecodedFileCache::default()),
            base_property_types_cache: Mutex::new(None),
        })
    }

    pub fn clear_cache(&mut self) {
        *self.decoded_file_cache.lock().unwrap() = DecodedFileCache::default();
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
        Ok(self
            .decode_with_base_property_types(file_id, base_property_types)?
            .map(|(value, _)| value))
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
        Ok(self
            .get_cached_shared::<T>(file_id)?
            .map(|value: Arc<T>| (*value).clone()))
    }

    pub fn get_cached_with_base_property_types<T>(
        &self,
        file_id: u32,
        base_property_types: Option<Arc<BTreeMap<u32, BasePropertyType>>>,
    ) -> io::Result<Option<T>>
    where
        T: IDBObj + Default + Clone + Send + 'static,
    {
        Ok(self
            .get_cached_shared_with_base_property_types(file_id, base_property_types)?
            .map(|value: Arc<T>| (*value).clone()))
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

    pub(crate) fn get_cached_shared<T>(&self, file_id: u32) -> io::Result<Option<Arc<T>>>
    where
        T: IDBObj + Default + Send + Sync + 'static,
    {
        let base_property_types =
            if self.header().r#type == DatFileType::Portal && file_id != 0x39000001 {
                self.base_property_types()?
            } else {
                None
            };
        self.get_cached_shared_with_base_property_types(file_id, base_property_types)
    }

    pub(crate) fn get_cached_shared_with_base_property_types<T>(
        &self,
        file_id: u32,
        base_property_types: Option<Arc<BTreeMap<u32, BasePropertyType>>>,
    ) -> io::Result<Option<Arc<T>>>
    where
        T: IDBObj + Default + Send + Sync + 'static,
    {
        let cache_budget = self.decoded_object_cache_budget_bytes();
        if cache_budget == 0 {
            return Ok(self
                .decode_with_base_property_types(file_id, base_property_types)?
                .map(|(value, _)| Arc::new(value)));
        }

        if let Some(cached) = self.try_get_cached_arc::<T>(file_id) {
            return Ok(Some(cached));
        }

        let Some((value, approx_bytes)) =
            self.decode_with_base_property_types(file_id, base_property_types)?
        else {
            return Ok(None);
        };

        let value: Arc<T> = Arc::new(value);
        if approx_bytes <= cache_budget {
            self.decoded_file_cache.lock().unwrap().insert(
                file_id,
                value.clone(),
                approx_bytes,
                cache_budget,
            );
        }
        Ok(Some(value))
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
        *self.decoded_file_cache.lock().unwrap() = DecodedFileCache::default();
        *self.base_property_types_cache.lock().unwrap() = None;
        Ok(true)
    }

    fn decode_with_base_property_types<T>(
        &self,
        file_id: u32,
        base_property_types: Option<Arc<BTreeMap<u32, BasePropertyType>>>,
    ) -> io::Result<Option<(T, usize)>>
    where
        T: IDBObj + Default,
    {
        let Some(bytes) = self.try_get_file_bytes(file_id, true)? else {
            return Ok(None);
        };
        let approx_bytes = bytes.len();
        let mut value = T::default();
        let mut reader = DatBinReader::with_base_property_types(&bytes, base_property_types);
        if !value.unpack(&mut reader) || reader.failed() {
            return Ok(None);
        }
        value.set_id(file_id);
        Ok(Some((value, approx_bytes)))
    }

    fn decoded_object_cache_budget_bytes(&self) -> usize {
        match self.options.file_caching_strategy {
            crate::Options::FileCachingStrategy::FileCachingStrategy::Never => 0,
            crate::Options::FileCachingStrategy::FileCachingStrategy::OnDemand => {
                self.options.typed_object_cache_budget_bytes
            }
        }
    }

    fn try_get_cached_arc<T>(&self, file_id: u32) -> Option<Arc<T>>
    where
        T: IDBObj + Default + Send + Sync + 'static,
    {
        let cached = self.decoded_file_cache.lock().unwrap().get(file_id)?;
        Arc::downcast::<T>(cached).ok()
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

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::{
        DBObjs::Palette::Palette,
        Generated::Enums::DatFileType::DatFileType,
        Options::{
            DatAccessType::DatAccessType, DatDatabaseOptions::DatDatabaseOptions,
            FileCachingStrategy::FileCachingStrategy,
        },
        Types::{ColorARGB::ColorARGB, DBObj::DBObjBase},
    };

    use super::DatDatabase;

    fn unique_temp_file() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("dat_ruster_writer_cache_{stamp}.dat"))
    }

    fn palette(id: u32, colors: usize, seed: u8) -> Palette {
        Palette {
            base: DBObjBase {
                id,
                ..Default::default()
            },
            colors: (0..colors)
                .map(|index| ColorARGB {
                    blue: seed.wrapping_add(index as u8),
                    green: seed.wrapping_add(1 + index as u8),
                    red: seed.wrapping_add(2 + index as u8),
                    alpha: seed.wrapping_add(3 + index as u8),
                })
                .collect(),
        }
    }

    fn new_portal_database(
        file_caching_strategy: FileCachingStrategy,
        typed_object_cache_budget_bytes: usize,
    ) -> (DatDatabase, PathBuf) {
        let path = unique_temp_file();
        let database = DatDatabase::new(DatDatabaseOptions {
            file_path: path.to_string_lossy().to_string(),
            access_type: DatAccessType::ReadWrite,
            file_caching_strategy,
            typed_object_cache_budget_bytes,
            ..DatDatabaseOptions::default()
        })
        .unwrap();

        database
            .block_allocator
            .init_new(DatFileType::Portal, 0, 1024, 4)
            .unwrap();

        (database, path)
    }

    #[test]
    fn decoded_object_cache_is_disabled_when_strategy_is_never() {
        let (database, path) = new_portal_database(FileCachingStrategy::Never, 1024);
        let value = palette(0x0400_0100, 2, 1);
        assert!(database.try_write_file(&value).unwrap());

        let _ = database.get_cached::<Palette>(value.base.id).unwrap().unwrap();
        let cache = database.decoded_file_cache.lock().unwrap();
        assert!(cache.entries.is_empty());
        assert_eq!(0, cache.current_bytes);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn decoded_object_cache_reuses_shared_values_without_growing() {
        let (database, path) = new_portal_database(FileCachingStrategy::OnDemand, 1024);
        let value = palette(0x0400_0110, 4, 5);
        assert!(database.try_write_file(&value).unwrap());

        let first = database
            .get_cached_shared::<Palette>(value.base.id)
            .unwrap()
            .unwrap();
        let initial_bytes = database.decoded_file_cache.lock().unwrap().current_bytes;
        let second = database
            .get_cached_shared::<Palette>(value.base.id)
            .unwrap()
            .unwrap();
        let final_bytes = database.decoded_file_cache.lock().unwrap().current_bytes;

        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(initial_bytes, final_bytes);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn decoded_object_cache_evicts_to_stay_within_byte_budget() {
        let first = palette(0x0400_0120, 4, 10);
        let second = palette(0x0400_0121, 4, 20);
        let expected_entry_size = 8 + (4 * 4);
        let (database, path) =
            new_portal_database(FileCachingStrategy::OnDemand, expected_entry_size);
        assert!(database.try_write_file(&first).unwrap());
        assert!(database.try_write_file(&second).unwrap());

        let _ = database
            .get_cached_shared::<Palette>(first.base.id)
            .unwrap()
            .unwrap();
        let _ = database
            .get_cached_shared::<Palette>(second.base.id)
            .unwrap()
            .unwrap();

        let cache = database.decoded_file_cache.lock().unwrap();
        assert_eq!(1, cache.entries.len());
        assert!(cache.current_bytes <= expected_entry_size);
        assert!(cache.entries.contains_key(&second.base.id));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn malformed_reads_do_not_populate_decoded_object_cache() {
        let (database, path) = new_portal_database(FileCachingStrategy::OnDemand, 1024);
        assert!(database
            .try_write_file_bytes(0x0400_0130, &[0xAA, 0xBB], 2, 1)
            .unwrap());

        let result = database.get_cached::<Palette>(0x0400_0130).unwrap();
        assert!(result.is_none());
        let cache = database.decoded_file_cache.lock().unwrap();
        assert!(cache.entries.is_empty());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn clear_cache_empties_decoded_and_metadata_caches() {
        let (mut database, path) = new_portal_database(FileCachingStrategy::OnDemand, 1024);
        let value = palette(0x0400_0140, 2, 30);
        assert!(database.try_write_file(&value).unwrap());

        let _ = database
            .get_cached_shared::<Palette>(value.base.id)
            .unwrap()
            .unwrap();
        let _ = database.base_property_types().unwrap();
        assert!(!database.decoded_file_cache.lock().unwrap().entries.is_empty());

        database.clear_cache();
        assert!(database.decoded_file_cache.lock().unwrap().entries.is_empty());
        assert!(database.base_property_types_cache.lock().unwrap().is_none());

        let _ = fs::remove_file(path);
    }
}
